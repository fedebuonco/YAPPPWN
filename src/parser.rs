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
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Manually format each field in the Args struct
        writeln!(f, "[+] Selected interface = {}", self.interface)?;
        writeln!(f, "[+] Selected firmare = {}", self.fw)?;
        Ok(())
    }
}

pub fn get_args() -> Args {
    let args = Args::parse();
    args
}
