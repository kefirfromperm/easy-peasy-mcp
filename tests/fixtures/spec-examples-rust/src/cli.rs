//! The command line.

use std::path::PathBuf;

use clap::Parser;

/// A small interpreter for the `.ez` scripting language.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    /// The configuration file.
    #[arg(long, default_value = "interp.toml")]
    pub config: PathBuf,
    /// Print the value of every script.
    // TODO: move to the configuration file
    #[arg(long)]
    pub trace: bool,
}
