extern crate libc;
extern crate pnet;
extern crate chrono;

use libc::timeval;
use self::pnet::packet::Packet;
use self::pnet::packet::ipv4::Ipv4Packet;
use self::chrono::{DateTime, TimeZone, Duration, NaiveDateTime, Utc};
use netdefs::layer2::ethernet::MacAddress;

use std::fmt;
use std::slice::Iter;
use std::sync::Mutex;
use std::net::Ipv4Addr;
use layer2::nf_frame::nf_frame;
use layer2::nf_cache::nf_cache;

lazy_static! {
    pub static ref EVENT_LOG: Mutex<EventLog> = Mutex::new(EventLog::new());
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum EventType {
    Multiple_MACs_for_IP,
    Multiple_IPs_for_MAC,
    SYN_Stealth_Scan_Detected,
    Full_TCP_Scan_Detected,
}

pub trait Event: Send {
    fn get_type(&self) -> EventType;
    fn get_timestamp(&self) -> &DateTime<Utc>;
    fn get_description(&self) -> &str;
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} ", self.get_timestamp().format("%Y/%m/%d %a %H:%M:%S Utc"));
        write!(f,"{}", self.get_description())
    }
}

pub struct SpoofEvent {
    event_type: EventType,
    timestamp: DateTime<Utc>,
    description: String,
    ips: Option<Vec<Ipv4Addr>>,
    macs: Option<Vec<MacAddress>>,
}

impl Event for SpoofEvent {
    fn get_type(&self) -> EventType {
        self.event_type
    }

    fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    fn get_description(&self) -> &str {
        &self.description
    }
}

impl SpoofEvent {
    pub fn new(etype: EventType, time: libc::timeval, ips: Option<Vec<Ipv4Addr>>, macs: Option<Vec<MacAddress>>) -> SpoofEvent {
        let dt: DateTime<Utc> = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(time.tv_sec, (time.tv_usec*1000) as u32), Utc);
        let mut desc = String::new();
        if etype == EventType::Multiple_MACs_for_IP {
            desc = format!("Multiple MAC addresses registered for IP. MACs: {}, IP: {}", macs.clone().unwrap().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "), ips.clone().unwrap().get(0 as usize).unwrap().to_string());
        } else if etype == EventType::Multiple_IPs_for_MAC {
            desc = format!("Multiple IP addresses registered for MAC. IPs: {}, MAC: {}", ips.clone().unwrap().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "), macs.clone().unwrap().get(0 as usize).unwrap().to_string());
        }

        SpoofEvent { event_type: etype, timestamp: dt, description: desc, ips: ips, macs: macs }
    }
}

pub struct EventLog {
    events: Vec<Box<Event>>,
    callbacks: Vec<fn(Box<Event>)>,
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for event in self.events.iter() {
            writeln!(f,"{}", event);
        }
        write!(f,"")
    }
}

impl EventLog {
    pub fn new() -> EventLog {
        EventLog { events: Vec::new(), callbacks: Vec::new() }
    }

    pub fn register_callback(&mut self, f: fn(Box<Event>)) {
        self.callbacks.push(f);
    }

    pub fn push_event(&mut self, event: Box<Event>) {
        self.events.push(event);
    }

    pub fn notify_callback(&self) {
       //TODO  
    }

    pub fn get_latest_event(&self) -> &Box<Event> {
        self.events.last().unwrap()
    }

    pub fn to_iter(&self) -> Iter<Box<Event>> {
        (&self.events).iter()
    }

    pub fn pop_event(&mut self) -> Box<Event> {
        self.events.pop().unwrap()
    }

    pub fn pull_event(&mut self) -> Box<Event> {
        self.events.remove(0)
    }

    pub fn print_log(&self) {
        println!("{}", self)
    }
}

trait SliceExt<T> {
    type Item;

    fn get<I>(&self, index: I) -> &I::Output
        where I: SliceIndex<Self::Item>;
}

impl<T> SliceExt<T> for [T] {
    type Item = T;

    fn get<I>(&self, index: I) -> &I::Output
        where I: SliceIndex<T>
    {
        panic!()
    }
}

pub trait SliceIndex<T> {
    type Output: ?Sized;
}

impl<T> SliceIndex<T> for usize {
    type Output = T;
}
