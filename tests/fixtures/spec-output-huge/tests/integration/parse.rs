#[test]
fn reports_the_column() {
    let error = app::parse("1 +").unwrap_err();
    assert_eq!(error.col, 3);
}
