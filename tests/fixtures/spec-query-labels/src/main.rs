fn main() {
    let cfg = config::load("app.toml");
    run(cfg).unwrap();
}
