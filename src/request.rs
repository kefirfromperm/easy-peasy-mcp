//! The arguments of a `find` call → a typed request (README.md#request, TOOLS.md#input-schema).

use serde_json::Value;

use crate::config::{self, Key, KeyValue};
use crate::error::{ErrorCode, Report};
use crate::output_file::FileTarget;
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
    /// The languages of the request: they replace the languages with the same names.
    pub languages: Vec<crate::languages::Language>,
    /// `output.cursor`: where the page starts.
    pub cursor: Option<String>,
    /// `within`: the number of the result whose files are searched.
    pub within: Option<u64>,
    /// `since`: the number of the result the differences are to.
    pub since: Option<u64>,
    /// `output.file`: the file the response is written to.
    pub file: Option<FileTarget>,
}

impl Request {
    /// The value of a whole number key given in the request, not by the configuration.
    #[must_use]
    pub fn given(&self, key: Key) -> Option<usize> {
        self.config.iter().find_map(|(k, value)| match value {
            KeyValue::Whole(n) if *k == key => Some(*n),
            _ => None,
        })
    }
}

/// Whether a request field is implemented: the plain fields, and the section objects with an implemented key.
fn is_implemented(field: &str) -> bool {
    [
        "version",
        "query",
        "dirs",
        "within",
        "since",
        "timeout",
        "languages",
    ]
    .contains(&field)
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
    if let Some(version) = object.get("version") {
        check_version(version)?;
    }
    let within = object
        .get("within")
        .map(|v| result_id("within", v))
        .transpose()?;
    let since = object
        .get("since")
        .map(|v| result_id("since", v))
        .transpose()?;
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
    let timeout = object.get("timeout").map(timeout).transpose()?;
    let Sections {
        config,
        cursor,
        file,
    } = sections(object)?;
    let languages = match object.get("languages") {
        None => Vec::new(),
        Some(Value::Object(languages)) => languages
            .iter()
            .map(|(name, value)| {
                config::languages_from_json(name, value).map_err(|p| p.request_report())
            })
            .collect::<Result<Vec<_>, _>>()?,
        Some(_) => {
            return Err(Report::new(
                ErrorCode::InvalidOption,
                "'languages' must be an object",
            ));
        }
    };
    Ok(Request {
        query,
        dirs,
        timeout,
        config,
        languages,
        cursor,
        within,
        since,
        file,
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
            let mut keys = keys.to_vec();
            if field == "output" {
                keys.extend(["cursor", "file"]);
            }
            let expected = format!("expected one of: {}", keys.join(", "));
            let hint = match did_you_mean(name, &keys) {
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

/// The configuration values of the section fields of a request, `output.cursor` and `output.file`.
struct Sections {
    config: Vec<(Key, KeyValue)>,
    cursor: Option<String>,
    file: Option<FileTarget>,
}

/// The configuration values of the section fields of a request, `output.cursor` and `output.file`.
fn sections(object: &serde_json::Map<String, Value>) -> Result<Sections, Report> {
    let mut config = Vec::new();
    let mut cursor = None;
    let mut file = None;
    for (field, section) in SECTION_FIELDS {
        match object.get(field) {
            None => {}
            Some(Value::Object(fields)) if field == "output" => {
                let mut fields = fields.clone();
                cursor = match fields.remove("cursor") {
                    None => None,
                    Some(Value::String(text)) => Some(text),
                    Some(_) => {
                        return Err(Report::new(
                            ErrorCode::InvalidOption,
                            "'output.cursor' must be a string",
                        ));
                    }
                };
                file = fields
                    .remove("file")
                    .map(|f| file_target(&f))
                    .transpose()?
                    .flatten();
                parse_section(field, section, &fields, &mut config)?;
            }
            Some(Value::Object(fields)) => parse_section(field, section, fields, &mut config)?,
            Some(_) => {
                return Err(Report::new(
                    ErrorCode::InvalidOption,
                    format!("'{field}' must be an object"),
                ));
            }
        }
    }
    Ok(Sections {
        config,
        cursor,
        file,
    })
}

/// The duration of the field `timeout`.
fn timeout(value: &Value) -> Result<std::time::Duration, Report> {
    value
        .as_str()
        .and_then(crate::config::parse_duration)
        .ok_or_else(|| {
            Report::new(
                ErrorCode::InvalidOption,
                "'timeout' must be a duration like \"10s\" (units ms, s, m)",
            )
        })
}

/// The value of `output.file`: a path, `true` for a name of the server, `false` for no file.
///
/// # Errors
///
/// `invalid_option` for another value.
pub fn file_target(value: &Value) -> Result<Option<FileTarget>, Report> {
    match value {
        Value::Bool(false) => Ok(None),
        Value::Bool(true) => Ok(Some(FileTarget::Auto)),
        Value::String(path) if !path.is_empty() => Ok(Some(FileTarget::Path(path.clone()))),
        _ => Err(Report::new(
            ErrorCode::InvalidOption,
            "'output.file' must be a path or true",
        )),
    }
}

/// The number of a result id `rN` of the field `within` or `since`.
fn result_id(field: &str, value: &Value) -> Result<u64, Report> {
    value
        .as_str()
        .and_then(|id| id.strip_prefix('r'))
        .and_then(|n| n.parse().ok())
        .ok_or_else(|| {
            Report::new(
                ErrorCode::InvalidOption,
                format!("'{field}' must be a result id like 'r12'"),
            )
        })
}

/// The spec versions this server supports (TOOLS.md#versioning).
pub const SPEC_VERSIONS: [u64; 1] = [1];

/// Checks the `version` field: a whole number of at least 1 and a supported spec version.
fn check_version(version: &Value) -> Result<(), Report> {
    let Some(version) = version.as_u64().filter(|v| *v >= 1) else {
        return Err(Report::new(
            ErrorCode::InvalidOption,
            "'version' must be a whole number of at least 1",
        ));
    };
    if SPEC_VERSIONS.contains(&version) {
        return Ok(());
    }
    let supported: Vec<String> = SPEC_VERSIONS.iter().map(ToString::to_string).collect();
    Err(Report::new(
        ErrorCode::UnsupportedVersion,
        format!("the spec version {version} is not supported"),
    )
    .hint(format!("supported spec versions: {}", supported.join(", "))))
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
    fn version() {
        assert!(parse(&json!({ "version": 1, "query": "x" })).is_ok());
        assert_eq!(
            text(&json!({ "version": 2 })),
            "error[unsupported_version]: the spec version 2 is not supported\n  hint: supported spec versions: 1\n"
        );
        for bad in [json!(0), json!("1"), json!(1.5), json!(-1)] {
            assert_eq!(
                text(&json!({ "version": bad })),
                "error[invalid_option]: 'version' must be a whole number of at least 1\n"
            );
        }
    }

    #[test]
    fn within() {
        assert_eq!(
            parse(&json!({ "query": "x", "within": "r12" }))
                .unwrap()
                .within,
            Some(12)
        );
        for bad in [json!("12"), json!("r"), json!("rx"), json!(12)] {
            assert_eq!(
                text(&json!({ "within": bad })),
                "error[invalid_option]: 'within' must be a result id like 'r12'\n"
            );
        }
    }

    #[test]
    fn since() {
        assert_eq!(
            parse(&json!({ "query": "x", "since": "r3" }))
                .unwrap()
                .since,
            Some(3)
        );
        assert_eq!(
            text(&json!({ "since": "3" })),
            "error[invalid_option]: 'since' must be a result id like 'r12'\n"
        );
    }

    #[test]
    fn queries_are_a_batch() {
        // the engine hands them to the batch, a single request doesn't have them
        assert_eq!(
            text(&json!({ "queries": [] })),
            "error[not_implemented]: the field 'queries' is not implemented yet\n"
        );
    }

    #[test]
    fn output() {
        assert_eq!(
            config(&json!({ "output": { "format": "json" } })),
            [(Key::Format, KeyValue::Text("json".into()))]
        );
        assert_eq!(
            text(&json!({ "output": { "format": "xml" } })),
            "error[invalid_option]: 'output.format' must be one of: text, json\n"
        );
        assert_eq!(
            config(&json!({ "output": { "detail": "files" } })),
            [(Key::Detail, KeyValue::Text("files".into()))]
        );
        assert_eq!(
            config(&json!({ "output": { "detail": "facets", "facets": ["git", "ext"] } })),
            [
                (Key::Detail, KeyValue::Text("facets".into())),
                (
                    Key::Facets,
                    KeyValue::List(vec!["git".into(), "ext".into()])
                )
            ]
        );
        assert_eq!(
            config(&json!({ "output": { "context": 1 } })),
            [(Key::Context, KeyValue::Whole(1))]
        );
        assert_eq!(
            config(&json!({ "output": { "before": 1 } })),
            [(Key::Before, KeyValue::Whole(1))]
        );
        assert_eq!(
            config(&json!({ "output": { "after": 1 } })),
            [(Key::After, KeyValue::Whole(1))]
        );
        assert_eq!(
            config(&json!({ "output": { "blank_context": true } })),
            [(Key::BlankContext, KeyValue::Bool(true))]
        );
        assert_eq!(
            config(&json!({ "output": { "dedent": false } })),
            [(Key::Dedent, KeyValue::Bool(false))]
        );
        assert_eq!(
            config(&json!({ "output": { "group_dirs": false } })),
            [(Key::GroupDirs, KeyValue::Bool(false))]
        );
        assert_eq!(
            config(&json!({ "output": { "group_repeated": 0 } })),
            [(Key::GroupRepeated, KeyValue::Whole(0))]
        );
        assert_eq!(
            config(&json!({ "output": { "highlight": "none" } })),
            [(Key::Highlight, KeyValue::Text("none".into()))]
        );
        assert_eq!(
            text(&json!({ "output": { "highlight": "bold" } })),
            "error[invalid_option]: 'output.highlight' must be one of: auto, none, columns, markers\n"
        );
        assert_eq!(
            config(&json!({ "output": { "markers": ["<", ">"] } })),
            [(Key::Markers, KeyValue::List(vec!["<".into(), ">".into()]))]
        );
        assert_eq!(
            config(&json!({ "output": { "path_fragments": true } })),
            [(Key::PathFragments, KeyValue::Bool(true))]
        );
        assert_eq!(
            config(&json!({ "output": { "path_style": "absolute" } })),
            [(Key::PathStyle, KeyValue::Text("absolute".into()))]
        );
        assert_eq!(
            config(&json!({ "output": { "meta": true } })),
            [(Key::Meta, KeyValue::Bool(true))]
        );
        assert_eq!(
            config(&json!({ "output": { "sort": "path asc" } })),
            [(Key::Sort, KeyValue::Text("path asc".into()))]
        );
    }

    #[test]
    fn cursor() {
        assert_eq!(
            parse(&json!({ "output": { "cursor": "abc" } }))
                .unwrap()
                .cursor,
            Some("abc".to_string())
        );
        assert_eq!(parse(&json!({})).unwrap().cursor, None);
        assert_eq!(
            text(&json!({ "output": { "cursor": 5 } })),
            "error[invalid_option]: 'output.cursor' must be a string\n"
        );
        assert!(
            text(&json!({ "output": { "cursr": "a" } }))
                .ends_with(", path_style, omit, cursor, file\n")
        );
    }

    #[test]
    fn output_file() {
        let file = |value: Value| parse(&json!({ "output": { "file": value } })).map(|r| r.file);
        assert_eq!(
            file(json!("a/b.txt")).unwrap(),
            Some(FileTarget::Path("a/b.txt".into()))
        );
        assert_eq!(file(json!(true)).unwrap(), Some(FileTarget::Auto));
        assert_eq!(file(json!(false)).unwrap(), None);
        assert_eq!(parse(&json!({})).unwrap().file, None);
        for bad in [json!(""), json!(1), json!(["a"]), json!(null)] {
            assert_eq!(
                file(bad).unwrap_err().to_text(),
                "error[invalid_option]: 'output.file' must be a path or true\n"
            );
        }
        // not a configuration value
        assert_eq!(
            config(&json!({ "output": { "file": "a.txt", "max_output": 100 } })),
            [(Key::MaxOutput, KeyValue::Whole(100))]
        );
    }

    #[test]
    fn values_given_in_the_request() {
        let request = parse(&json!({ "output": { "max_output": 100 } })).unwrap();
        assert_eq!(request.given(Key::MaxOutput), Some(100));
        assert_eq!(request.given(Key::Budget), None);
    }

    #[test]
    fn output_limits() {
        assert_eq!(
            config(&json!({ "output": { "max_files": 1 } })),
            [(Key::MaxFiles, KeyValue::Whole(1))]
        );
        assert_eq!(
            config(&json!({ "output": { "max_lines": 1 } })),
            [(Key::MaxLines, KeyValue::Whole(1))]
        );
        assert_eq!(
            config(&json!({ "output": { "max_matches": 1 } })),
            [(Key::MaxMatches, KeyValue::Whole(1))]
        );
        assert_eq!(
            config(&json!({ "output": { "max_matches_per_file": 1 } })),
            [(Key::MaxMatchesPerFile, KeyValue::Whole(1))]
        );
        assert_eq!(
            config(&json!({ "output": { "max_line_length": 20 } })),
            [(Key::MaxLineLength, KeyValue::Whole(20))]
        );
        assert_eq!(
            config(&json!({ "output": { "max_output_per_file": 100 } })),
            [(Key::MaxOutputPerFile, KeyValue::Whole(100))]
        );
        assert_eq!(
            config(&json!({ "output": { "max_output": 100 } })),
            [(Key::MaxOutput, KeyValue::Whole(100))]
        );
        assert_eq!(
            config(&json!({ "output": { "budget": 1000 } })),
            [(Key::Budget, KeyValue::Whole(1000))]
        );
        assert_eq!(
            text(&json!({ "output": { "budget": 99 } })),
            "error[invalid_option]: 'output.budget' must be a whole number of at least 100\n"
        );
        assert_eq!(
            config(&json!({ "output": { "chars_per_token": 3.5 } })),
            [(Key::CharsPerToken, KeyValue::Number(3.5))]
        );
        assert_eq!(
            text(&json!({ "output": { "chars_per_token": 0 } })),
            "error[invalid_option]: 'output.chars_per_token' must be a positive number\n"
        );
        assert_eq!(
            config(&json!({ "output": { "funnel": "never" } })),
            [(Key::Funnel, KeyValue::Text("never".into()))]
        );
        assert_eq!(
            config(&json!({ "output": { "facet_depth": 1 } })),
            [(Key::FacetDepth, KeyValue::Whole(1))]
        );
        assert_eq!(
            config(&json!({ "output": { "facet_limit": 1 } })),
            [(Key::FacetLimit, KeyValue::Whole(1))]
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
            config(&json!({ "traversal": { "archives": "all" } })),
            [(Key::Archives, KeyValue::Text("all".into()))]
        );
        assert_eq!(
            config(&json!({ "traversal": { "max_archive_size": "1G" } })),
            [(Key::MaxArchiveSize, KeyValue::Text("1G".into()))]
        );
        assert_eq!(
            config(&json!({ "traversal": { "skip_generated": false } })),
            [(Key::SkipGenerated, KeyValue::Bool(false))]
        );
    }

    #[test]
    fn escape() {
        assert_eq!(
            config(&json!({ "escape": { "max_level": 1 } })),
            [(Key::MaxLevel, KeyValue::Whole(1))]
        );
        assert_eq!(
            text(&json!({ "escape": { "max_level": -1 } })),
            "error[invalid_option]: 'escape.max_level' must be a whole number\n"
        );
        assert_eq!(
            text(&json!({ "escape": { "depth": 1 } })),
            "error[invalid_option]: unknown field 'escape.depth'\n  hint: expected one of: max_level\n"
        );
    }

    #[test]
    fn query_options() {
        assert_eq!(
            config(&json!({ "query_options": { "fuzzy_min_score": 0.3 } })),
            [(Key::FuzzyMinScore, KeyValue::Number(0.3))]
        );
        assert_eq!(
            config(&json!({ "query_options": { "fuzzy_min_score": 1 } })),
            [(Key::FuzzyMinScore, KeyValue::Number(1.0))]
        );
        assert_eq!(
            text(&json!({ "query_options": { "fuzzy_min_score": 1.5 } })),
            "error[invalid_option]: 'query_options.fuzzy_min_score' must be a number from 0 to 1\n"
        );
    }

    #[test]
    fn kinds() {
        assert_eq!(
            config(&json!({ "kinds": { "generated_markers": ["AUTOGEN"] } })),
            [(
                Key::GeneratedMarkers,
                KeyValue::List(vec!["AUTOGEN".into()])
            )]
        );
        assert_eq!(
            text(&json!({ "kinds": { "generated_markers": "AUTOGEN" } })),
            "error[invalid_option]: 'kinds.generated_markers' must be a list of strings\n"
        );
        assert_eq!(
            config(&json!({ "kinds": { "generated_marker_lines": 2 } })),
            [(Key::GeneratedMarkerLines, KeyValue::Whole(2))]
        );
        assert_eq!(
            text(&json!({ "kinds": { "generated_marker_lines": 0 } })),
            "error[invalid_option]: 'kinds.generated_marker_lines' must be a whole number of at least 1\n"
        );
    }

    #[test]
    fn kinds_generated() {
        assert_eq!(
            config(&json!({ "kinds": { "generated": ["**/x/**"] } })),
            [(Key::KindsGenerated, KeyValue::List(vec!["**/x/**".into()]))]
        );
        assert_eq!(
            text(&json!({ "kinds": { "generated": ["[a-"] } })),
            "error[invalid_option]: invalid wildcard '[a-' in 'kinds.generated'\n  hint: unclosed '['\n"
        );
    }

    #[test]
    fn kinds_test() {
        assert_eq!(
            config(&json!({ "kinds": { "test": ["**/x/**"] } })),
            [(Key::KindsTest, KeyValue::List(vec!["**/x/**".into()]))]
        );
        assert_eq!(
            text(&json!({ "kinds": { "test": ["[a-"] } })),
            "error[invalid_option]: invalid wildcard '[a-' in 'kinds.test'\n  hint: unclosed '['\n"
        );
    }

    #[test]
    fn kinds_doc() {
        assert_eq!(
            config(&json!({ "kinds": { "doc": ["**/x/**"] } })),
            [(Key::KindsDoc, KeyValue::List(vec!["**/x/**".into()]))]
        );
        assert_eq!(
            text(&json!({ "kinds": { "doc": ["[a-"] } })),
            "error[invalid_option]: invalid wildcard '[a-' in 'kinds.doc'\n  hint: unclosed '['\n"
        );
    }

    #[test]
    fn kinds_config() {
        assert_eq!(
            config(&json!({ "kinds": { "config": ["**/x/**"] } })),
            [(Key::KindsConfig, KeyValue::List(vec!["**/x/**".into()]))]
        );
        assert_eq!(
            text(&json!({ "kinds": { "config": ["[a-"] } })),
            "error[invalid_option]: invalid wildcard '[a-' in 'kinds.config'\n  hint: unclosed '['\n"
        );
    }

    #[test]
    fn git() {
        assert_eq!(
            config(&json!({ "git": { "base": "develop" } })),
            [(Key::Base, KeyValue::Text("develop".into()))]
        );
        for base in [json!(""), json!("-x"), json!(1)] {
            assert_eq!(
                text(&json!({ "git": { "base": base } })),
                "error[invalid_option]: 'git.base' must be \"auto\" or a branch, tag or commit\n"
            );
        }
    }

    #[test]
    fn languages() {
        assert!(parse(&json!({ "languages": { "terraform": { "extensions": ["tf"] } } })).is_ok());
        assert_eq!(
            parse(&json!({ "languages": { "groovy": { "names": ["Jenkinsfile"] } } }))
                .unwrap()
                .languages[0]
                .names,
            ["Jenkinsfile"]
        );
        assert_eq!(
            parse(&json!({ "languages": { "deno": { "shebangs": ["deno"] } } }))
                .unwrap()
                .languages[0]
                .shebangs,
            ["deno"]
        );
        assert_eq!(
            parse(&json!({ "languages": { "tf": { "line_comments": ["#", "//"] } } }))
                .unwrap()
                .languages[0]
                .line_comments,
            ["#", "//"]
        );
        assert_eq!(
            parse(&json!({ "languages": { "pas": { "block_comments": [["(*", "*)"]] } } }))
                .unwrap()
                .languages[0]
                .block_comments,
            [("(*".to_string(), "*)".to_string())]
        );
        assert!(
            parse(&json!({ "languages": { "pas": { "nested_block_comments": true } } }))
                .unwrap()
                .languages[0]
                .nested_block_comments
        );
        assert_eq!(
            parse(&json!({ "languages": { "tf": { "strings": ["\""] } } }))
                .unwrap()
                .languages[0]
                .strings,
            ["\""]
        );
        assert!(
            !parse(&json!({ "languages": { "hcl": { "source": false } } }))
                .unwrap()
                .languages[0]
                .source
        );
        assert_eq!(
            text(&json!({ "languages": { "terraform": { "extensions": "tf" } } })),
            "error[invalid_option]: 'languages.terraform.extensions' must be a list of strings\n"
        );
        assert_eq!(
            text(&json!({ "languages": { "c++": { "extensions": ["c2"] } } })),
            "error[invalid_option]: invalid language name 'c++'\n  hint: a letter, then letters, digits, _ and - are expected\n"
        );
        assert_eq!(
            text(&json!({ "languages": { "rust": 1 } })),
            "error[invalid_option]: 'languages.rust' must be an object\n"
        );
        assert_eq!(
            text(&json!({ "languages": [] })),
            "error[invalid_option]: 'languages' must be an object\n"
        );
        assert!(
            text(&json!({ "languages": { "rust": { "extension": [] } } })).starts_with(
                "error[invalid_option]: unknown field 'languages.rust.extension'\n  hint: did you mean 'extensions'? expected one of: extensions, names,"
            )
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
