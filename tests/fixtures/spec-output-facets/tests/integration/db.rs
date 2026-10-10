#[test]
fn db() {
    let db = Db::open(":memory:").unwrap();
    db.insert(1).unwrap();
}
