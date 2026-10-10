//! The server binary.

mod config;
mod error;
mod server;

use clap::Parser;
use crate::config::Config;

/// The command line.
#[derive(Parser)]
struct Args {
    config: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let cfg = config::load(&args.config).unwrap();
    run(cfg)
}

/// Starts the server and blocks until it stops.
fn run(cfg: Config) {
    let threads = cfg.threads();
    let address = cfg.address();
    tracing::info!("listening on {address} with {threads} threads");
    if let Err(e) = server::serve(&address, threads) {
        tracing::error!("the server failed: {e}");
        std::process::exit(1);
    }
}
