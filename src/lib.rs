use clap::{Parser, Subcommand};

mod reset;
pub use reset::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate captions using deepgram
    Reset(Reset),
}
