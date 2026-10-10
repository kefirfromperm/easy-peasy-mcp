#[test]
fn cli() {
    let out = run(&["1 + 2"]).expect("runs");
    let n: f64 = out.trim().parse().expect("a number");
    assert_eq!(n, 3.0);
}
