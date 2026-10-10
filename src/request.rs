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
    /// The query text; empty for a missing `query`.
    pub query: String,
    /// The working directories as given; `None` — `server.default_dirs`.
    pub dirs: Option<Vec<String>>,
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
        .find(|f| !["query", "dirs"].contains(*f) && object.contains_key(**f))
    {
        return Err(Report::not_implemented(&format!("the field '{field}'")));
    }
    let dirs = match object.get("dirs") {
        None => None,
        Some(Value::Array(dirs)) if !dirs.is_empty() => Some(
            dirs.iter()
                .map(|d| d.as_str().map(str::to_string))
                .collect::<Option<Vec<_>>>()
                .ok_or_else(invalid_dirs)?,
        ),
        Some(_) => return Err(invalid_dirs()),
    };
    let query = match object.get("query") {
        None => String::new(),
        Some(Value::String(q)) => q.clone(),
        Some(_) => {
            return Err(Report::new(
                ErrorCode::InvalidOption,
                "'query' must be a string",
            ));
        }
    };
    Ok(Request { query, dirs })
}

/// The error of a `dirs` field which doesn't match the schema.
fn invalid_dirs() -> Report {
    Report::new(
        ErrorCode::InvalidOption,
        "'dirs' must be a non-empty list of strings",
    )
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
    fn dirs() {
        assert_eq!(
            parse(&json!({ "dirs": ["a", "../b"] })).unwrap().dirs,
            Some(vec!["a".to_string(), "../b".to_string()])
        );
        assert_eq!(parse(&json!({})).unwrap().dirs, None);
        for dirs in [json!([]), json!("a"), json!([1])] {
            assert_eq!(
                text(&json!({ "dirs": dirs })),
                "error[invalid_option]: 'dirs' must be a non-empty list of strings\n"
            );
        }
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
