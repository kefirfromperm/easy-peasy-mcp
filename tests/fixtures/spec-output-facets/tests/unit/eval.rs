#[test]
fn evaluates() {
    let a: f64 = "1".parse().unwrap();
    let b: f64 = "2".parse().unwrap();
    let sum = Some(a + b).expect("a sum");
    assert_eq!(sum, 3.0);
}
