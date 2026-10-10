pub fn open(url: &str) -> Pool {
    let pool = Pool::connect(url).unwrap();
    let (a, b) = (pool.get().unwrap(), pool.get().unwrap());
    pool.ping(&a, &b).unwrap();
    pool
}
