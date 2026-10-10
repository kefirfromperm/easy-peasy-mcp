use std::path::Path;

fn main() {
    let settings = app::config::load_config(Path::new("config.toml")).expect("a valid config");
    app::server::run(settings);
}
