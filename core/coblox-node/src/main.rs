use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "coblox-node", version = coblox_core::core_version())]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Start the node service (networking is introduced by a later specification).
    Start,
}

fn main() {
    match Cli::parse().command {
        Some(Command::Start) => println!("coblox-node start is not configured yet"),
        None => println!("coblox-node {}", coblox_core::core_version()),
    }
}
