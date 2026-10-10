//! The compiled query and its evaluation for one file (QUERY.md#basics, OUTPUT.md#file-conditions).

use regex::Regex;

use super::ast::CondId;

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

/// The values of one file which conditions look at.
#[derive(Debug, Clone, Copy)]
pub struct FileValues<'a> {
    /// The NFC file name.
    pub name: &'a str,
    /// The NFC path relative to the working directory, `/` as the separator.
    pub path: &'a str,
    /// The type of the entry; symlinks are not followed.
    pub entry_type: EntryType,
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
}

impl Filter {
    /// Whether the file matches.
    pub fn eval(&self, file: &FileValues<'_>) -> bool {
        match self {
            Self::And(items) => items.iter().all(|f| f.eval(file)),
            Self::Or(items) => items.iter().any(|f| f.eval(file)),
            Self::Xor(items) => items.iter().filter(|f| f.eval(file)).count() % 2 == 1,
            Self::Not(inner) => !inner.eval(file),
            Self::Text {
                target, matcher, ..
            } => matcher.is_match(match target {
                TextTarget::Name => file.name,
                TextTarget::Path => file.path,
            }),
            Self::Type {
                values, negated, ..
            } => values.contains(&file.entry_type) != *negated,
        }
    }

    /// The ids of the conditions which made a matching file match, in the order of appearance.
    pub fn contributors(&self, file: &FileValues<'_>) -> Vec<CondId> {
        let mut found = Vec::new();
        self.collect(file, &mut found);
        found.sort_by_key(|(order, _)| *order);
        found.dedup_by_key(|(order, _)| *order);
        found.into_iter().map(|(_, id)| id).collect()
    }

    /// Adds the contributing conditions of this true part: all parts of `and`, the true parts of `or` / `xor`,
    /// nothing under `not`.
    fn collect(&self, file: &FileValues<'_>, found: &mut Vec<(usize, CondId)>) {
        match self {
            Self::And(items) => items.iter().for_each(|f| f.collect(file, found)),
            Self::Or(items) | Self::Xor(items) => {
                for f in items.iter().filter(|f| f.eval(file)) {
                    f.collect(file, found);
                }
            }
            Self::Not(_) => {}
            Self::Text { id, order, .. } | Self::Type { id, order, .. } => {
                found.push((*order, id.clone()));
            }
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
        };
        filter.eval(&file).then(|| {
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
}
