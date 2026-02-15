mod contract;
mod tui;
mod vpn;
use crate::{tui::signer, vpn::OpenVpnCredentials};
use anyhow::Context;
use clap::{Parser, Subcommand};
use ethexe_ethereum::{primitives::Address, Ethereum};
use gsigner::PrivateKey;
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "p2pvpn", about = "Decentralised VPN client")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect to a VPN provider
    Connect {
        /// OpenVPN username (used only when profile needs auth-user-pass)
        #[arg(long)]
        ovpn_username: Option<String>,
        /// OpenVPN password (used only when profile needs auth-user-pass)
        #[arg(long)]
        ovpn_password: Option<String>,
        /// Ethereum address of user to used for ranking VPN providers (optional).
        ///
        /// You have to import the corresponding private key using `import-key` command for this to work.
        #[arg(long)]
        sender_address: Address,
    },
    DeployContract {
        sender_address: gsigner::Address,
    },
    ImportKey {
        private_key: PrivateKey,
    },
}

const VARA_ETH_VALIDATOR: &str = "wss://vara-eth-validator-2.gear-tech.io:9944";
const ETH_RPC: &str = "wss://hoodi-reth-rpc.gear-tech.io/ws";
const ROUTER_ADDRESS: &str = "0xBC888a8B050B9B76a985d91c815d2c4f2131a58A";
const VPN_ADDRESS: &str = "0x037c5239b22fbc60905fbc6b94eb179fd6221bee";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Connect {
            ovpn_username,
            ovpn_password,
            sender_address,
        } => {
            let signer = signer()?;
            let router =
                Address::from_str(ROUTER_ADDRESS).with_context(|| "Invalid router address")?;

            let eth = Ethereum::new(ETH_RPC, router.into(), signer, sender_address.into()).await?;
            let api = ethexe_sdk::VaraEthApi::new(ETH_RPC, eth).await?;

            let credentials = match (ovpn_username, ovpn_password) {
                (Some(username), Some(password)) => Some(OpenVpnCredentials { username, password }),
                (None, None) => None,
                _ => anyhow::bail!(
                    "both --ovpn-username and --ovpn-password must be provided together"
                ),
            };
            let vpn_addr =
                gsigner::Address::from_str(VPN_ADDRESS).with_context(|| "Invalid VPN address")?;
            api.wrapped_vara()
                .approve(vpn_addr.into(), 10000 * 10u128.pow(12))
                .await?;
            api.mirror(vpn_addr.into())
                .executable_balance_top_up(10000 * 10u128.pow(12))
                .await?;
            tui::connect(
                api,
                gsigner::Address::from_str(VPN_ADDRESS).with_context(|| "Invalid VPN address")?,
                credentials,
            )
            .await?
        }
        Commands::DeployContract { sender_address } => {
            tui::deploy(sender_address).await?;
        }
        Commands::ImportKey { private_key } => {
            tui::import_key(private_key).await?;
        }
    }

    Ok(())
}
