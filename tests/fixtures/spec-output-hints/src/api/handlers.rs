use crate::parser::parse_query;

pub fn search(raw: &str) -> usize {
    parse_query(raw).len()
}
