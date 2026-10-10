#[test]
fn parses_a_number() {
    let n: f64 = "1".parse().unwrap();
    assert_eq!(n, 1.0);
}
