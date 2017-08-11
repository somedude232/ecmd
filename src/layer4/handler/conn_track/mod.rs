extern crate libc;
extern crate chrono;
extern crate netdefs;
extern crate pnet;

use std::collections::HashMap;
use std::net::Ipv4Addr;

use layer2::nf_frame::nf_frame;

use self::netdefs::layer2::ethernet::{MacAddress, IpMacCombo};
use self::netdefs::layer4::tcp::{TCPPort,TCPFlag};
use self::pnet::packet::tcp::TcpPacket;
use self::chrono::{DateTime, TimeZone, Duration, NaiveDateTime, Utc};

pub enum TCPConnectionState {
    CLOSED,
    LISTEN,
    SYN_RCVD,
    ESTD,
    CLOSING,
}

pub struct TCPMessage {
    src_ip: Ipv4Addr,
    src_mac: MacAddress,
    dst_ip: Ipv4Addr,
    src_port: TCPPort,
    dst_port: TCPPort,
    flags: Vec<TCPFlag>,
    timestamp: DateTime<Utc>,
}

pub struct TCPBareMessage {
    flags: Vec<TCPFlag>,
    timestamp: DateTime<Utc>,
}

pub struct TCPBareConnection {
    messages: Vec<TCPBareMessage>,
    state: TCPConnectionState,
}

pub struct TCPConnection {
    src_ip: Ipv4Addr,
    src_mac: MacAddress,
    dst_ip: Ipv4Addr,
    src_port: TCPPort,
    dst_port: TCPPort,
    messages: Vec<TCPMessage>,
    most_recent: DateTime<Utc>,
}

pub struct TCPConnectionHistory {
    src_ip: Ipv4Addr,
    src_mac: MacAddress,
    dst_ip: Ipv4Addr,
    connections: Vec<TCPBareConnection>,
}

pub struct TCPBareConnectionHistory {
    connections: HashMap<(TCPPort,TCPPort),Vec<TCPBareConnection>>,
}

pub struct IncomingTCPTrafficHistory {
    traffic: HashMap<IpMacCombo,TCPBareConnectionHistory>,
}

impl IncomingTCPTrafficHistory {
    pub fn new() -> IncomingTCPTrafficHistory {
        IncomingTCPTrafficHistory { traffic: HashMap::new() }
    }

    pub fn record_frame_from_ip(ip: Ipv4Addr, frame: TcpPacket) {
        let matching_entries = traffic.keys().filter(|x| x.get_ip() == ip).collect::<Vec<Ipv4Addr>>();
        if matching_entries.len() > 0 {
            
        } else if matching_entries.len() > 1 {
            
        } else {
            let key = IpMacCombo::from_ip(ip);
            traffic.insert(key,TCPBareConnectionHistory::new_from_frame(frame));
        }
    }

    pub fn get_traffic_from_ip() -> Vec<TCPConnection> {
        
    }
}

impl TCPBareConnectionHistory {
    
}

impl TCPBareConnection {

}

impl TCPBareMessage {
    pub fn from_frame(frame: nf_frame) -> TCPBareMessage {
        
    }
}
