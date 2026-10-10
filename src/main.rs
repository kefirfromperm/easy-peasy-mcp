//! The `easy-peasy-mcp` binary: command line, logging, the MCP server over stdio.

use std::process::ExitCode;

use anyhow::Context as _;
use easy_peasy_mcp::{Engine, EngineOptions, SPEC_VERSION, mcp::Server};
use rmcp::ServiceExt as _;

const USAGE: &str = "usage: easy-peasy-mcp [--version]";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => match run() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("easy-peasy-mcp: {e:#}");
                ExitCode::FAILURE
            }
        },
        [flag] if flag == "--version" => {
            println!(
                "easy-peasy-mcp {}, spec {SPEC_VERSION}",
                env!("CARGO_PKG_VERSION")
            );
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_env_filter(tracing_subscriber::EnvFilter::new("warn"))
        .init();
    let start_dir = std::env::current_dir().context("the current directory is not readable")?;
    let server = Server::new(Engine::new(EngineOptions::new(start_dir)));
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async {
        let service = server
            .serve(rmcp::transport::stdio())
            .await
            .context("MCP initialization failed")?;
        service.waiting().await.context("the MCP service failed")?;
        anyhow::Ok(())
    })
}
