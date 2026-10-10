fn load(raw: &str) -> Config {
    let cfg: Config = toml::from_str(raw).unwrap();
    cfg
}
