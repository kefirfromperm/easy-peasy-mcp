//! The application.

mod cache;
mod cli;
mod config;
mod eval;
mod output;
mod parser;
mod server;
mod walk;

fn main() {
    let args = cli::parse();
    let cfg = config::load();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(cfg.threads)
        .enable_all()
        .build()
        .unwrap();
    let listener = runtime.block_on(tokio::net::TcpListener::bind(("0.0.0.0", cfg.port))).unwrap();
    let state = server::State::new(&cfg.root).unwrap();
    if args.check {
        let files = walk::files(&cfg.root).unwrap();
        println!("{} files", files.len());
        return;
    }
    runtime.block_on(server::run(listener, state)).unwrap();
}
