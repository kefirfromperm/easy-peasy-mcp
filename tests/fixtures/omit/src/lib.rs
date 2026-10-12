pub fn load() -> Config {
    let raw = read().unwrap();
    let text = decode(&raw);
    parse(&text).unwrap()
}
