extern crate libc;
extern crate nfqueue;

use std::clone::Clone;
use netdefs::layer2::ethernet::{MacAddress,parse_ethertype};
use netdefs::layer3::NetworkProtocol;

// This is a simplified frame designed to adapt from messages from nfqueue-rs
pub struct nf_frame {
    id: u32,
    ethertype: NetworkProtocol,
    mac_addr: MacAddress,
    timestamp: libc::timeval,
    payload: Vec<u8>,
}

impl<'a> nf_frame {
    pub fn from_msg(msg: &nfqueue::Message) -> nf_frame {
        nf_frame { id: msg.get_id(), ethertype: parse_ethertype(msg.get_l3_proto()).unwrap(), mac_addr: MacAddress::from_str(&msg.get_packet_hw().unwrap().to_string()), timestamp: msg.get_timestamp().unwrap(), payload: msg.get_payload().to_vec() }
    }

    pub fn get_id(&'a self) -> u32 {
        self.id
    }

    pub fn get_ethertype(&'a self) -> &'a NetworkProtocol {
        &self.ethertype
    }

    pub fn get_mac_addr(&'a self) -> &'a MacAddress {
        &self.mac_addr
    }

    pub fn get_timestamp(&'a self) -> &'a libc::timeval {
        &self.timestamp
    }

    pub fn get_timestamp_float(&self) -> f32 {
        self.timestamp.tv_sec as f32 + (self.timestamp.tv_usec as f32 / 1000000.0)
    }

    pub fn get_payload(&'a self) -> &'a Vec<u8> {
        &self.payload
    }
}

impl Clone for nf_frame {
   fn clone(&self) -> nf_frame {
       let mut new_vec: Vec<u8> = Vec::new();
       for byte in self.payload.iter() {
           new_vec.push(*byte);
       }
       nf_frame { id: self.id, ethertype: self.ethertype, mac_addr: self.mac_addr, timestamp: self.timestamp, payload: new_vec}
   }
}

