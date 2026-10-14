pub fn load() -> Config {
    let raw = read().unwrap().unwrap();
    let text = decode(&raw);
    parse(&text).unwrap()
}
