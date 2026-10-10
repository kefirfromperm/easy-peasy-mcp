//! The compiled query and its evaluation for one file (QUERY.md#basics, OUTPUT.md#file-conditions).

use regex::Regex;

use super::ast::CondId;
use crate::content::Content;
use crate::git::{GitStatus, GitValue};

/// How a text value is matched: a regex over the NFC form of the value.
#[derive(Debug, Clone)]
pub struct TextMatcher(pub(crate) Regex);

impl TextMatcher {
    /// Whether `value` matches.
    pub fn is_match(&self, value: &str) -> bool {
        self.0.is_match(value)
    }
}

/// The file values a text condition can look at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTarget {
    /// The file name.
    Name,
    /// The path relative to the working directory, `/` as the separator.
    Path,
}

/// The type of an entry (QUERY.md#files-and-directories).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    File,
    Dir,
    Link,
}

impl EntryType {
    /// All types in the order of the spec.
    pub const ALL: [EntryType; 3] = [Self::File, Self::Dir, Self::Link];

    /// The value as written in a query.
    pub fn keyword(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Dir => "dir",
            Self::Link => "link",
        }
    }

    /// The type written as `word`, in any case.
    pub fn from_keyword(word: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|t| t.keyword().eq_ignore_ascii_case(word))
    }
}

/// What a file is for (QUERY.md#kind).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Source,
    Test,
    Config,
    Doc,
    Generated,
    Other,
}

impl Kind {
    /// All kinds in the order of the spec.
    pub const ALL: [Kind; 6] = [
        Self::Source,
        Self::Test,
        Self::Config,
        Self::Doc,
        Self::Generated,
        Self::Other,
    ];

    /// The value as written in a query.
    pub fn keyword(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Test => "test",
            Self::Config => "config",
            Self::Doc => "doc",
            Self::Generated => "generated",
            Self::Other => "other",
        }
    }

    /// The kind written as `word`, in any case.
    pub fn from_keyword(word: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|k| k.keyword().eq_ignore_ascii_case(word))
    }
}

/// Whether and how the content of an entry is known.
#[derive(Debug, Clone, Copy)]
pub enum ContentState<'a> {
    /// Not read yet: content conditions are unknown, so cheap conditions decide first.
    NotRead,
    /// The entry has no content (a directory, a symlink): content conditions are false.
    Absent,
    /// The decoded content.
    Text(&'a Content),
    /// The content is not searched (a binary file): content conditions are unknown.
    Unknown,
}

/// The values of one file which conditions look at.
#[derive(Debug, Clone, Copy)]
pub struct FileValues<'a> {
    /// The NFC file name.
    pub name: &'a str,
    /// The NFC path relative to the working directory, `/` as the separator.
    pub path: &'a str,
    /// The type of the entry; symlinks are not followed.
    pub entry_type: EntryType,
    /// A file of size 0 or a directory without entries; only computed when the query has `empty`.
    pub empty: bool,
    /// The size in bytes; only read when the query has `size`.
    pub size: u64,
    /// The modification time in seconds since the Unix epoch; only read when the query has `modified`.
    pub modified: Option<f64>,
    /// The creation time in seconds since the Unix epoch; `None` when the file system doesn't provide it.
    pub created: Option<f64>,
    /// The last access time in seconds since the Unix epoch; `None` when the file system doesn't provide it.
    pub accessed: Option<f64>,
    /// The language, `unknown` without one; only detected when the query has `lang` or `kind`.
    pub lang: &'a str,
    /// The kind; only computed when the query has `kind`.
    pub kind: Kind,
    /// The git status; `None` outside of a repository, for directories, or without a `git` condition.
    pub git: Option<GitStatus>,
    /// The content for `text` conditions.
    pub content: ContentState<'a>,
}

/// An interval of numbers with inclusive or exclusive ends; `negated` turns it into its complement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    /// The lower end and whether it is included.
    pub min: Option<(f64, bool)>,
    /// The upper end and whether it is included.
    pub max: Option<(f64, bool)>,
    pub negated: bool,
}

impl Bounds {
    /// Whether `v` is in the interval (or outside of it when negated).
    pub fn contains(&self, v: f64) -> bool {
        let above = self
            .min
            .is_none_or(|(m, incl)| if incl { v >= m } else { v > m });
        let below = self
            .max
            .is_none_or(|(m, incl)| if incl { v <= m } else { v < m });
        (above && below) != self.negated
    }
}

/// The numeric values a comparison condition can look at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberTarget {
    /// The size in bytes.
    Size,
    /// The modification time in seconds since the Unix epoch.
    Modified,
    /// The creation time in seconds since the Unix epoch, if the file system provides it.
    Created,
    /// The last access time in seconds since the Unix epoch, if the file system provides it.
    Accessed,
}

/// The compiled query, evaluated for every file; brackets are gone, the precedence is in the tree.
#[derive(Debug, Clone)]
pub enum Filter {
    And(Vec<Filter>),
    Or(Vec<Filter>),
    /// True for an odd number of true parts.
    Xor(Vec<Filter>),
    Not(Box<Filter>),
    /// A `name` or `path` condition; `order` is its position of appearance in the query.
    Text {
        id: CondId,
        order: usize,
        target: TextTarget,
        matcher: TextMatcher,
    },
    /// `type = v`, `type in (...)`: the entry type is one of `values`; `type != v`: `negated`.
    Type {
        id: CondId,
        order: usize,
        values: Vec<EntryType>,
        negated: bool,
    },
    /// `empty`.
    Empty {
        id: CondId,
        order: usize,
    },
    /// `lang = v`, `lang in (...)`: the language is one of `values` (lower case); `lang != v`: `negated`.
    Lang {
        id: CondId,
        order: usize,
        values: Vec<String>,
        negated: bool,
    },
    /// `kind = v`, `kind in (...)`: the kind is one of `values`; `kind != v`: `negated`.
    Kind {
        id: CondId,
        order: usize,
        values: Vec<Kind>,
        negated: bool,
    },
    /// `git = v`, `git in (...)`: the file has one of the statuses; `git != v`: it is in a repository and has none
    /// of them. False outside of a repository.
    Git {
        id: CondId,
        order: usize,
        values: Vec<GitValue>,
        negated: bool,
    },
    /// A `text` condition in file scope: the regex runs over the whole normalized content, or over every line
    /// separately (`=~` without `/m`); `whole_word` for `/w`.
    Content {
        id: CondId,
        order: usize,
        regex: Regex,
        per_line: bool,
        whole_word: bool,
    },
    /// `len` inside `line( ... )`: the number of characters of the line.
    Len {
        id: CondId,
        order: usize,
        bounds: Bounds,
    },
    /// `line( ... )`: true if the inner expression is true for one line.
    Line {
        id: CondId,
        order: usize,
        inner: Box<Filter>,
    },
    /// A comparison of a numeric value: `size > 1M`, `size in 1K..2K`.
    Number {
        id: CondId,
        order: usize,
        target: NumberTarget,
        bounds: Bounds,
    },
}

/// The byte ranges of the matches of `regex` in the normalized content: over the whole text, or per line; with
/// `whole_word` only matches without a letter, digit or `_` right before and after them.
pub fn content_matches(
    regex: &Regex,
    per_line: bool,
    whole_word: bool,
    content: &Content,
) -> Vec<std::ops::Range<usize>> {
    if per_line {
        (0..content.line_count())
            .flat_map(|i| {
                let (start, end) = content.line_range(i);
                find_all(regex, &content.text()[start..end], whole_word)
                    .into_iter()
                    .map(move |r| start + r.start..start + r.end)
            })
            .collect()
    } else {
        find_all(regex, content.text(), whole_word)
    }
}

/// A letter, a digit or `_`: what `/w` doesn't allow next to a match.
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// All matches in `hay`; with `whole_word` a rejected match lets the search go on one character after its start.
pub fn find_all(regex: &Regex, hay: &str, whole_word: bool) -> Vec<std::ops::Range<usize>> {
    if !whole_word {
        return regex.find_iter(hay).map(|m| m.range()).collect();
    }
    let mut out = Vec::new();
    let mut pos = 0;
    while pos <= hay.len() {
        let Some(m) = regex.find_at(hay, pos) else {
            break;
        };
        let before = hay[..m.start()].chars().next_back();
        let after = hay[m.end()..].chars().next();
        if !m.is_empty() && !before.is_some_and(is_word_char) && !after.is_some_and(is_word_char) {
            out.push(m.range());
            pos = m.end();
        } else {
            pos = m.start() + hay[m.start()..].chars().next().map_or(1, char::len_utf8);
        }
    }
    out
}

/// A truth value of three-valued logic (QUERY.md#files-whose-content-is-not-searched).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Truth {
    False,
    Unknown,
    True,
}

impl From<bool> for Truth {
    fn from(b: bool) -> Self {
        if b { Self::True } else { Self::False }
    }
}

impl Truth {
    /// `not`: unknown stays unknown.
    #[must_use]
    pub fn not(self) -> Self {
        match self {
            Self::False => Self::True,
            Self::Unknown => Self::Unknown,
            Self::True => Self::False,
        }
    }

    /// `and`: false wins, then unknown.
    #[must_use]
    pub fn and(self, other: Self) -> Self {
        self.min(other)
    }

    /// `or`: true wins, then unknown.
    #[must_use]
    pub fn or(self, other: Self) -> Self {
        self.max(other)
    }

    /// `xor`: unknown with anything is unknown.
    #[must_use]
    pub fn xor(self, other: Self) -> Self {
        match (self, other) {
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
            (a, b) => Self::from(a != b),
        }
    }

    fn rank(self) -> u8 {
        match self {
            Self::False => 0,
            Self::Unknown => 1,
            Self::True => 2,
        }
    }

    fn min(self, other: Self) -> Self {
        if self.rank() <= other.rank() {
            self
        } else {
            other
        }
    }

    fn max(self, other: Self) -> Self {
        if self.rank() >= other.rank() {
            self
        } else {
            other
        }
    }
}

impl Filter {
    /// The truth value of the query for the file.
    pub fn eval(&self, file: &FileValues<'_>) -> Truth {
        match self {
            Self::And(items) => items.iter().fold(Truth::True, |t, f| t.and(f.eval(file))),
            Self::Or(items) => items.iter().fold(Truth::False, |t, f| t.or(f.eval(file))),
            Self::Xor(items) => items.iter().fold(Truth::False, |t, f| t.xor(f.eval(file))),
            Self::Not(inner) => inner.eval(file).not(),
            Self::Text {
                target, matcher, ..
            } => Truth::from(matcher.is_match(match target {
                TextTarget::Name => file.name,
                TextTarget::Path => file.path,
            })),
            Self::Type {
                values, negated, ..
            } => Truth::from(values.contains(&file.entry_type) != *negated),
            Self::Empty { .. } => Truth::from(file.empty),
            // validation keeps `len` inside line scope, where `eval_line` evaluates it
            Self::Len { .. } => Truth::False,
            Self::Lang {
                values, negated, ..
            } => Truth::from(values.iter().any(|v| v == file.lang) != *negated),
            Self::Kind {
                values, negated, ..
            } => Truth::from(values.contains(&file.kind) != *negated),
            Self::Content {
                regex,
                per_line,
                whole_word,
                ..
            } => match file.content {
                ContentState::NotRead | ContentState::Unknown => Truth::Unknown,
                ContentState::Absent => Truth::False,
                ContentState::Text(content) => {
                    Truth::from(!content_matches(regex, *per_line, *whole_word, content).is_empty())
                }
            },
            Self::Line { inner, .. } => match file.content {
                ContentState::NotRead | ContentState::Unknown => Truth::Unknown,
                ContentState::Absent => Truth::False,
                ContentState::Text(content) => {
                    Truth::from((0..content.line_count()).any(|i| inner.eval_line(content.line(i))))
                }
            },
            Self::Git {
                values, negated, ..
            } => Truth::from(
                file.git
                    .is_some_and(|status| values.iter().any(|v| status.has(*v)) != *negated),
            ),
            #[allow(clippy::cast_precision_loss)] // sizes up to 2^53 bytes are exact
            Self::Number { target, bounds, .. } => {
                let value = match target {
                    NumberTarget::Size => Some(file.size as f64),
                    NumberTarget::Modified => file.modified,
                    NumberTarget::Created => file.created,
                    NumberTarget::Accessed => file.accessed,
                };
                value.map_or(Truth::Unknown, |v| Truth::from(bounds.contains(v)))
            }
        }
    }

    /// Whether the whole query is true for the file: only then the file is returned.
    pub fn matches(&self, file: &FileValues<'_>) -> bool {
        self.eval(file) == Truth::True
    }

    /// The ids of the conditions which made a matching file match, in the order of appearance.
    #[cfg(test)]
    pub fn contributors(&self, file: &FileValues<'_>) -> Vec<CondId> {
        self.contributing_leaves(file)
            .into_iter()
            .filter_map(Filter::id)
            .map(|(_, id)| id.clone())
            .collect()
    }

    /// The conditions which made a matching file match, in the order of appearance: all parts of a true `and`, the
    /// true parts of `or` / `xor`, nothing under `not`.
    pub fn contributing_leaves(&self, file: &FileValues<'_>) -> Vec<&Filter> {
        let mut found = Vec::new();
        self.collect(file, &mut found);
        found.sort_by_key(|f| f.id().map(|(order, _)| order));
        found.dedup_by_key(|f| f.id().map(|(order, _)| order));
        found
    }

    /// The order of appearance and the id of a condition; `None` for logical operators.
    pub fn id(&self) -> Option<(usize, &CondId)> {
        match self {
            Self::And(_) | Self::Or(_) | Self::Xor(_) | Self::Not(_) => None,
            Self::Text { id, order, .. }
            | Self::Type { id, order, .. }
            | Self::Empty { id, order }
            | Self::Lang { id, order, .. }
            | Self::Kind { id, order, .. }
            | Self::Git { id, order, .. }
            | Self::Content { id, order, .. }
            | Self::Line { id, order, .. }
            | Self::Len { id, order, .. }
            | Self::Number { id, order, .. } => Some((*order, id)),
        }
    }

    /// Whether a line scope expression is true for the line.
    pub fn eval_line(&self, line: &str) -> bool {
        match self {
            Self::And(items) => items.iter().all(|f| f.eval_line(line)),
            Self::Or(items) => items.iter().any(|f| f.eval_line(line)),
            Self::Xor(items) => items.iter().filter(|f| f.eval_line(line)).count() % 2 == 1,
            Self::Not(inner) => !inner.eval_line(line),
            Self::Content {
                regex, whole_word, ..
            } => !find_all(regex, line, *whole_word).is_empty(),
            #[allow(clippy::cast_precision_loss)] // line lengths are far below 2^53
            Self::Len { bounds, .. } => bounds.contains(line.chars().count() as f64),
            _ => false,
        }
    }

    /// The conditions which made a line scope expression true for the line, in the order of appearance.
    pub fn contributing_in_line(&self, line: &str) -> Vec<&Filter> {
        let mut found = Vec::new();
        self.collect_line(line, &mut found);
        found.sort_by_key(|f| f.id().map(|(order, _)| order));
        found.dedup_by_key(|f| f.id().map(|(order, _)| order));
        found
    }

    fn collect_line<'f>(&'f self, line: &str, found: &mut Vec<&'f Filter>) {
        match self {
            Self::And(items) => items.iter().for_each(|f| f.collect_line(line, found)),
            Self::Or(items) | Self::Xor(items) => {
                for f in items.iter().filter(|f| f.eval_line(line)) {
                    f.collect_line(line, found);
                }
            }
            Self::Not(_) => {}
            leaf => found.push(leaf),
        }
    }

    fn collect<'f>(&'f self, file: &FileValues<'_>, found: &mut Vec<&'f Filter>) {
        match self {
            Self::And(items) => items.iter().for_each(|f| f.collect(file, found)),
            Self::Or(items) | Self::Xor(items) => {
                for f in items.iter().filter(|f| f.eval(file) == Truth::True) {
                    f.collect(file, found);
                }
            }
            Self::Not(_) => {}
            leaf => found.push(leaf),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(n: u32, value: &str) -> Filter {
        Filter::Text {
            id: CondId::Number(n),
            order: n as usize,
            target: TextTarget::Name,
            matcher: TextMatcher(Regex::new(&format!("^{value}$")).unwrap()),
        }
    }

    fn ids(filter: &Filter, name: &str) -> Option<Vec<String>> {
        let file = FileValues {
            name,
            path: name,
            entry_type: EntryType::File,
            empty: false,
            size: 0,
            modified: Some(0.0),
            created: None,
            accessed: None,
            lang: "unknown",
            kind: Kind::Other,
            git: None,
            content: ContentState::Absent,
        };
        filter.matches(&file).then(|| {
            filter
                .contributors(&file)
                .iter()
                .map(ToString::to_string)
                .collect()
        })
    }

    #[test]
    fn and_lists_all_parts() {
        let f = Filter::And(vec![text(1, "a.*"), text(2, ".*z")]);
        assert_eq!(ids(&f, "abz"), Some(vec!["#1".into(), "#2".into()]));
        assert_eq!(ids(&f, "ab"), None);
    }

    #[test]
    fn or_lists_true_parts() {
        let f = Filter::Or(vec![text(1, "a"), text(2, "b"), text(3, ".")]);
        assert_eq!(ids(&f, "b"), Some(vec!["#2".into(), "#3".into()]));
    }

    #[test]
    fn xor_is_an_odd_number_of_true_parts() {
        let f = Filter::Xor(vec![text(1, ".*a.*"), text(2, ".*b.*"), text(3, ".*c.*")]);
        assert_eq!(
            ids(&f, "abc"),
            Some(vec!["#1".into(), "#2".into(), "#3".into()])
        );
        assert_eq!(ids(&f, "ab"), None);
        assert_eq!(ids(&f, "c"), Some(vec!["#3".into()]));
    }

    #[test]
    fn nothing_under_not() {
        let f = Filter::And(vec![text(1, ".*"), Filter::Not(Box::new(text(2, "x")))]);
        assert_eq!(ids(&f, "y"), Some(vec!["#1".into()]));
        let f = Filter::Not(Box::new(Filter::Not(Box::new(text(1, "x")))));
        assert_eq!(ids(&f, "x"), Some(vec![]));
    }

    #[test]
    fn nested_branches() {
        // (#1 and #2) or #3: a true #1 in a false branch is not listed
        let f = Filter::Or(vec![
            Filter::And(vec![text(1, "a.*"), text(2, ".*b")]),
            text(3, ".*"),
        ]);
        assert_eq!(ids(&f, "ax"), Some(vec!["#3".into()]));
        assert_eq!(
            ids(&f, "ab"),
            Some(vec!["#1".into(), "#2".into(), "#3".into()])
        );
    }

    fn created(n: u32) -> Filter {
        Filter::Number {
            id: CondId::Number(n),
            order: n as usize,
            target: NumberTarget::Created,
            bounds: Bounds {
                min: Some((0.0, true)),
                max: None,
                negated: false,
            },
        }
    }

    #[test]
    fn truth_tables() {
        use Truth::{False, True, Unknown};
        assert_eq!(Unknown.not(), Unknown);
        assert_eq!(True.and(Unknown), Unknown);
        assert_eq!(False.and(Unknown), False);
        assert_eq!(True.or(Unknown), True);
        assert_eq!(False.or(Unknown), Unknown);
        assert_eq!(True.xor(Unknown), Unknown);
        assert_eq!(True.xor(True), False);
        assert_eq!(True.xor(False), True);
    }

    #[test]
    fn a_missing_date_is_unknown() {
        let file = FileValues {
            name: "a",
            path: "a",
            entry_type: EntryType::File,
            empty: false,
            size: 0,
            modified: Some(0.0),
            created: None,
            accessed: None,
            lang: "unknown",
            kind: Kind::Other,
            git: None,
            content: ContentState::Absent,
        };
        assert_eq!(created(1).eval(&file), Truth::Unknown);
        assert!(!Filter::Not(Box::new(created(1))).matches(&file));
        let either = Filter::Or(vec![created(1), text(2, "a")]);
        assert!(either.matches(&file));
        assert_eq!(either.contributors(&file), [CondId::Number(2)]);
        assert!(!Filter::And(vec![created(1), text(2, "a")]).matches(&file));
    }
}
