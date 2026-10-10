pub fn run() -> i32 {
    let (a, b) = rayon::join(|| 1, || 2);
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async move { a + b })
}
