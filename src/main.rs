mod tui;
mod vpn;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "p2pvpn", about = "Decentralised VPN client")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect to a VPN provider
    Connect,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Connect => tui::run()?,
    }

    Ok(())
}
