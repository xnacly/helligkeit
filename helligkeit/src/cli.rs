use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    /// device to operate on
    pub target: String,

    #[arg(short, long)]
    /// Match devices by substring matching
    pub like: bool,

    #[command(subcommand)]
    pub command: Option<Action>,

    /// disable stdout/stderr writes
    #[arg(short, long)]
    pub silent: bool,

    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Action {
    /// Device info
    Info,
    /// Get current brightness of device
    Get,
    /// Set device to value
    Set { value: usize },

    /// List all controllable and supported devices
    List,
}
