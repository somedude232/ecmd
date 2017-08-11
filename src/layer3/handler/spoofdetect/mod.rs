extern crate lazy_static;

use std::sync::{Mutex,MutexGuard};
use std::net::Ipv4Addr;

use netdefs::layer2::ethernet::{IpMacCombo, IpMacSet, IPMACAssociateError, MacAddress};

lazy_static! {
    pub static ref IP_MAC_TABLE: Mutex<IpMacSet> = Mutex::new(IpMacSet::new());
}


pub fn insert_ip_mac_entry(ip: Ipv4Addr, mac: MacAddress) {
    let combo = IpMacCombo::new(ip,mac);
    let mut lock = IP_MAC_TABLE.lock().unwrap();
    lock.push(combo);
}

pub fn insert_ip_entry(ip: Ipv4Addr) {
    let mut lock = IP_MAC_TABLE.lock().unwrap();
    lock.push_ip(ip);
}

pub fn insert_mac_entry(mac: MacAddress) {
    let mut lock = IP_MAC_TABLE.lock().unwrap();
    lock.push_mac(mac);
}

pub fn ip_mac_entry_has_multiples(ip: Ipv4Addr, mac: MacAddress) -> bool {
    let mut lock = IP_MAC_TABLE.lock().unwrap();
    lock.has_multiples_of(ip,mac)
}

pub fn ip_has_multiple_macs(ip: Ipv4Addr) -> bool {
    let mut lock = IP_MAC_TABLE.lock().unwrap();
    lock.ip_has_multiple_macs(ip)
}

pub fn mac_has_multiple_ips(mac: MacAddress) -> bool {
    let mut lock = IP_MAC_TABLE.lock().unwrap();
    lock.mac_has_multiple_ips(mac)
}

pub fn get_ip_mac_entry_by_ip<'a>(lock: &'a mut MutexGuard<IpMacSet>, ip: Ipv4Addr) -> Vec<&'a IpMacCombo> {
    lock.get_by_ip(ip)
}

pub fn get_ip_mac_entry_by_mac<'a>(lock: &'a mut MutexGuard<IpMacSet>, mac: MacAddress) -> Vec<&'a IpMacCombo> {
    lock.get_by_mac(mac)
}

pub fn ip_mac_table_contains(entry: IpMacCombo) -> bool {
    let mut lock = IP_MAC_TABLE.lock().unwrap();
    lock.contains(&entry)
}

pub fn ip_mac_table_contains_ip(ip: Ipv4Addr) -> bool {
    let mut lock = IP_MAC_TABLE.lock().unwrap();
    lock.contains_ip(ip)
}

pub fn ip_mac_table_contains_mac(mac: MacAddress) -> bool {
    let mut lock = IP_MAC_TABLE.lock().unwrap();
    lock.contains_mac(mac)
}

pub fn associate_ip_mac_entries(lock: &mut MutexGuard<IpMacSet>, ip: Ipv4Addr, mac: MacAddress) -> Result<IpMacCombo,IPMACAssociateError> {
    lock.associate_ip_mac(ip,mac)
}

pub fn remove_ip_entries(ip: Ipv4Addr) {
    let mut lock = IP_MAC_TABLE.lock().unwrap();
    lock.remove_by_ip(ip);
}

pub fn remove_mac_entries(mac: MacAddress) {
    let mut lock = IP_MAC_TABLE.lock().unwrap();
    lock.remove_by_mac(mac);
}
