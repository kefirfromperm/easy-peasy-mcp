impl Config {
    fn load() {
        let cfg = parse(&raw).unwrap();
        let port = cfg.port.unwrap();
    }
}
