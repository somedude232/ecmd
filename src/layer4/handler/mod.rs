extern crate nfqueue;
extern crate libc;
extern crate netdefs;
extern crate pnet;

use self::pnet::packet::Packet;
use self::pnet::packet::tcp::TcpPacket;
use self::pnet::packet::udp::UdpPacket;
use self::pnet::packet::icmp::IcmpPacket;
use self::pnet::packet::ipv4::Ipv4Packet;

mod scandetect;
mod conn_track;

use netdefs::layer2::ethernet::*;

use layer2::nf_frame::*;
use layer2::nf_cache::nf_cache;

use self::scandetect::all_tcp_scan;

pub fn l4_process_hopopt (frame: nf_frame) {

}

pub fn l4_process_icmp (frame: nf_frame) {
    let ipv4_packet = Ipv4Packet::new(frame.get_payload()).unwrap();
    let icmp_packet = IcmpPacket::new(ipv4_packet.payload()).unwrap();
    
}

pub fn l4_process_ip_in_ip (frame: nf_frame) {

}

pub fn l4_process_tcp (frame: nf_frame) {
    let result = all_tcp_scan(frame);

    
}

pub fn l4_process_egp (frame: nf_frame) {

}

pub fn l4_process_igp (frame: nf_frame) {

}

pub fn l4_process_udp (frame: nf_frame) {

}

pub fn l4_process_ipv6 (frame: nf_frame) {

}

pub fn l4_process_ipv6_route (frame: nf_frame) {

}

pub fn l4_process_ipv6_frag (frame: nf_frame) {

}

pub fn l4_process_ipv6_icmp (frame: nf_frame) {

}

pub fn l4_process_ipv6_nonxt (frame: nf_frame) {

}

pub fn l4_process_ipv6_opts (frame: nf_frame) {

}

pub fn l4_process_sctp (frame: nf_frame) {

}

pub fn l4_process_fc (frame: nf_frame) {

}
