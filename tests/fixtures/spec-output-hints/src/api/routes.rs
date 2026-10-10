pub const ROUTES: &[&str] = &["/search"];

pub fn route(raw: &str) -> usize {
    crate::parser::parse_query(raw).len()
}
