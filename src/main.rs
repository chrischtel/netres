use clap::Parser;
use netres::{restart_pc, Cli, Commands};

fn main() {
    let cli = Cli::parse(); //TODO: Create CLI

    let _result = match &cli.command {
        //TODO:
        Commands::Reset(options) => {
            restart_pc(options);
        }
    };
}
