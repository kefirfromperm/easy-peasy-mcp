pub fn parse_query(input: &str) -> Query {
    Query::new(input)
}

pub fn parse_query_fast(input: &str) -> Query {
    parse_query(input)
}
