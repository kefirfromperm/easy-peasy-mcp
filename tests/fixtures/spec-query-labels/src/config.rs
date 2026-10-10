pub fn load(path: &str) -> Config {
    let raw = std::fs::read_to_string(path).unwrap();
    let cfg: Config = toml::from_str(&raw).expect("valid TOML");
    cfg
}
