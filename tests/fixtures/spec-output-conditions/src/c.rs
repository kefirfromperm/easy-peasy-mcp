pub fn path(raw: &str) -> String {
    let url = url::Url::parse(raw).unwrap();
    url.path_segments().expect("a base").collect()
}
