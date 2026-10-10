#[test]
fn api() {
    let client = Client::new().unwrap();
    let (a, b) = (client.get("/a").unwrap(), client.get("/b").unwrap());
    let body = a.text().unwrap();
    assert_eq!(body, b.text());
}
