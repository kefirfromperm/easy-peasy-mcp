//! The arguments of a `find` call → a typed request (README.md#request, TOOLS.md#input-schema).

use serde_json::Value;

use crate::config::{self, Key, KeyValue};
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

/// The request fields which hold the keys of a configuration section, and their sections.
const SECTION_FIELDS: [(&str, &str); 6] = [
    ("traversal", "traversal"),
    ("output", "output"),
    ("escape", "escape"),
    ("query_options", "query"),
    ("kinds", "kinds"),
    ("git", "git"),
];

/// A validated `find` request.
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    /// The query text; empty for a missing `query`.
    pub query: String,
    /// The working directories as given; `None` — `server.default_dirs`.
    pub dirs: Option<Vec<String>>,
    /// The time limit; `None` — `server.timeout`; zero — no limit.
    pub timeout: Option<std::time::Duration>,
    /// The configuration values of the request: they replace the session values.
    pub config: Vec<(Key, KeyValue)>,
}

/// Whether a request field is implemented: the plain fields, and the section objects with an implemented key.
fn is_implemented(field: &str) -> bool {
    ["query", "dirs", "timeout"].contains(&field)
        || SECTION_FIELDS
            .iter()
            .any(|(f, section)| *f == field && Key::ALL.iter().any(|k| k.section() == *section))
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
        .find(|f| !is_implemented(f) && object.contains_key(**f))
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
    let timeout = match object.get("timeout") {
        None => None,
        Some(timeout) => Some(
            timeout
                .as_str()
                .and_then(crate::config::parse_duration)
                .ok_or_else(|| {
                    Report::new(
                        ErrorCode::InvalidOption,
                        "'timeout' must be a duration like \"10s\" (units ms, s, m)",
                    )
                })?,
        ),
    };
    let mut config = Vec::new();
    for (field, section) in SECTION_FIELDS {
        match object.get(field) {
            None => {}
            Some(Value::Object(fields)) => parse_section(field, section, fields, &mut config)?,
            Some(_) => {
                return Err(Report::new(
                    ErrorCode::InvalidOption,
                    format!("'{field}' must be an object"),
                ));
            }
        }
    }
    Ok(Request {
        query,
        dirs,
        timeout,
        config,
    })
}

/// The object of a request field which holds the keys of a configuration section: the implemented keys,
/// `not_implemented` for the others of the spec, `invalid_option` for an unknown key or a wrong value.
fn parse_section(
    field: &str,
    section: &str,
    fields: &serde_json::Map<String, Value>,
    config: &mut Vec<(Key, KeyValue)>,
) -> Result<(), Report> {
    let keys = config::section_keys(section);
    for (name, value) in fields {
        if let Some(key) = Key::find(section, name) {
            let checked = key.ty().check_json(&format!("{field}.{name}"), value)?;
            if let Some(feature) = key.unimplemented(&checked) {
                return Err(Report::not_implemented(&feature));
            }
            config.push((key, checked));
        } else if keys.contains(&name.as_str()) {
            return Err(Report::not_implemented(&format!(
                "the field '{field}.{name}'"
            )));
        } else {
            let expected = format!("expected one of: {}", keys.join(", "));
            let hint = match did_you_mean(name, keys) {
                Some(close) => format!("did you mean '{close}'? {expected}"),
                None => expected,
            };
            return Err(Report::new(
                ErrorCode::InvalidOption,
                format!("unknown field '{field}.{name}'"),
            )
            .hint(hint));
        }
    }
    Ok(())
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

    fn config(arguments: &Value) -> Vec<(Key, KeyValue)> {
        parse(arguments).unwrap().config
    }

    #[test]
    fn traversal() {
        assert_eq!(
            config(&json!({ "traversal": { "min_depth": 2, "max_depth": 1 } })),
            [
                (Key::MaxDepth, KeyValue::Whole(1)),
                (Key::MinDepth, KeyValue::Whole(2))
            ]
        );
        assert_eq!(config(&json!({})), []);
        assert_eq!(
            text(&json!({ "traversal": { "max_depth": 1.5 } })),
            "error[invalid_option]: 'traversal.max_depth' must be a whole number\n"
        );
        assert_eq!(
            text(&json!({ "traversal": { "min_depth": "2" } })),
            "error[invalid_option]: 'traversal.min_depth' must be a whole number\n"
        );
        assert_eq!(
            text(&json!({ "traversal": [] })),
            "error[invalid_option]: 'traversal' must be an object\n"
        );
        assert!(
            text(&json!({ "traversal": { "colour": 1 } }))
                .starts_with("error[invalid_option]: unknown field 'traversal.colour'\n  hint: expected one of: min_depth, ")
        );
        assert_eq!(
            config(&json!({ "traversal": { "follow_symlinks": true, "hidden": true } })),
            [
                (Key::FollowSymlinks, KeyValue::Bool(true)),
                (Key::Hidden, KeyValue::Bool(true))
            ]
        );
        assert_eq!(
            text(&json!({ "traversal": { "follow_symlinks": 1 } })),
            "error[invalid_option]: 'traversal.follow_symlinks' must be true or false\n"
        );
        assert_eq!(
            config(
                &json!({ "traversal": { "ignore_files": [".aiignore"], "git_excludes": false } })
            ),
            [
                (Key::GitExcludes, KeyValue::Bool(false)),
                (Key::IgnoreFiles, KeyValue::List(vec![".aiignore".into()]))
            ]
        );
        assert_eq!(
            text(&json!({ "traversal": { "ignore_files": [1] } })),
            "error[invalid_option]: 'traversal.ignore_files' must be a list of strings\n"
        );
        assert_eq!(
            config(&json!({ "traversal": { "ignore_paths": ["~/ignore", "a"] } })),
            [(
                Key::IgnorePaths,
                KeyValue::List(vec!["~/ignore".into(), "a".into()])
            )]
        );
        assert_eq!(
            text(&json!({ "traversal": { "ignore_paths": "a" } })),
            "error[invalid_option]: 'traversal.ignore_paths' must be a list of strings\n"
        );
        assert_eq!(
            config(&json!({ "traversal": { "exclude": ["**/target/**"] } })),
            [(Key::Exclude, KeyValue::List(vec!["**/target/**".into()]))]
        );
        assert_eq!(
            text(&json!({ "traversal": { "exclude": ["a", "[a-"] } })),
            "error[invalid_option]: invalid wildcard '[a-' in 'traversal.exclude'\n  hint: unclosed '['\n"
        );
        assert_eq!(
            text(&json!({ "traversal": { "exclude": "a" } })),
            "error[invalid_option]: 'traversal.exclude' must be a list of strings\n"
        );
        assert_eq!(
            config(&json!({ "traversal": { "path_case": "insensitive" } })),
            [(Key::PathCase, KeyValue::Text("insensitive".into()))]
        );
        assert_eq!(
            text(&json!({ "traversal": { "path_case": "upper" } })),
            "error[invalid_option]: 'traversal.path_case' must be one of: native, sensitive, insensitive\n"
        );
        assert_eq!(
            config(&json!({ "traversal": { "binary": "exclude" } })),
            [(Key::Binary, KeyValue::Text("exclude".into()))]
        );
        assert_eq!(
            text(&json!({ "traversal": { "binary": "text" } })),
            "error[invalid_option]: 'traversal.binary' must be one of: skip, search, exclude\n"
        );
        assert_eq!(
            config(&json!({ "traversal": { "max_file_size": "1.5K" } })),
            [(Key::MaxFileSize, KeyValue::Text("1.5K".into()))]
        );
        assert_eq!(
            text(&json!({ "traversal": { "max_file_size": 1024 } })),
            "error[invalid_option]: 'traversal.max_file_size' must be a size like \"10M\" (units B, K, M, G)\n"
        );
        assert_eq!(
            config(&json!({ "traversal": { "encoding": "Latin1" } })),
            [(Key::Encoding, KeyValue::Text("Latin1".into()))]
        );
        assert_eq!(
            text(&json!({ "traversal": { "encoding": "latin-99" } })),
            "error[invalid_option]: 'traversal.encoding' must be \"auto\" or an encoding name like \"windows-1252\"\n"
        );
        assert_eq!(
            config(&json!({ "traversal": { "fallback_encoding": "koi8-r" } })),
            [(Key::FallbackEncoding, KeyValue::Text("koi8-r".into()))]
        );
        assert_eq!(
            text(&json!({ "traversal": { "fallback_encoding": "auto" } })),
            "error[invalid_option]: 'traversal.fallback_encoding' must be an encoding name like \"windows-1251\"\n"
        );
        assert_eq!(
            config(&json!({ "traversal": { "archives": "compressed" } })),
            [(Key::Archives, KeyValue::Text("compressed".into()))]
        );
        assert_eq!(
            text(&json!({ "traversal": { "archives": "all" } })),
            "error[not_implemented]: the value 'all' of 'traversal.archives' is not implemented yet\n"
        );
        assert_eq!(
            text(&json!({ "traversal": { "max_archive_size": "1G" } })),
            "error[not_implemented]: the field 'traversal.max_archive_size' is not implemented yet\n"
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
