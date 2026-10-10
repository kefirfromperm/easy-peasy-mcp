use std::path::Path;

use app::config::load_config;

fn main() {
    // no unwrap( here: a broken config.toml is reported with its reason
    let settings = load_config(Path::new("config.toml")).expect("a valid config.toml");
    app::server::run(settings);
}
