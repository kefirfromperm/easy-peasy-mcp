fn load() {
let raw = read(path)?;
let cfg = parse(&raw).unwrap();
}
