//! The arguments of a `find` call → a typed request (README.md#request, TOOLS.md#input-schema).

use serde_json::Value;

use crate::error::{ErrorCode, Report};
use crate::query::did_you_mean;

/// The request fields of the spec, in the order of the input schema.
pub const FIELDS: [&str; 14] = [
    "version",
    "dirs",
    "query",
    "queries",
    "within",
    "since",
    "timeout",
    "traversal",
    "output",
    "escape",
    "query_options",
    "kinds",
    "git",
    "languages",
];

/// A validated `find` request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    /// The query text.
    pub query: String,
}

/// Checks the arguments of `find`.
pub fn parse(arguments: &Value) -> Result<Request, Report> {
    let empty = serde_json::Map::new();
    let object = match arguments {
        Value::Null => &empty,
        Value::Object(object) => object,
        _ => {
            return Err(Report::new(
                ErrorCode::InvalidOption,
                "the arguments must be an object",
            ));
        }
    };
    if let Some(unknown) = object.keys().find(|k| !FIELDS.contains(&k.as_str())) {
        let expected = format!("expected one of: {}", FIELDS.join(", "));
        let hint = match did_you_mean(unknown, &FIELDS) {
            Some(s) => format!("did you mean '{s}'? {expected}"),
            None => expected,
        };
        return Err(Report::new(
            ErrorCode::InvalidOption,
            format!("unknown field '{unknown}'"),
        )
        .hint(hint));
    }
    if let Some(field) = FIELDS
        .iter()
        .find(|f| **f != "query" && object.contains_key(**f))
    {
        return Err(Report::not_implemented(&format!("the field '{field}'")));
    }
    match object.get("query") {
        None => Err(Report::not_implemented("the empty query")),
        Some(Value::String(q)) if q.trim().is_empty() => {
            Err(Report::not_implemented("the empty query"))
        }
        Some(Value::String(q)) => Ok(Request { query: q.clone() }),
        Some(_) => Err(Report::new(
            ErrorCode::InvalidOption,
            "'query' must be a string",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn text(arguments: &Value) -> String {
        parse(arguments).unwrap_err().to_text()
    }

    #[test]
    fn query() {
        assert_eq!(
            parse(&json!({ "query": "name = 'a'" })).unwrap().query,
            "name = 'a'"
        );
    }

    #[test]
    fn unknown_fields() {
        assert_eq!(
            text(&json!({ "query": "x", "colour": 1 })),
            "error[invalid_option]: unknown field 'colour'\n  hint: expected one of: version, dirs, query, queries, within, since, timeout, traversal, output, escape, query_options, kinds, git, languages\n"
        );
        assert!(text(&json!({ "dir": ["."] })).contains("hint: did you mean 'dirs'?"));
    }

    #[test]
    fn spec_fields_are_not_implemented() {
        assert_eq!(
            text(&json!({ "query": "x", "output": {} })),
            "error[not_implemented]: the field 'output' is not implemented yet\n"
        );
    }

    #[test]
    fn missing_or_empty_query() {
        for args in [json!({}), json!(null), json!({ "query": " \n" })] {
            assert_eq!(parse(&args).unwrap().query.trim(), "");
        }
    }

    #[test]
    fn wrong_types() {
        assert_eq!(
            text(&json!({ "query": 1 })),
            "error[invalid_option]: 'query' must be a string\n"
        );
        assert_eq!(
            text(&json!([])),
            "error[invalid_option]: the arguments must be an object\n"
        );
    }
}
