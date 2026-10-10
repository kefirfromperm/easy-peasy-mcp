#[test]
fn loads_the_example() {
    let cfg = app::config::load("app.toml".as_ref()).unwrap();
    assert_eq!(cfg.port, Some(8080));
}
