use clap::Parser;
use miette::{IntoDiagnostic, Result};
use netres::{restart_pc, Cli, Commands};

fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse(); //TODO: Create CLI

    let result = match &cli.command {
        //TODO:
        Commands::Reset(options) => {
            restart_pc(options);
        }
    };
}
