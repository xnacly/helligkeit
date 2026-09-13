use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// disable stdout/stderr writes
    #[arg(short, long)]
    pub silent: bool,

    #[command(subcommand)]
    pub command: Action,
}

#[derive(Subcommand)]
pub enum Action {
    /// Device info
    Info { device: String },
    /// Get current brightness of device
    Get { device: String },
    /// Set device to value
    Set { device: String, value: usize },

    /// List all controllable and supported devices
    List,
}
