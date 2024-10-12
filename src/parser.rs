use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub interface: String,
}

pub fn get_args() -> Args {
    let args = Args::parse();
    args
}
