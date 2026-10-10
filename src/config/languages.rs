//! The languages of the configuration: one section `[languages.<name>]` per language, which replaces a language as a
//! whole (CONFIG.md#languages).

use std::fmt::Write as _;

use super::{Source, key_line, toml_string};
use crate::error::{ErrorCode, Report};
use crate::languages::Language;
use crate::query::did_you_mean;

/// The keys of a language, in the order of the spec.
pub const KEYS: [&str; 8] = [
    "extensions",
    "names",
    "shebangs",
    "line_comments",
    "block_comments",
    "nested_block_comments",
    "strings",
    "source",
];

/// The keys this server version implements.
const IMPLEMENTED: [&str; 7] = [
    "extensions",
    "names",
    "shebangs",
    "line_comments",
    "block_comments",
    "nested_block_comments",
    "strings",
];

/// What every language name must be.
pub const NAME_EXPECTED: &str = "a letter, then letters, digits, _ and - are expected";

/// A language of the settings and where it comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageSetting {
    pub language: Language,
    pub source: Source,
}

/// What is wrong with a language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Problem {
    /// The name is not a word.
    Name(String),
    /// The language is not a table (an object in a request).
    NotATable(String),
    /// A key which isn't one of a language, and the closest one.
    UnknownKey {
        language: String,
        key: String,
        close: Option<&'static str>,
    },
    /// A key of the spec which isn't implemented yet.
    NotImplemented { language: String, key: String },
    /// A value of a wrong type, and what it must be.
    Value {
        language: String,
        key: &'static str,
        expected: &'static str,
    },
    /// An extension or a name which is also in another language.
    Conflict {
        language: String,
        key: &'static str,
        item: String,
        other: String,
    },
}

impl Problem {
    /// The full key of the problem, e.g. `languages.rust.extensions`.
    #[must_use]
    pub fn key(&self) -> String {
        match self {
            Self::Name(name) | Self::NotATable(name) => format!("languages.{name}"),
            Self::UnknownKey { language, key, .. } | Self::NotImplemented { language, key } => {
                format!("languages.{language}.{key}")
            }
            Self::Value { language, key, .. } | Self::Conflict { language, key, .. } => {
                format!("languages.{language}.{key}")
            }
        }
    }

    /// What is wrong with the value of a key: the conflict, or what the value must be.
    #[must_use]
    pub fn expected(&self) -> String {
        match self {
            Self::Value { expected, .. } => format!("{expected} is expected"),
            Self::Conflict {
                key, item, other, ..
            } => format!(
                "the {} '{item}' is also in 'languages.{other}'",
                singular(key)
            ),
            Self::Name(_) => NAME_EXPECTED.to_string(),
            _ => String::new(),
        }
    }
}

impl Problem {
    /// The message of the problem in the config file.
    #[must_use]
    pub fn file_message(&self) -> String {
        match self {
            Self::Name(name) => format!("invalid language name '{name}': {NAME_EXPECTED}"),
            Self::NotATable(name) => format!("'languages.{name}' must be a section"),
            Self::UnknownKey {
                language, close, ..
            } => match close {
                Some(close) => format!(
                    "unknown key '{}', did you mean 'languages.{language}.{close}'?",
                    self.key()
                ),
                None => format!("unknown key '{}'", self.key()),
            },
            Self::NotImplemented { .. } => {
                format!("the key '{}' is not implemented yet", self.key())
            }
            Self::Value { .. } | Self::Conflict { .. } => {
                format!("invalid value for '{}': {}", self.key(), self.expected())
            }
        }
    }

    /// The error of the problem in `update_config`.
    #[must_use]
    pub fn update_report(&self) -> Report {
        match self {
            Self::Name(name) => Report::new(
                ErrorCode::InvalidValue,
                format!("invalid language name '{name}'"),
            )
            .hint(NAME_EXPECTED),
            Self::NotATable(name) => Report::new(
                ErrorCode::UnknownKey,
                format!("'languages.{name}' must be a section"),
            ),
            Self::UnknownKey {
                language, close, ..
            } => {
                let report = Report::new(
                    ErrorCode::UnknownKey,
                    format!("unknown key '{}'", self.key()),
                );
                match close {
                    Some(close) => {
                        report.hint(format!("did you mean 'languages.{language}.{close}'?"))
                    }
                    None => report,
                }
            }
            Self::NotImplemented { .. } => {
                Report::not_implemented(&format!("the key '{}'", self.key()))
            }
            Self::Value { .. } | Self::Conflict { .. } => Report::new(
                ErrorCode::InvalidValue,
                format!("invalid value for '{}'", self.key()),
            )
            .hint(self.expected()),
        }
    }

    /// The error of the problem in a request.
    #[must_use]
    pub fn request_report(&self) -> Report {
        match self {
            Self::Name(name) => Report::new(
                ErrorCode::InvalidOption,
                format!("invalid language name '{name}'"),
            )
            .hint(NAME_EXPECTED),
            Self::NotATable(name) => Report::new(
                ErrorCode::InvalidOption,
                format!("'languages.{name}' must be an object"),
            ),
            Self::UnknownKey { close, .. } => {
                let expected = format!("expected one of: {}", KEYS.join(", "));
                let hint = match close {
                    Some(close) => format!("did you mean '{close}'? {expected}"),
                    None => expected,
                };
                Report::new(
                    ErrorCode::InvalidOption,
                    format!("unknown field '{}'", self.key()),
                )
                .hint(hint)
            }
            Self::NotImplemented { .. } => {
                Report::not_implemented(&format!("the field '{}'", self.key()))
            }
            Self::Value { expected, .. } => Report::new(
                ErrorCode::InvalidOption,
                format!("'{}' must be {expected}", self.key()),
            ),
            Self::Conflict {
                key, item, other, ..
            } => Report::new(
                ErrorCode::InvalidOption,
                format!(
                    "the {} '{item}' of '{}' is also in 'languages.{other}'",
                    singular(key),
                    self.key()
                ),
            ),
        }
    }
}

/// `extension` of `extensions`.
fn singular(key: &str) -> &str {
    key.strip_suffix('s').unwrap_or(key)
}

/// Whether `name` is a language name: a letter, then letters, digits, `_` and `-`.
pub fn is_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(char::is_alphabetic)
        && chars.all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

/// A new language with the given name: no extensions, names, shebangs or comments, code.
pub fn new_language(name: &str) -> Language {
    Language {
        name: name.to_lowercase(),
        extensions: Vec::new(),
        names: Vec::new(),
        shebangs: Vec::new(),
        source: true,
        line_comments: Vec::new(),
        block_comments: Vec::new(),
        nested_block_comments: false,
        strings: Vec::new(),
    }
}

/// The type of a key of a language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeyType {
    Strings,
    Pairs,
    Bool,
}

impl KeyType {
    /// The type of a key of a language.
    fn of(key: &str) -> Self {
        match key {
            "block_comments" => Self::Pairs,
            "nested_block_comments" | "source" => Self::Bool,
            _ => Self::Strings,
        }
    }

    /// What a value of this type must be.
    fn expected(self) -> &'static str {
        match self {
            Self::Strings => "a list of strings",
            Self::Pairs => "a list of [start, end] pairs of strings",
            Self::Bool => "true or false",
        }
    }
}

/// A value of a key of a language in the config file or in a request.
#[derive(Clone, Copy)]
enum Raw<'a> {
    Toml(&'a toml_edit::Item),
    Json(&'a serde_json::Value),
}

impl Raw<'_> {
    fn strings(self) -> Option<Vec<String>> {
        match self {
            Self::Toml(item) => super::keys::toml_strings(item),
            Self::Json(value) => value
                .as_array()?
                .iter()
                .map(|v| v.as_str().map(str::to_string))
                .collect(),
        }
    }

    fn pairs(self) -> Option<Vec<(String, String)>> {
        let pair = |items: Vec<Option<&str>>| match items.as_slice() {
            [Some(a), Some(b)] => Some(((*a).to_string(), (*b).to_string())),
            _ => None,
        };
        match self {
            Self::Toml(item) => item
                .as_array()?
                .iter()
                .map(|v| pair(v.as_array()?.iter().map(toml_edit::Value::as_str).collect()))
                .collect(),
            Self::Json(value) => value
                .as_array()?
                .iter()
                .map(|v| {
                    pair(
                        v.as_array()?
                            .iter()
                            .map(serde_json::Value::as_str)
                            .collect(),
                    )
                })
                .collect(),
        }
    }

    fn boolean(self) -> Option<bool> {
        match self {
            Self::Toml(item) => item.as_bool(),
            Self::Json(value) => value.as_bool(),
        }
    }
}

/// Checks a key of a language and sets it: unknown, not implemented, of a wrong type.
fn set_key(language: &mut Language, key: &str, raw: Raw<'_>) -> Result<(), Problem> {
    let Some(key) = KEYS.into_iter().find(|k| *k == key) else {
        return Err(Problem::UnknownKey {
            language: language.name.clone(),
            key: key.to_string(),
            close: did_you_mean(key, &KEYS),
        });
    };
    if !IMPLEMENTED.contains(&key) {
        return Err(Problem::NotImplemented {
            language: language.name.clone(),
            key: key.to_string(),
        });
    }
    let ty = KeyType::of(key);
    let invalid = || Problem::Value {
        language: language.name.clone(),
        key,
        expected: ty.expected(),
    };
    match ty {
        KeyType::Strings => {
            let items = raw.strings().ok_or_else(invalid)?;
            match key {
                "extensions" => language.extensions = items,
                "names" => language.names = items,
                "shebangs" => language.shebangs = items,
                "line_comments" => language.line_comments = items,
                _ => language.strings = items,
            }
        }
        KeyType::Pairs => language.block_comments = raw.pairs().ok_or_else(invalid)?,
        KeyType::Bool => {
            let value = raw.boolean().ok_or_else(invalid)?;
            if key == "source" {
                language.source = value;
            } else {
                language.nested_block_comments = value;
            }
        }
    }
    Ok(())
}

/// A language of a TOML table (the config file, `update_config`).
///
/// # Errors
///
/// The problem of the language, with the byte offset of the key it is about, if any.
pub fn from_toml(name: &str, item: &toml_edit::Item) -> Result<Language, (Problem, Option<usize>)> {
    if !is_name(name) {
        return Err((Problem::Name(name.to_string()), None));
    }
    let Some(table) = item.as_table_like() else {
        return Err((Problem::NotATable(name.to_lowercase()), None));
    };
    let mut language = new_language(name);
    for (key, value) in table.iter() {
        let at = table
            .get_key_value(key)
            .and_then(|(k, _)| k.span())
            .map(|s| s.start);
        set_key(&mut language, key, Raw::Toml(value)).map_err(|p| (p, at))?;
    }
    Ok(language)
}

/// A language of a request.
///
/// # Errors
///
/// The problem of the language.
pub fn from_json(name: &str, value: &serde_json::Value) -> Result<Language, Problem> {
    if !is_name(name) {
        return Err(Problem::Name(name.to_string()));
    }
    let Some(object) = value.as_object() else {
        return Err(Problem::NotATable(name.to_lowercase()));
    };
    let mut language = new_language(name);
    for (key, value) in object {
        set_key(&mut language, key, Raw::Json(value))?;
    }
    Ok(language)
}

/// The languages with the changes: a changed language replaces the one with the same name or is added after the
/// others.
///
/// # Errors
///
/// An extension or a name of a changed language which is also in another language.
pub fn merge(
    base: &[LanguageSetting],
    changes: Vec<LanguageSetting>,
) -> Result<Vec<LanguageSetting>, Problem> {
    let mut languages = base.to_vec();
    let mut names = Vec::new();
    for change in changes {
        names.push(change.language.name.clone());
        match languages
            .iter_mut()
            .find(|l| l.language.name == change.language.name)
        {
            Some(existing) => *existing = change,
            None => languages.push(change),
        }
    }
    for name in &names {
        let language = &languages
            .iter()
            .find(|l| &l.language.name == name)
            .expect("a changed language is in the table")
            .language;
        for (key, items) in [
            ("extensions", &language.extensions),
            ("names", &language.names),
        ] {
            for item in items {
                let lower = item.to_lowercase();
                let other = languages.iter().map(|l| &l.language).find(|other| {
                    other.name != *name
                        && match key {
                            "extensions" => &other.extensions,
                            _ => &other.names,
                        }
                        .iter()
                        .any(|o| o.to_lowercase() == lower)
                });
                if let Some(other) = other {
                    return Err(Problem::Conflict {
                        language: name.clone(),
                        key,
                        item: item.clone(),
                        other: other.name.clone(),
                    });
                }
            }
        }
    }
    Ok(languages)
}

/// The value of a key of a language as TOML.
fn value(language: &Language, key: &str) -> String {
    let list = |items: &[String]| {
        format!(
            "[{}]",
            items
                .iter()
                .map(|i| toml_string(i))
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    match key {
        "extensions" => list(&language.extensions),
        "names" => list(&language.names),
        "shebangs" => list(&language.shebangs),
        "line_comments" => list(&language.line_comments),
        "strings" => list(&language.strings),
        "block_comments" => format!(
            "[{}]",
            language
                .block_comments
                .iter()
                .map(|(a, b)| format!("[{}, {}]", toml_string(a), toml_string(b)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        "nested_block_comments" => language.nested_block_comments.to_string(),
        _ => language.source.to_string(),
    }
}

/// The languages as the tables of `get_config`, every table after an empty line.
pub fn render(languages: &[LanguageSetting], sources: bool) -> String {
    let mut out = String::new();
    for setting in languages {
        let _ = writeln!(out, "\n[languages.{}]", setting.language.name);
        for key in IMPLEMENTED {
            let line = format!("{key} = {}", value(&setting.language, key));
            if sources {
                let _ = writeln!(out, "{line}    # {}", setting.source);
            } else {
                let _ = writeln!(out, "{line}");
            }
        }
    }
    out
}

/// The lines of `update_config` for the languages whose keys changed: the new value with the old one, `, reset` for
/// a language of `resets` which `sets` doesn't set again.
pub fn changes(
    old: &[LanguageSetting],
    new: &[LanguageSetting],
    resets: &[String],
    sets: &[String],
) -> String {
    let mut names: Vec<&str> = old.iter().map(|l| l.language.name.as_str()).collect();
    for language in new {
        if !names.contains(&language.language.name.as_str()) {
            names.push(&language.language.name);
        }
    }
    let find = |languages: &[LanguageSetting], name: &str| {
        languages
            .iter()
            .find(|l| l.language.name == name)
            .map(|l| l.language.clone())
    };
    let mut out = String::new();
    for name in names {
        let (was, now) = (find(old, name), find(new, name));
        let reset = if resets.iter().any(|r| r == name) && !sets.iter().any(|s| s == name) {
            ", reset"
        } else {
            ""
        };
        let mut header = false;
        for key in IMPLEMENTED {
            let before = was.as_ref().map(|l| value(l, key));
            let after = now.as_ref().map(|l| value(l, key));
            if before == after {
                continue;
            }
            if !header {
                let _ = writeln!(out, "\n[languages.{name}]");
                header = true;
            }
            let line = key_line(key, after.as_deref());
            let before = before.as_deref().unwrap_or("(not set)");
            let _ = writeln!(out, "{line}    # was {before}{reset}");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn builtin() -> Vec<LanguageSetting> {
        crate::languages::builtin_languages()
            .into_iter()
            .map(|language| LanguageSetting {
                language,
                source: Source::Default,
            })
            .collect()
    }

    fn toml(text: &str) -> Result<Language, (Problem, Option<usize>)> {
        let doc = toml_edit::Document::parse(text.to_string()).unwrap();
        let (name, item) = doc.as_table().iter().next().unwrap();
        let table = item.as_table_like().unwrap();
        let (lang, item) = table.iter().next().unwrap();
        assert_eq!(name, "languages");
        from_toml(lang, item)
    }

    #[test]
    fn languages_of_toml_and_json() {
        let tf = toml("[languages.Terraform]\nextensions = ['tf']\n").unwrap();
        assert_eq!(tf.name, "terraform");
        assert_eq!(tf.extensions, ["tf"]);
        assert!(tf.source && tf.line_comments.is_empty());
        assert_eq!(
            toml("[languages.x]\nextensions = 'tf'\n").unwrap_err().0,
            Problem::Value {
                language: "x".into(),
                key: "extensions",
                expected: "a list of strings"
            }
        );
        assert_eq!(
            toml("[languages.x]\nextension = []\n").unwrap_err().0,
            Problem::UnknownKey {
                language: "x".into(),
                key: "extension".into(),
                close: Some("extensions")
            }
        );
        assert!(matches!(
            toml("[languages.x]\nsource = true\n").unwrap_err().0,
            Problem::NotImplemented { .. }
        ));
        assert_eq!(
            toml("languages.rust = 1\n").unwrap_err().0,
            Problem::NotATable("rust".into())
        );
        assert_eq!(
            from_json("c++", &serde_json::json!({})).unwrap_err(),
            Problem::Name("c++".into())
        );
        assert!(is_name("objective-c") && is_name("c_2") && !is_name("2c") && !is_name(""));
    }

    #[test]
    fn merging_replaces_adds_and_finds_conflicts() {
        let base = builtin();
        let mut tf = new_language("terraform");
        tf.extensions = vec!["tf".into()];
        let setting = |language| LanguageSetting {
            language,
            source: Source::Update,
        };
        let merged = merge(&base, vec![setting(tf.clone())]).unwrap();
        assert_eq!(merged.len(), base.len() + 1);
        assert_eq!(merged.last().unwrap().language, tf);
        let mut html = new_language("html");
        html.extensions = vec!["html".into(), "tpl".into()];
        let merged = merge(&base, vec![setting(html.clone())]).unwrap();
        assert_eq!(merged.len(), base.len());
        assert!(merged.iter().any(|l| l.language == html));
        let mut bad = new_language("terraform");
        bad.extensions = vec!["tf".into(), "RS".into()];
        let problem = merge(&base, vec![setting(bad)]).unwrap_err();
        assert_eq!(problem.key(), "languages.terraform.extensions");
        assert_eq!(
            problem.expected(),
            "the extension 'RS' is also in 'languages.rust'"
        );
        // replacing rust frees its extension
        let mut rust = new_language("rust");
        rust.extensions = vec!["rlib".into()];
        let mut other = new_language("other");
        other.extensions = vec!["rs".into()];
        assert!(merge(&base, vec![setting(rust), setting(other)]).is_ok());
    }

    #[test]
    fn tables_and_changes() {
        let mut tf = new_language("terraform");
        tf.extensions = vec!["tf".into(), "it's".into()];
        let languages = vec![LanguageSetting {
            language: tf,
            source: Source::Update,
        }];
        // the keys of a language as far as they are implemented
        assert!(
            render(&languages, true).starts_with(
                "\n[languages.terraform]\nextensions = ['tf', \"it's\"]    # update\n"
            )
        );
        assert!(
            changes(&[], &languages, &[], &["terraform".into()]).starts_with(
                "\n[languages.terraform]\nextensions = ['tf', \"it's\"]    # was (not set)\n"
            )
        );
        assert!(
            changes(&languages, &[], &["terraform".into()], &[]).starts_with(
                "\n[languages.terraform]\n# extensions =  (not set)    # was ['tf', \"it's\"], reset\n"
            )
        );
        assert_eq!(changes(&languages, &languages, &[], &[]), "");
        let mut c = new_language("c");
        c.block_comments = vec![("/*".into(), "*/".into())];
        c.nested_block_comments = true;
        assert_eq!(value(&c, "block_comments"), "[['/*', '*/']]");
        assert_eq!(value(&c, "nested_block_comments"), "true");
        assert_eq!(value(&c, "source"), "true");
    }
}
