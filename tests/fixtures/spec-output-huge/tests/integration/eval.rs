#[test]
fn divides_by_zero() {
    let error = app::eval("1 / 0").unwrap_err();
    assert!(error.message.contains("zero"));
}
