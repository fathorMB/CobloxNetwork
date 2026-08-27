use clap::{Parser, Subcommand};
use std::path::PathBuf;

use coblox_core::hash::{ChainId, Digest32};
use coblox_node::config::{NodeConfig, devnet_4_validator_set, devnet_timeouts};
use coblox_node::node::NodeRunner;
use coblox_node::signer::SigningKey;

#[derive(Debug, Parser)]
#[command(name = "coblox-node", version = coblox_core::core_version())]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Start the node service and participate in BFT consensus over P2P network.
    Start {
        /// Validator ID (e.g. val-000, val-001, val-002, val-003)
        #[arg(long, default_value = "val-000")]
        validator_id: String,

        /// Seed index for deterministic devnet key derivation (0..3)
        #[arg(long, default_value_t = 0)]
        seed_index: u8,

        /// Custom 32-byte (64 hex digit) seed, overriding `seed_index`.
        ///
        /// Read from the command line, where every local user can see it in the
        /// process table. Acceptable only because a devnet key is a public
        /// constant to begin with; see the runbook.
        #[arg(long)]
        seed_hex: Option<String>,

        /// Directory for WAL and block storage. Required: there is no default,
        /// and the reason is that the previous one — `./data/val-000` — put a
        /// validator's signed votes inside the source tree of a public
        /// repository, one `git add -A` away from being published, and left the
        /// working tree dirty after every run. [REVIEW-049] RF-005, [DEBT-049].
        #[arg(long)]
        data_dir: PathBuf,

        /// Multiaddress to listen on (e.g. /ip4/127.0.0.1/tcp/9001)
        #[arg(long, default_value = "/ip4/127.0.0.1/tcp/9001")]
        listen_addr: String,

        /// Multiaddresses of seed peers to connect to
        #[arg(long, value_delimiter = ',')]
        seed_peers: Vec<String>,

        /// Optional target height to finalize before exiting (useful for devnet tests)
        #[arg(long)]
        target_height: Option<u64>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Some(Command::Start {
            validator_id,
            seed_index,
            seed_hex,
            data_dir,
            listen_addr,
            seed_peers,
            target_height,
        }) => {
            println!(
                "Starting coblox-node validator={validator_id} pid={}",
                std::process::id()
            );

            let signing_key = if let Some(hex_str) = seed_hex {
                let bytes = hex::decode(&hex_str)?;
                // A 31-byte seed used to panic on the slice index.
                // [REVIEW-049] RF-017.
                let seed: [u8; 32] = bytes.as_slice().try_into().map_err(|_| {
                    format!(
                        "--seed-hex must decode to exactly 32 bytes (64 hex digits); got {} byte(s)",
                        bytes.len()
                    )
                })?;
                SigningKey::from_seed(&seed)
            } else {
                let seed = [seed_index + 1; 32];
                SigningKey::from_seed(&seed)
            };

            let (validator_set, _) = devnet_4_validator_set();
            let chain_id = ChainId::from_digest(Digest32::repeated(0x7a));
            let genesis_block_id = Digest32::repeated(0x01);

            let config = NodeConfig {
                validator_id: validator_id.clone(),
                node_id: validator_id,
                signing_key,
                network_id: "coblox-devnet-0".to_owned(),
                chain_id,
                genesis_block_id,
                listen_addr,
                seed_peers,
                data_dir,
                validator_set,
                timeouts: devnet_timeouts(),
                target_height,
            };

            let (mut runner, network) = NodeRunner::new(config)?;

            if let Some(net) = network {
                tokio::spawn(async move {
                    net.run().await;
                });
            }

            runner.run().await?;
            println!("Target height reached. Shutting down.");
            Ok(())
        }
        None => {
            println!("coblox-node {}", coblox_core::core_version());
            Ok(())
        }
    }
}
