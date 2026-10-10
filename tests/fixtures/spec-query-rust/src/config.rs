use std::fs;

pub fn load(path: &str) -> Config {
    let raw = fs::read_to_string(path).unwrap();
    let cfg: Config = toml::from_str(&raw).expect("valid config");
    cfg
}
