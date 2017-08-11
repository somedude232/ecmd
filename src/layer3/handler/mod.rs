extern crate nfqueue;
extern crate libc;
extern crate netdefs;
extern crate pnet;

mod spoofdetect;

use std::str::FromStr;
use std::net::Ipv4Addr;

use self::pnet::packet::PrimitiveValues;
use self::pnet::packet::ipv4::Ipv4Packet;
use self::pnet::packet::ip::{IpNextHeaderProtocol,IpNextHeaderProtocols};

use layer4::handler::*;
use layer3::handler::spoofdetect::*;
use layer2::nf_frame::*;
use layer2::nf_cache::*;
use security::eventlog::*;

use netdefs::layer2::ethernet::{MacAddress,IpMacCombo,parse_ethertype};
use netdefs::layer3::ipv4::parse_protocol_field;
use netdefs::layer4::TransportProtocol;

pub fn l3_process_ipv4(frame: nf_frame) {
    let payload = &(&frame).get_payload().clone();
    let l3_packet = Ipv4Packet::new(payload).unwrap();
    let l4_proto = parse_protocol_field(l3_packet.get_next_level_protocol().to_primitive_values().0).unwrap();

    let src_ip = l3_packet.get_source();
    let src_mac = *((&frame).get_mac_addr());
    let src_ip_mac_combo = IpMacCombo::new(src_ip,src_mac);

    if !ip_mac_table_contains(src_ip_mac_combo) {
	insert_ip_mac_entry(src_ip, src_mac);
	let has_mult_ips = mac_has_multiple_ips(src_mac);
	let has_mult_macs = ip_has_multiple_macs(src_ip);
        if has_mult_macs {
            let ref mut event_lock = EVENT_LOG.lock().unwrap();
            let ref mut ip_mac_lock = IP_MAC_TABLE.lock().unwrap();
            event_lock.push_event(Box::new(SpoofEvent::new(EventType::Multiple_MACs_for_IP, *(&frame).get_timestamp(), Some(get_ip_mac_entry_by_ip(ip_mac_lock,src_ip).iter().map(|x| x.get_ip().unwrap()).collect::<Vec<Ipv4Addr>>()), Some(get_ip_mac_entry_by_ip(ip_mac_lock,src_ip).iter().map(|x| x.get_mac().unwrap()).collect::<Vec<MacAddress>>()))));
            event_lock.print_log();
        }
	if has_mult_ips {
            let ref mut event_lock = EVENT_LOG.lock().unwrap();
            let ref mut ip_mac_lock = IP_MAC_TABLE.lock().unwrap();
            event_lock.push_event(Box::new(SpoofEvent::new(EventType::Multiple_IPs_for_MAC, *(&frame).get_timestamp(), Some(get_ip_mac_entry_by_mac(ip_mac_lock,src_mac).iter().map(|x| x.get_ip().unwrap()).collect::<Vec<Ipv4Addr>>()), Some(get_ip_mac_entry_by_mac(ip_mac_lock,src_mac).iter().map(|x| x.get_mac().unwrap()).collect::<Vec<MacAddress>>()))));
            event_lock.print_log();
        }
    }
    
    l3_transfer_to_l4(frame, l4_proto);
}

pub fn l3_process_ipv6(frame: nf_frame) {
    
}

pub fn l3_process_arp(frame: nf_frame) {

}

pub fn l3_process_wake_on_lan(frame: nf_frame) {
    
}

pub fn l3_process_reverse_arp(frame: nf_frame) {

}

pub fn l3_process_appletalk(frame: nf_frame) {
    
}

pub fn l3_process_appletalk_arp(frame: nf_frame) {

}

pub fn l3_process_vlan_tagged(frame: nf_frame) {
    
}

pub fn l3_process_ipx(frame: nf_frame) {

}

pub fn l3_process_hyper_scsi(frame: nf_frame) {
    
}

pub fn l3_process_ataoe(frame: nf_frame) {

}

pub fn l3_process_lldp(frame: nf_frame) {
    
}

pub fn l3_process_mac_sec(frame: nf_frame) {

}

pub fn l3_process_fcoe(frame: nf_frame) {
    
}

pub fn l3_process_double_vlan_tagged(frame: nf_frame) {

}

pub fn l3_transfer_to_l4(frame: nf_frame, l4_proto: TransportProtocol) {
    match l4_proto {
        TransportProtocol::ICMP => l4_process_icmp(frame),
        TransportProtocol::TCP => l4_process_tcp(frame),
        TransportProtocol::UDP => l4_process_udp(frame),
        _ => (),
    }
}
