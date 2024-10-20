use clap::Parser;
use std::fmt::{Display, Formatter, Result};

#[derive(Parser, Debug)]
#[command(version, about="YAPPPWN, Yet Another PPPwn (in Rust)", long_about = None)]
pub struct Args {
    /// Interface where the ps4 is connected to
    #[arg(short, long)]
    pub interface: String,
    /// Firmware version from 1100 (11.00) to 900 (9.00)
    #[arg(long)]
    pub fw: u32,
    /// Stage 1 Payload Path
    #[arg(long)]
    pub stage_1: String,
    /// Stage 2 Payload Path
    #[arg(long)]
    pub stage_2: String,
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Manually format each field in the Args struct
        writeln!(f, "[+] Selected interface = {}", self.interface)?;
        writeln!(f, "[+] Selected firmare = {}", self.fw)?;
        writeln!(f, "[+] Selected stage 1 payload = {}", self.stage_1)?;
        writeln!(f, "[+] Selected stage 2 payload = {}", self.stage_2)?;
        Ok(())
    }
}

pub fn get_args() -> Args {
    Args::parse()
}
