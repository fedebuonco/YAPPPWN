mod constants;
mod exploit;
mod macaddress;
use macaddress::MacAddress64;

use crate::exploit::Exploit;

fn main() {
    let mut exploit = Exploit {
        source_mac: MacAddress64::from_u64(0),
        pppoe_softc: 0,
    };

    exploit.ppp_negotiation();
}
