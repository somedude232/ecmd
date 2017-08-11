extern crate pnet;

use self::pnet::packet::Packet;
use self::pnet::packet::ipv4::Ipv4Packet;
use self::pnet::packet::tcp::TcpPacket;

use std::sync::Mutex;
use std::sync::MutexGuard;
use std::collections::HashMap;

use layer2::nf_frame::nf_frame;
use layer2::nf_cache::nf_cache;

use netdefs::layer4::tcp::{TCPFlag, parse_tcp_flags};

type scan_func = fn(nf_frame) -> bool;

const NUM_SCANS: u8 = 2;

lazy_static! {
    static ref FUNC_CACHE_MAP: Mutex<HashMap<u32, nf_cache>> = Mutex::new(HashMap::new());
}

fn get_cache_using_ptr_key<'a>(lock: &'a mut MutexGuard<HashMap<u32, nf_cache>>, func: &scan_func) -> &'a mut nf_cache {
    let ptr_int = &(*func as u32);

    if !lock.contains_key(ptr_int) {
        lock.insert(*ptr_int, nf_cache::new());
    }

    lock.get_mut(ptr_int).unwrap()
}

pub fn all_tcp_scan(frame: nf_frame) -> bool {

    // At the time of writing this, Rust doesn't support getting function pointers from a module,
    // so you'll have to add any custom scan checking routines below.

    half_tcp_scan_detect(frame.clone()) || full_tcp_scan_detect(frame.clone())
}

pub fn half_tcp_scan_detect(frame: nf_frame) -> bool {
    let mut lock = FUNC_CACHE_MAP.lock().unwrap();
    let func_ptr = &(half_tcp_scan_detect as scan_func);
    let ref mut packet_cache = get_cache_using_ptr_key(&mut lock,func_ptr);

    let mac_addr = &frame.get_mac_addr();
    let ipv4_packet = Ipv4Packet::new(&frame.get_payload()).unwrap();
    let tcp_packet = TcpPacket::new(ipv4_packet.payload()).unwrap();

    if packet_cache.len() < 1 {
        packet_cache.push(frame.clone());
        return false;
    }

    let flags = parse_tcp_flags(tcp_packet.get_flags());

    if flags == [TCPFlag::SYN] {
        
    }

    false
}

pub fn full_tcp_scan_detect(frame: nf_frame) -> bool {
    false
}
