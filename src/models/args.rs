use clap::Parser;

/// Arguments for the CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Should we pull the last month, if not we pull the current month
    #[arg(short, default_value_t = false)]
    pub last_month: bool,
}
