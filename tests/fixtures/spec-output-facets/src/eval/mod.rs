pub fn eval(text: &str) -> f64 {
    let value: f64 = text.trim().parse().unwrap();
    value
}
