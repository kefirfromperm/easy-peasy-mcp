fn load() {
let raw = read(path)?;
let cfg = parse(&raw).unwrap();
let port = cfg.port;
Ok(cfg)
}
fn save() {
let out = render();
write(out).unwrap();
}
