use clap::{ArgGroup, Parser};
use std::fmt::{Display, Formatter, Result};

#[derive(Parser, Debug)]
#[command(version, about="YAPPPWN, Yet Another PPPwn (in Rust)", long_about = None)]
#[command(group = ArgGroup::new("exploit")
    .args(&["interface", "fw", "stage_1", "stage_2", "retries"])
    .multiple(true)
    .required(false)
)]
pub struct Args {
    /// Interface where the ps4 is connected to
    #[arg(short, long)]
    pub interface: Option<String>,
    /// Firmware version from 1100 (11.00) to 900 (9.00)
    #[arg(long)]
    pub fw: Option<u32>,
    /// Stage 1 Payload Path
    #[arg(long)]
    pub stage_1: Option<String>,
    /// Stage 2 Payload Path
    #[arg(long)]
    pub stage_2: Option<String>,
    /// Automatic retries for memory corruption stage
    #[arg(short, default_value = "3")]
    pub retries: Option<usize>,
    /// List all network interfaces and exit
    #[arg(long)]
    pub list_interfaces: bool,
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Print only if the option is present
        if let Some(interface) = &self.interface {
            writeln!(f, "[+] Selected interface = {}", interface)?;
        }
        if let Some(fw) = &self.fw {
            writeln!(f, "[+] Selected firmware = {}", fw)?;
        }
        if let Some(stage_1) = &self.stage_1 {
            writeln!(f, "[+] Selected stage 1 payload = {}", stage_1)?;
        }
        if let Some(stage_2) = &self.stage_2 {
            writeln!(f, "[+] Selected stage 2 payload = {}", stage_2)?;
        }
        if let Some(retries) = &self.retries {
            writeln!(
                f,
                "[+] Automatic retries for memory corruption = {}",
                retries
            )?;
        }
        Ok(())
    }
}

pub fn get_args() -> Args {
    Args::parse()
}
