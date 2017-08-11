extern crate libc;
extern crate nfqueue;
extern crate netdefs;
#[macro_use]
extern crate lazy_static;

use netdefs::layer3::NetworkProtocol;
use netdefs::layer2::ethernet::parse_ethertype;

mod layer2;
mod layer3;
mod layer4;
mod security; // Apache no sue pls

use layer2::nf_frame::*;
use layer2::nf_cache::nf_cache;
use layer3::handler::*;

#[allow(unused_variables)]
fn init_processing(msg: &nfqueue::Message, state: &mut ()) {
    let frame = nf_frame::from_msg(msg);
    let ethertype = *frame.get_ethertype();

    match ethertype {
        NetworkProtocol::IPv4 => l3_process_ipv4(frame),
        NetworkProtocol::IPv6 => l3_process_ipv6(frame),
        NetworkProtocol::ARP  => l3_process_arp(frame),
        _ => (),
    }

    msg.set_verdict(nfqueue::Verdict::Drop);
}

fn main() {
    let mut q = nfqueue::Queue::new(());

    q.open();

    let rc = q.bind(libc::AF_INET);
    assert!(rc == 0);

    q.create_queue(3, init_processing);
    q.set_mode(nfqueue::CopyMode::CopyPacket, 0xffff);

    q.run_loop();

    q.close();
}
