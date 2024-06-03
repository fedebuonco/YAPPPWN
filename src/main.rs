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
        exploit_target_mac: MacAddress64::from_u64(0),
        pppoe_softc: 0,
        cap_device: cap,
    };

    exploit.ppp_negotiation();
}
