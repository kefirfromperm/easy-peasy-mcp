//! The changeable keys of the configuration: their section, name, type and default (CONFIG.md#sections).

use std::fmt::Write as _;

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
}

impl Key {
    /// All keys in the order of the spec.
    pub const ALL: [Key; 11] = [
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
        }
    }

    /// The built-in default.
    #[must_use]
    pub fn default_value(self) -> Value {
        let list = |items: &[&str]| Value::List(items.iter().map(ToString::to_string).collect());
        match self {
            Self::DefaultDirs => list(&["."]),
            Self::Timeout => Value::Text("30s".to_string()),
            Self::Threads | Self::MinDepth => Value::Whole(0),
            Self::ResultCache => Value::Whole(50),
            Self::MaxDepth => Value::NotSet,
            Self::FollowSymlinks | Self::Hidden => Value::Bool(false),
            Self::IgnoreFiles => list(&[".gitignore", ".ignore", ".aiignore"]),
            Self::GitExcludes => Value::Bool(true),
            Self::IgnorePaths => list(&[]),
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

    /// The key `name` of `section`, if it is implemented.
    #[must_use]
    pub fn find(section: &str, name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|k| k.section() == section && k.name() == name)
    }
}

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
        }
    }

    /// What a value of a request must be: the end of `'<field>.<key>' must be ...`.
    #[must_use]
    pub fn must_be(self) -> &'static str {
        match self {
            Self::Bool => "true or false",
            Self::Whole(_) | Self::OptWhole => "a whole number",
            Self::Duration => "a duration like \"10s\" (units ms, s, m)",
            Self::List(_) => "a list of strings",
            Self::NonEmptyList(_) => "a non-empty list of strings",
        }
    }

    /// The value of a TOML item, if it has this type.
    pub fn from_toml(self, item: &toml_edit::Item) -> Option<Value> {
        match self {
            Self::Bool => item.as_bool().map(Value::Bool),
            Self::Whole(_) | Self::OptWhole => item
                .as_integer()
                .and_then(|n| usize::try_from(n).ok())
                .map(Value::Whole),
            Self::Duration => item
                .as_str()
                .filter(|t| super::parse_duration(t).is_some())
                .map(|t| Value::Text(t.to_string())),
            Self::List(_) => toml_strings(item).map(Value::List),
            Self::NonEmptyList(_) => toml_strings(item)
                .filter(|items| !items.is_empty())
                .map(Value::List),
        }
    }

    /// The value of a JSON value of a request, if it has this type.
    pub fn from_json(self, value: &serde_json::Value) -> Option<Value> {
        match self {
            Self::Bool => value.as_bool().map(Value::Bool),
            Self::Whole(_) | Self::OptWhole => value
                .as_u64()
                .and_then(|n| usize::try_from(n).ok())
                .map(Value::Whole),
            Self::Duration => value
                .as_str()
                .filter(|t| super::parse_duration(t).is_some())
                .map(|t| Value::Text(t.to_string())),
            Self::List(_) => json_strings(value).map(Value::List),
            Self::NonEmptyList(_) => json_strings(value)
                .filter(|items| !items.is_empty())
                .map(Value::List),
        }
    }
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
        assert_eq!(Type::Bool.from_toml(&item("true")), Some(Value::Bool(true)));
        assert_eq!(Type::Bool.from_toml(&item("1")), None);
        assert_eq!(Type::OptWhole.from_toml(&item("-1")), None);
        assert_eq!(
            Type::Duration.from_toml(&item("\"2m\"")),
            Some(Value::Text("2m".into()))
        );
        assert_eq!(Type::Duration.from_toml(&item("\"2h\"")), None);
        assert_eq!(Type::NonEmptyList("paths").from_toml(&item("[]")), None);
        assert_eq!(
            Type::List("paths").from_toml(&item("[]")),
            Some(Value::List(vec![]))
        );
        assert_eq!(
            Type::Whole(None).from_json(&json!(2)),
            Some(Value::Whole(2))
        );
        assert_eq!(Type::Whole(None).from_json(&json!(1.5)), None);
        assert_eq!(Type::List("paths").from_json(&json!(["a", 1])), None);
        assert_eq!(
            Type::Whole(Some("0: off")).expected(),
            "a whole number is expected (0: off)"
        );
        assert_eq!(Type::List("paths").must_be(), "a list of strings");
    }
}
