fn main() {
    let cfg = load().unwrap();
    // TODO handle the error
    run(cfg);
}
