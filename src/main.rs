//! The `easy-peasy-mcp` binary: command line, logging, the MCP server over stdio.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::Context as _;
use easy_peasy_mcp::{Engine, EngineOptions, SPEC_VERSION, config, mcp::Server};
use rmcp::ServiceExt as _;

const USAGE: &str = "usage: easy-peasy-mcp [--config <path>] [--root <dir>]... [--timeout <duration>] [--no-update] \
[--log-level <level>]
               [--version]";

/// What the command line asks for.
enum Command {
    Version,
    Serve {
        config: Option<PathBuf>,
        /// The `tracing` filter of `--log-level`.
        log_level: &'static str,
    },
}

/// Why the command line is rejected.
enum ArgError {
    Usage,
    /// An option of the spec this version doesn't implement yet.
    NotImplemented(String),
}

fn parse_args(args: &[String]) -> Result<Command, ArgError> {
    let mut config = None;
    let mut log_level = None;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--version" => return Ok(Command::Version),
            "--config" if config.is_none() => {
                config = Some(PathBuf::from(args.next().ok_or(ArgError::Usage)?));
            }
            "--log-level" if log_level.is_none() => {
                let level = args.next().ok_or(ArgError::Usage)?;
                log_level = Some(
                    ["error", "warn", "info", "debug"]
                        .into_iter()
                        .find(|l| l == level)
                        .ok_or(ArgError::Usage)?,
                );
            }
            "--root" | "--timeout" | "--no-update" => {
                return Err(ArgError::NotImplemented(arg.clone()));
            }
            _ => return Err(ArgError::Usage),
        }
    }
    Ok(Command::Serve {
        config,
        log_level: log_level.unwrap_or("warn"),
    })
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match parse_args(&args) {
        Ok(Command::Version) => {
            println!(
                "easy-peasy-mcp {}, spec {SPEC_VERSION}",
                env!("CARGO_PKG_VERSION")
            );
            ExitCode::SUCCESS
        }
        Ok(Command::Serve { config, log_level }) => {
            tracing_subscriber::fmt()
                .with_writer(std::io::stderr)
                .with_ansi(false)
                .with_env_filter(tracing_subscriber::EnvFilter::new(log_level))
                .init();
            let env = |name: &str| std::env::var(name).ok();
            let file = config::locate(config.as_deref(), &env, std::env::consts::OS, &|p| {
                p.is_file()
            });
            if let Some(file) = &file
                && let Err(e) = config::load(file)
            {
                eprintln!("easy-peasy-mcp: {e}");
                return ExitCode::FAILURE;
            }
            match run(file.as_deref()) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("easy-peasy-mcp: {e:#}");
                    ExitCode::FAILURE
                }
            }
        }
        Err(ArgError::NotImplemented(option)) => {
            eprintln!("easy-peasy-mcp: the option '{option}' is not implemented yet");
            ExitCode::from(2)
        }
        Err(ArgError::Usage) => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn run(config_file: Option<&Path>) -> anyhow::Result<()> {
    let start_dir = std::env::current_dir().context("the current directory is not readable")?;
    tracing::info!(
        "easy-peasy-mcp {} started in {}, config file: {}",
        env!("CARGO_PKG_VERSION"),
        start_dir.display(),
        config_file.map_or_else(|| "none".to_string(), |f| f.display().to_string())
    );
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
