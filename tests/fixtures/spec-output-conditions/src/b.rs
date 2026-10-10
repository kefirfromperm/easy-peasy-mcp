pub fn host(raw: &str) -> String {
    let url = url::Url::parse(raw).expect("a url");
    url.host_str().map(str::to_string).expect("a host")
}
