// never call .unwrap() here, see config.rs
fn main() {
    let cfg = config::load("app.toml");
    run(cfg)
}
