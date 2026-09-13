use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// disable stdout/stderr writes
    #[arg(short, long)]
    silent: bool,

    #[command(subcommand)]
    commands: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Device(s) info
    Info { device: Option<String> },
    /// Get current brightness of device
    Get { device: String },
    /// Set device to value
    Set { device: String, value: usize },

    /// List all controllable and supported devices
    List,
}
