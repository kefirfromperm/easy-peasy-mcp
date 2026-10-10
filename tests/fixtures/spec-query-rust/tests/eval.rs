#[test]
fn eval_small_input() {
    assert_eq!(eval("1").unwrap(), 1);
}

#[test]
fn eval_big_input_slow() {
    assert!(eval(&"1+".repeat(100_000)).is_ok());
}
