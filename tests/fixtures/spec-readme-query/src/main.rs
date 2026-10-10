// Never unwrap() the configuration of a user without a message.
use std::path::PathBuf;

mod config;

fn main() {
    let path = PathBuf::from("app.toml");
    let cfg = config::load(&path).unwrap();
    let port = cfg.port.unwrap_or(8080);
    app::run(cfg, port);
}
