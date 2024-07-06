mod constants;
mod exploit;
use pcap::{Active, Capture, Device};
use pnet::datalink::{self, NetworkInterface};

use crate::exploit::Exploit;

fn run_exploit() {
    // Find interface
    let interface_name = "enp4s0";
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap();
    // Exploit
    let mut exploit = Exploit {
        target_mac: [0, 0, 0, 0, 0, 0],
        pppoe_softc: 0,
        source_mac: [0, 0, 0, 0, 0, 0],
        host_uniq: [0, 0, 0, 0, 0, 0, 0, 0],
    };
    // PPP negotiation
    exploit.ppp_negotiation(interface);
}

fn main() {
    run_exploit()
}
