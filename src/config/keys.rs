//! The changeable keys of the configuration: their section, name, type and default (CONFIG.md#sections).

use std::fmt::Write as _;

use crate::error::{ErrorCode, Report};
use crate::query::{nfc, parse_size, wildcard};

/// A key which `update_config` and the request change: every key of the spec except the read-only ones, as far as
/// it is implemented. The order of the variants is the order of the spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    DefaultDirs,
    Timeout,
    Threads,
    ResultCache,
    MinDepth,
    MaxDepth,
    FollowSymlinks,
    Hidden,
    IgnoreFiles,
    GitExcludes,
    IgnorePaths,
    Exclude,
    PathCase,
    Binary,
    MaxFileSize,
    Encoding,
    FallbackEncoding,
    Archives,
    MaxArchiveSize,
    SkipGenerated,
    Format,
    Detail,
    Funnel,
    Meta,
    Context,
    Before,
    After,
    PathFragments,
    Highlight,
    Markers,
    MaxFiles,
    MaxLines,
    MaxMatches,
    MaxMatchesPerFile,
    MaxLineLength,
    MaxOutputPerFile,
    MaxOutput,
    Dedent,
    BlankContext,
    GroupDirs,
    GroupRepeated,
    Sort,
    PathStyle,
    MaxLevel,
    FuzzyMinScore,
    GeneratedMarkers,
    GeneratedMarkerLines,
    KindsGenerated,
    KindsTest,
    KindsDoc,
    KindsConfig,
    Base,
}

impl Key {
    /// All keys in the order of the spec.
    pub const ALL: [Key; 52] = [
        Self::DefaultDirs,
        Self::Timeout,
        Self::Threads,
        Self::ResultCache,
        Self::MinDepth,
        Self::MaxDepth,
        Self::FollowSymlinks,
        Self::Hidden,
        Self::IgnoreFiles,
        Self::GitExcludes,
        Self::IgnorePaths,
        Self::Exclude,
        Self::PathCase,
        Self::Binary,
        Self::MaxFileSize,
        Self::Encoding,
        Self::FallbackEncoding,
        Self::Archives,
        Self::MaxArchiveSize,
        Self::SkipGenerated,
        Self::Format,
        Self::Detail,
        Self::Funnel,
        Self::Meta,
        Self::Context,
        Self::Before,
        Self::After,
        Self::PathFragments,
        Self::Highlight,
        Self::Markers,
        Self::MaxFiles,
        Self::MaxLines,
        Self::MaxMatches,
        Self::MaxMatchesPerFile,
        Self::MaxLineLength,
        Self::MaxOutputPerFile,
        Self::MaxOutput,
        Self::Dedent,
        Self::BlankContext,
        Self::GroupDirs,
        Self::GroupRepeated,
        Self::Sort,
        Self::PathStyle,
        Self::MaxLevel,
        Self::FuzzyMinScore,
        Self::GeneratedMarkers,
        Self::GeneratedMarkerLines,
        Self::KindsGenerated,
        Self::KindsTest,
        Self::KindsDoc,
        Self::KindsConfig,
        Self::Base,
    ];

    /// The section, the name and the type.
    fn spec(self) -> (&'static str, &'static str, Type) {
        match self {
            Self::DefaultDirs => ("server", "default_dirs", Type::NonEmptyList("paths")),
            Self::Timeout => ("server", "timeout", Type::Duration),
            Self::Threads => (
                "server",
                "threads",
                Type::Whole(Some("0: the number of CPU cores")),
            ),
            Self::ResultCache => ("server", "result_cache", Type::Whole(Some("0: off"))),
            Self::MinDepth => ("traversal", "min_depth", Type::Whole(None)),
            Self::MaxDepth => ("traversal", "max_depth", Type::OptWhole),
            Self::FollowSymlinks => ("traversal", "follow_symlinks", Type::Bool),
            Self::Hidden => ("traversal", "hidden", Type::Bool),
            Self::IgnoreFiles => ("traversal", "ignore_files", Type::List("file names")),
            Self::GitExcludes => ("traversal", "git_excludes", Type::Bool),
            Self::IgnorePaths => ("traversal", "ignore_paths", Type::List("paths")),
            Self::Exclude => ("traversal", "exclude", Type::Wildcards),
            Self::PathCase => (
                "traversal",
                "path_case",
                Type::Choice(&["native", "sensitive", "insensitive"]),
            ),
            Self::Binary => (
                "traversal",
                "binary",
                Type::Choice(&["skip", "search", "exclude"]),
            ),
            Self::MaxFileSize => ("traversal", "max_file_size", Type::Size),
            Self::Encoding => ("traversal", "encoding", Type::Encoding),
            Self::FallbackEncoding => ("traversal", "fallback_encoding", Type::OptEncoding),
            Self::Archives => (
                "traversal",
                "archives",
                Type::Choice(&["none", "compressed", "all"]),
            ),
            Self::MaxArchiveSize => ("traversal", "max_archive_size", Type::Size),
            Self::SkipGenerated => ("traversal", "skip_generated", Type::Bool),
            Self::Format => ("output", "format", Type::Choice(&["text", "json"])),
            Self::Detail => (
                "output",
                "detail",
                Type::Choice(&["exists", "count", "facets", "files", "lines"]),
            ),
            Self::Context => ("output", "context", Type::Whole(None)),
            Self::Before => ("output", "before", Type::OptWhole),
            Self::After => ("output", "after", Type::OptWhole),
            Self::Dedent => ("output", "dedent", Type::Bool),
            Self::BlankContext => ("output", "blank_context", Type::Bool),
            Self::PathFragments => ("output", "path_fragments", Type::Bool),
            Self::Meta => ("output", "meta", Type::Bool),
            Self::Funnel => (
                "output",
                "funnel",
                Type::Choice(&["auto", "always", "never"]),
            ),
            Self::Highlight => (
                "output",
                "highlight",
                Type::Choice(&["auto", "none", "columns", "markers"]),
            ),
            Self::Markers => ("output", "markers", Type::Markers),
            Self::MaxFiles => ("output", "max_files", Type::AtLeast(1)),
            Self::MaxLines => ("output", "max_lines", Type::AtLeast(1)),
            Self::MaxMatches => ("output", "max_matches", Type::AtLeast(1)),
            Self::MaxMatchesPerFile => ("output", "max_matches_per_file", Type::AtLeast(1)),
            Self::MaxLineLength => ("output", "max_line_length", Type::AtLeast(20)),
            Self::MaxOutputPerFile => ("output", "max_output_per_file", Type::AtLeast(100)),
            Self::MaxOutput => ("output", "max_output", Type::AtLeast(100)),
            Self::GroupDirs => ("output", "group_dirs", Type::Bool),
            Self::GroupRepeated => ("output", "group_repeated", Type::Whole(Some("0: off"))),
            Self::Sort => (
                "output",
                "sort",
                Type::Choice(&[
                    "relevance",
                    "path asc",
                    "path desc",
                    "modified asc",
                    "modified desc",
                    "size asc",
                    "size desc",
                    "matches asc",
                    "matches desc",
                ]),
            ),
            Self::PathStyle => (
                "output",
                "path_style",
                Type::Choice(&["relative", "absolute"]),
            ),
            Self::MaxLevel => ("escape", "max_level", Type::Whole(None)),
            Self::FuzzyMinScore => ("query", "fuzzy_min_score", Type::Score),
            Self::GeneratedMarkers => ("kinds", "generated_markers", Type::List("strings")),
            Self::GeneratedMarkerLines => ("kinds", "generated_marker_lines", Type::AtLeast(1)),
            Self::KindsConfig => ("kinds", "config", Type::Wildcards),
            Self::Base => ("git", "base", Type::GitBase),
            Self::KindsDoc => ("kinds", "doc", Type::Wildcards),
            Self::KindsTest => ("kinds", "test", Type::Wildcards),
            Self::KindsGenerated => ("kinds", "generated", Type::Wildcards),
        }
    }

    /// The built-in default.
    #[must_use]
    pub fn default_value(self) -> Value {
        let list = |items: &[&str]| Value::List(items.iter().map(ToString::to_string).collect());
        match self {
            Self::DefaultDirs => list(&["."]),
            Self::Timeout => Value::Text("30s".to_string()),
            Self::Threads | Self::MinDepth | Self::Context => Value::Whole(0),
            Self::ResultCache | Self::MaxMatchesPerFile => Value::Whole(50),
            Self::MaxFiles => Value::Whole(100),
            Self::MaxLines => Value::Whole(500),
            Self::MaxLineLength => Value::Whole(300),
            Self::MaxOutputPerFile => Value::Whole(4000),
            Self::MaxOutput => Value::Whole(40000),
            Self::MaxMatches => Value::Whole(1000),
            Self::MaxLevel | Self::GroupRepeated => Value::Whole(3),
            Self::FuzzyMinScore => Value::Number(0.5),
            Self::GeneratedMarkers => list(&crate::search::DEFAULT_MARKERS),
            Self::KindsConfig => list(&crate::search::DEFAULT_CONFIG),
            Self::KindsDoc => list(&crate::search::DEFAULT_DOC),
            Self::KindsTest => list(&crate::search::DEFAULT_TEST),
            Self::KindsGenerated => list(&crate::search::DEFAULT_GENERATED),
            Self::GeneratedMarkerLines => Value::Whole(crate::search::DEFAULT_MARKER_LINES),
            Self::MaxDepth | Self::FallbackEncoding | Self::Before | Self::After => Value::NotSet,
            Self::FollowSymlinks
            | Self::Hidden
            | Self::BlankContext
            | Self::PathFragments
            | Self::Meta => Value::Bool(false),
            Self::IgnoreFiles => list(&[".gitignore", ".ignore", ".aiignore"]),
            Self::GitExcludes | Self::SkipGenerated | Self::Dedent | Self::GroupDirs => {
                Value::Bool(true)
            }
            Self::IgnorePaths | Self::Exclude => list(&[]),
            Self::Markers => list(&["⟦", "⟧"]),
            Self::PathCase => Value::Text("native".to_string()),
            Self::Binary => Value::Text("skip".to_string()),
            Self::MaxFileSize => Value::Text("10M".to_string()),
            Self::Encoding | Self::Base | Self::Highlight | Self::Funnel => {
                Value::Text("auto".to_string())
            }
            Self::Archives => Value::Text("none".to_string()),
            Self::Format => Value::Text("text".to_string()),
            Self::PathStyle => Value::Text("relative".to_string()),
            Self::Sort => Value::Text("relevance".to_string()),
            Self::Detail => Value::Text("lines".to_string()),
            Self::MaxArchiveSize => Value::Text("100M".to_string()),
        }
    }

    /// The section of the config file.
    #[must_use]
    pub fn section(self) -> &'static str {
        self.spec().0
    }

    /// The name in its section.
    #[must_use]
    pub fn name(self) -> &'static str {
        self.spec().1
    }

    /// The type of the values.
    #[must_use]
    pub fn ty(self) -> Type {
        self.spec().2
    }

    /// A value of the spec which this server version doesn't implement yet: the feature for `not_implemented`.
    #[must_use]
    pub fn unimplemented(self, value: &Value) -> Option<String> {
        UNIMPLEMENTED
            .iter()
            .find(|(key, v)| *key == self && matches!(value, Value::Text(t) if t == v))
            .map(|(_, v)| format!("the value '{v}' of '{}.{}'", self.section(), self.name()))
    }

    /// The key `name` of `section`, if it is implemented.
    #[must_use]
    pub fn find(section: &str, name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|k| k.section() == section && k.name() == name)
    }
}

/// The values of the spec which this server version doesn't implement yet.
const UNIMPLEMENTED: [(Key, &str); 1] = [(Key::Detail, "facets")];

/// The type of the values of a key: how a value is checked and what an invalid one is told.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    /// `true` or `false`.
    Bool,
    /// A whole number; the note says what a special value means, e.g. `0: off`.
    Whole(Option<&'static str>),
    /// A whole number, or not set.
    OptWhole,
    /// A whole number and `ms`, `s` or `m`, as written.
    Duration,
    /// A list of strings; what they are, e.g. `file names`.
    List(&'static str),
    /// A non-empty list of strings; what they are, e.g. `paths`.
    NonEmptyList(&'static str),
    /// A list of wildcards, matched like `path ~`.
    Wildcards,
    /// One of a fixed set of strings.
    Choice(&'static [&'static str]),
    /// A size like the values of the `size` target, as written: `"10M"`.
    Size,
    /// `"auto"` or a WHATWG encoding label, as written.
    Encoding,
    /// A WHATWG encoding label, as written, or not set.
    OptEncoding,
    /// A number from 0 to 1.
    Score,
    /// A whole number of at least the given one.
    AtLeast(usize),
    /// `auto` or a branch, tag or commit: a non-empty name which doesn't start with `-`.
    GitBase,
    /// Two non-empty strings: the opening and the closing marker.
    Markers,
}

impl Type {
    /// What a value of the config file or of `update_config` must be: the end of `invalid value for '...': ...` and
    /// the hint of `invalid_value`.
    #[must_use]
    pub fn expected(self) -> String {
        match self {
            Self::Bool => "true or false is expected".to_string(),
            Self::Whole(None) | Self::OptWhole => "a whole number is expected".to_string(),
            Self::Whole(Some(note)) => format!("a whole number is expected ({note})"),
            Self::Duration => "a duration like \"30s\" is expected (units ms, s, m)".to_string(),
            Self::List(what) => format!("a list of {what} is expected"),
            Self::NonEmptyList(what) => format!("a non-empty list of {what} is expected"),
            Self::Wildcards => "a list of wildcards is expected".to_string(),
            Self::Choice(values) => format!("expected one of: {}", values.join(", ")),
            Self::Size => "a size like \"10M\" is expected (units B, K, M, G)".to_string(),
            Self::Encoding => {
                "\"auto\" or an encoding name like \"windows-1252\" is expected".to_string()
            }
            Self::OptEncoding => "an encoding name like \"windows-1251\" is expected".to_string(),
            Self::Score => "a number from 0 to 1 is expected".to_string(),
            Self::AtLeast(min) => format!("a whole number of at least {min} is expected"),
            Self::GitBase => "\"auto\" or a branch, tag or commit is expected".to_string(),
            Self::Markers => "two non-empty strings are expected".to_string(),
        }
    }

    /// What a value of a request must be: the end of `'<field>.<key>' must be ...`.
    #[must_use]
    pub fn must_be(self) -> String {
        match self {
            Self::Bool => "true or false".to_string(),
            Self::Whole(_) | Self::OptWhole => "a whole number".to_string(),
            Self::Duration => "a duration like \"10s\" (units ms, s, m)".to_string(),
            Self::List(_) | Self::Wildcards => "a list of strings".to_string(),
            Self::NonEmptyList(_) => "a non-empty list of strings".to_string(),
            Self::Choice(values) => format!("one of: {}", values.join(", ")),
            Self::Size => "a size like \"10M\" (units B, K, M, G)".to_string(),
            Self::Encoding => "\"auto\" or an encoding name like \"windows-1252\"".to_string(),
            Self::OptEncoding => "an encoding name like \"windows-1251\"".to_string(),
            Self::Score => "a number from 0 to 1".to_string(),
            Self::AtLeast(min) => format!("a whole number of at least {min}"),
            Self::GitBase => "\"auto\" or a branch, tag or commit".to_string(),
            Self::Markers => "two non-empty strings".to_string(),
        }
    }

    /// The value of a TOML item of the config file or of `update_config`; the error is what the value must be, the
    /// end of `invalid value for '...': ...` and the hint of `invalid_value`.
    ///
    /// # Errors
    ///
    /// The item doesn't have this type or is an invalid value of it.
    pub fn check_toml(self, item: &toml_edit::Item) -> Result<Value, String> {
        let value = self.parse_toml(item).ok_or_else(|| self.expected())?;
        self.validate(&value).map_err(|p| p.expected())?;
        Ok(value)
    }

    /// The value of a field of a request; `name` is the field as written in errors, e.g. `traversal.exclude`.
    ///
    /// # Errors
    ///
    /// `invalid_option`: the value doesn't have this type or is an invalid value of it.
    pub fn check_json(self, name: &str, value: &serde_json::Value) -> Result<Value, Report> {
        let checked = self.parse_json(value).ok_or_else(|| {
            Report::new(
                ErrorCode::InvalidOption,
                format!("'{name}' must be {}", self.must_be()),
            )
        })?;
        self.validate(&checked).map_err(|p| p.report(name))?;
        Ok(checked)
    }

    /// Checks a value of the right shape.
    fn validate(self, value: &Value) -> Result<(), Problem> {
        match (self, value) {
            (Self::Wildcards, Value::List(items)) => {
                for item in items {
                    if let Err(e) = wildcard::compile(&nfc(item), false) {
                        return Err(Problem::Wildcard {
                            wildcard: item.clone(),
                            hint: e.0,
                        });
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// The value of a TOML item, if it has the shape of this type.
    fn parse_toml(self, item: &toml_edit::Item) -> Option<Value> {
        match self {
            Self::Bool => item.as_bool().map(Value::Bool),
            Self::Whole(_) | Self::OptWhole => item
                .as_integer()
                .and_then(|n| usize::try_from(n).ok())
                .map(Value::Whole),
            Self::AtLeast(min) => item
                .as_integer()
                .and_then(|n| usize::try_from(n).ok())
                .filter(|n| *n >= min)
                .map(Value::Whole),
            Self::Duration => item
                .as_str()
                .filter(|t| super::parse_duration(t).is_some())
                .map(|t| Value::Text(t.to_string())),
            Self::List(_) | Self::Wildcards => toml_strings(item).map(Value::List),
            Self::NonEmptyList(_) => toml_strings(item)
                .filter(|items| !items.is_empty())
                .map(Value::List),
            Self::Markers => toml_strings(item)
                .filter(|m| are_markers(m))
                .map(Value::List),
            Self::Choice(values) => item
                .as_str()
                .filter(|v| values.contains(v))
                .map(|v| Value::Text(v.to_string())),
            Self::Size => item
                .as_str()
                .filter(|v| parse_size(v).is_some())
                .map(|v| Value::Text(v.to_string())),
            Self::Encoding => item
                .as_str()
                .filter(|v| *v == "auto" || encoding(v).is_some())
                .map(|v| Value::Text(v.to_string())),
            Self::OptEncoding => item
                .as_str()
                .filter(|v| encoding(v).is_some())
                .map(|v| Value::Text(v.to_string())),
            Self::GitBase => item
                .as_str()
                .filter(|v| is_git_base(v))
                .map(|v| Value::Text(v.to_string())),
            #[allow(clippy::cast_precision_loss)] // only 0 and 1 are whole numbers in the range
            Self::Score => item
                .as_float()
                .or_else(|| item.as_integer().map(|n| n as f64))
                .filter(|n| (0.0..=1.0).contains(n))
                .map(Value::Number),
        }
    }

    /// The value of a JSON value of a request, if it has the shape of this type.
    fn parse_json(self, value: &serde_json::Value) -> Option<Value> {
        match self {
            Self::Bool => value.as_bool().map(Value::Bool),
            Self::Whole(_) | Self::OptWhole => value
                .as_u64()
                .and_then(|n| usize::try_from(n).ok())
                .map(Value::Whole),
            Self::AtLeast(min) => value
                .as_u64()
                .and_then(|n| usize::try_from(n).ok())
                .filter(|n| *n >= min)
                .map(Value::Whole),
            Self::Duration => value
                .as_str()
                .filter(|t| super::parse_duration(t).is_some())
                .map(|t| Value::Text(t.to_string())),
            Self::List(_) | Self::Wildcards => json_strings(value).map(Value::List),
            Self::NonEmptyList(_) => json_strings(value)
                .filter(|items| !items.is_empty())
                .map(Value::List),
            Self::Markers => json_strings(value)
                .filter(|m| are_markers(m))
                .map(Value::List),
            Self::Choice(values) => value
                .as_str()
                .filter(|v| values.contains(v))
                .map(|v| Value::Text(v.to_string())),
            Self::Size => value
                .as_str()
                .filter(|v| parse_size(v).is_some())
                .map(|v| Value::Text(v.to_string())),
            Self::Encoding => value
                .as_str()
                .filter(|v| *v == "auto" || encoding(v).is_some())
                .map(|v| Value::Text(v.to_string())),
            Self::OptEncoding => value
                .as_str()
                .filter(|v| encoding(v).is_some())
                .map(|v| Value::Text(v.to_string())),
            Self::GitBase => value
                .as_str()
                .filter(|v| is_git_base(v))
                .map(|v| Value::Text(v.to_string())),
            Self::Score => value
                .as_f64()
                .filter(|n| (0.0..=1.0).contains(n))
                .map(Value::Number),
        }
    }
}

/// Whether the strings are an opening and a closing marker: two non-empty strings.
fn are_markers(items: &[String]) -> bool {
    items.len() == 2 && items.iter().all(|m| !m.is_empty())
}

/// What is wrong with a value of the right shape.
enum Problem {
    /// An invalid wildcard and the hint of `invalid_wildcard`.
    Wildcard { wildcard: String, hint: String },
}

impl Problem {
    /// The problem in the config file and in `update_config`.
    fn expected(&self) -> String {
        match self {
            Self::Wildcard { wildcard, hint } => format!("invalid wildcard '{wildcard}': {hint}"),
        }
    }

    /// The problem in the field `name` of a request.
    fn report(&self, name: &str) -> Report {
        match self {
            Self::Wildcard { wildcard, hint } => Report::new(
                ErrorCode::InvalidOption,
                format!("invalid wildcard '{wildcard}' in '{name}'"),
            )
            .hint(hint.clone()),
        }
    }
}

/// The WHATWG encoding of a label, any case; not the `replacement` encoding, which decodes nothing.
#[must_use]
pub fn encoding(label: &str) -> Option<&'static encoding_rs::Encoding> {
    encoding_rs::Encoding::for_label(label.as_bytes()).filter(|e| *e != encoding_rs::REPLACEMENT)
}

/// Whether a name can be a git base: not empty and not an option of `git`.
fn is_git_base(name: &str) -> bool {
    !name.is_empty() && !name.starts_with('-')
}

/// A TOML array of strings.
pub(super) fn toml_strings(item: &toml_edit::Item) -> Option<Vec<String>> {
    item.as_array()?
        .iter()
        .map(|v| v.as_str().map(str::to_string))
        .collect()
}

/// A JSON array of strings.
fn json_strings(value: &serde_json::Value) -> Option<Vec<String>> {
    value
        .as_array()?
        .iter()
        .map(|v| v.as_str().map(str::to_string))
        .collect()
}

/// A checked value of a key; its variant always matches the type of the key.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Whole(usize),
    /// A value which is not set, e.g. `max_depth` without a limit.
    NotSet,
    Text(String),
    List(Vec<String>),
    Number(f64),
}

impl Value {
    /// The value as TOML; `None` when it is not set.
    #[must_use]
    pub fn to_toml(&self) -> Option<String> {
        Some(match self {
            Self::Bool(b) => b.to_string(),
            Self::Whole(n) => n.to_string(),
            Self::NotSet => return None,
            Self::Text(text) => basic_string(text),
            Self::Number(n) => n.to_string(),
            Self::List(items) => format!(
                "[{}]",
                items
                    .iter()
                    .map(|i| super::toml_string(i))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        })
    }
}

/// A TOML basic string (`"..."`).
fn basic_string(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                let _ = write!(out, "\\u{:04X}", u32::from(c));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_in_the_order_of_the_spec() {
        let mut sorted = Key::ALL;
        sorted.sort();
        assert_eq!(sorted, Key::ALL);
        for key in Key::ALL {
            let keys = super::super::section_keys(key.section());
            assert!(keys.contains(&key.name()), "{key:?}");
            assert_eq!(Key::find(key.section(), key.name()), Some(key));
        }
        let positions: Vec<(usize, usize)> = Key::ALL
            .iter()
            .map(|k| {
                let sections = super::super::section_names();
                let section = sections.iter().position(|s| *s == k.section()).unwrap();
                let keys = super::super::section_keys(k.section());
                (section, keys.iter().position(|n| *n == k.name()).unwrap())
            })
            .collect();
        let mut ordered = positions.clone();
        ordered.sort_unstable();
        assert_eq!(positions, ordered);
    }

    #[test]
    fn values_as_toml() {
        assert_eq!(Value::Bool(true).to_toml().unwrap(), "true");
        assert_eq!(Value::Whole(3).to_toml().unwrap(), "3");
        assert_eq!(Value::NotSet.to_toml(), None);
        assert_eq!(
            Value::Text("a\"b\\c".into()).to_toml().unwrap(),
            r#""a\"b\\c""#
        );
        assert_eq!(
            Value::List(vec!["a".into(), "it's".into()])
                .to_toml()
                .unwrap(),
            "['a', \"it's\"]"
        );
        assert_eq!(Value::List(vec![]).to_toml().unwrap(), "[]");
    }

    #[test]
    fn types_check_toml_and_json() {
        use serde_json::json;
        let item = |text: &str| {
            let doc = toml_edit::Document::parse(format!("x = {text}")).unwrap();
            doc.as_table()["x"].clone()
        };
        assert_eq!(
            Type::Bool.parse_toml(&item("true")),
            Some(Value::Bool(true))
        );
        assert_eq!(Type::Bool.parse_toml(&item("1")), None);
        assert_eq!(Type::OptWhole.parse_toml(&item("-1")), None);
        assert_eq!(
            Type::Duration.parse_toml(&item("\"2m\"")),
            Some(Value::Text("2m".into()))
        );
        assert_eq!(Type::Duration.parse_toml(&item("\"2h\"")), None);
        assert_eq!(Type::NonEmptyList("paths").parse_toml(&item("[]")), None);
        assert_eq!(
            Type::List("paths").parse_toml(&item("[]")),
            Some(Value::List(vec![]))
        );
        assert_eq!(
            Type::Whole(None).parse_json(&json!(2)),
            Some(Value::Whole(2))
        );
        assert_eq!(Type::Whole(None).parse_json(&json!(1.5)), None);
        assert_eq!(Type::List("paths").parse_json(&json!(["a", 1])), None);
        assert_eq!(
            Type::Whole(Some("0: off")).expected(),
            "a whole number is expected (0: off)"
        );
        assert_eq!(Type::List("paths").must_be(), "a list of strings");
        let case = Type::Choice(&["native", "sensitive"]);
        assert_eq!(
            case.check_toml(&item("'sensitive'")),
            Ok(Value::Text("sensitive".into()))
        );
        assert_eq!(
            case.check_toml(&item("'Native'")),
            Err("expected one of: native, sensitive".into())
        );
        assert_eq!(case.must_be(), "one of: native, sensitive");
        assert_eq!(
            Type::Wildcards.check_toml(&item("['**/a/**', '[a-']")),
            Err("invalid wildcard '[a-': unclosed '['".into())
        );
        assert_eq!(
            Type::Wildcards.check_toml(&item("'a'")),
            Err("a list of wildcards is expected".into())
        );
        assert_eq!(
            Type::Wildcards
                .check_json("traversal.exclude", &json!(["{a"]))
                .unwrap_err()
                .to_text(),
            "error[invalid_option]: invalid wildcard '{a' in 'traversal.exclude'\n  hint: unclosed '{'\n"
        );
    }

    #[test]
    fn markers_are_two_strings() {
        use serde_json::json;
        let item = |text: &str| {
            let doc = toml_edit::Document::parse(format!("x = {text}")).unwrap();
            doc.as_table()["x"].clone()
        };
        assert_eq!(
            Type::Markers.check_toml(&item("['<<', '>>']")),
            Ok(Value::List(vec!["<<".into(), ">>".into()]))
        );
        for bad in ["['<']", "['<', '>', '!']", "['', '>']", "'<>'"] {
            assert_eq!(
                Type::Markers.check_toml(&item(bad)),
                Err("two non-empty strings are expected".into()),
                "{bad}"
            );
        }
        assert_eq!(Type::Markers.parse_json(&json!(["<", ""])), None);
        assert_eq!(Type::Markers.must_be(), "two non-empty strings");
        assert_eq!(
            Key::Markers.default_value(),
            Value::List(vec!["⟦".into(), "⟧".into()])
        );
    }
}
