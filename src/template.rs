//! The template `to` of `rename`: placeholders of the old path and groups of `pattern` (TOOLS.md#the-target).

use crate::error::{ErrorCode, QueryError};

/// The placeholders of the old path, in the order of the spec.
pub(crate) const PLACEHOLDERS: [&str; 5] = ["dir", "name", "stem", "ext", "path"];

/// A part of the old path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Placeholder {
    Dir,
    Name,
    Stem,
    Ext,
    Path,
}

impl Placeholder {
    fn of(name: &str) -> Option<Self> {
        Some(match name {
            "dir" => Self::Dir,
            "name" => Self::Name,
            "stem" => Self::Stem,
            "ext" => Self::Ext,
            "path" => Self::Path,
            _ => return None,
        })
    }
}

/// A part of a template.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Part {
    Text(String),
    Placeholder(Placeholder),
    /// A group of `pattern` by its number (a named group by the number of its name).
    Group(usize),
}

/// The groups of `pattern`: their number and the names of the named ones with their numbers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Groups {
    /// Whether there is a `pattern`.
    pub pattern: bool,
    /// The number of groups, without the whole match.
    pub count: usize,
    pub names: Vec<(String, usize)>,
}

impl Groups {
    /// The groups of a compiled `pattern`.
    pub(crate) fn of(regex: &regex::Regex) -> Self {
        Self {
            pattern: true,
            count: regex.captures_len() - 1,
            names: regex
                .capture_names()
                .enumerate()
                .filter_map(|(i, name)| Some((name?.to_string(), i)))
                .collect(),
        }
    }

    /// The hint of an unknown placeholder or group.
    fn hint(&self) -> String {
        let placeholders = format!("the placeholders are {}", PLACEHOLDERS.join(", "));
        if self.count == 0 {
            return format!("{placeholders}; groups need a 'pattern'");
        }
        let groups: Vec<String> = (1..=self.count)
            .map(|n| n.to_string())
            .chain(self.names.iter().map(|(name, _)| name.clone()))
            .collect();
        format!(
            "{placeholders}; the groups of 'pattern' are {}",
            groups.join(", ")
        )
    }
}

/// A checked template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Template {
    parts: Vec<Part>,
}

impl Template {
    /// Parses `to` with the groups of `pattern`; the error has its byte range in `to`.
    pub(crate) fn parse(to: &str, groups: &Groups) -> Result<Self, QueryError> {
        if to.starts_with('/') {
            return Err(QueryError::new(
                ErrorCode::InvalidTemplate,
                "the target must be relative",
                0..1,
            )
            .hint("the target is relative to the working directory"));
        }
        let mut parts = Vec::new();
        let mut text = String::new();
        let mut rest = to.char_indices().peekable();
        while let Some((i, c)) = rest.next() {
            match c {
                '{' if rest.peek().is_some_and(|(_, n)| *n == '{') => {
                    rest.next();
                    text.push('{');
                }
                '}' if rest.peek().is_some_and(|(_, n)| *n == '}') => {
                    rest.next();
                    text.push('}');
                }
                '}' => {
                    return Err(QueryError::new(
                        ErrorCode::InvalidTemplate,
                        "unmatched '}'",
                        i..i + 1,
                    )
                    .hint("write '}}' for a brace"));
                }
                '{' => {
                    let Some(len) = to[i + 1..].find('}') else {
                        return Err(QueryError::new(
                            ErrorCode::InvalidTemplate,
                            "unclosed '{'",
                            i..to.len(),
                        )
                        .hint("write '{{' for a brace"));
                    };
                    let end = i + 1 + len;
                    let name = &to[i + 1..end];
                    let span = i..end + 1;
                    while rest.peek().is_some_and(|(j, _)| *j <= end) {
                        rest.next();
                    }
                    if !text.is_empty() {
                        parts.push(Part::Text(std::mem::take(&mut text)));
                    }
                    parts.push(placeholder(name, groups, span)?);
                }
                c => text.push(c),
            }
        }
        if to.ends_with('/') {
            return Err(QueryError::new(
                ErrorCode::InvalidTemplate,
                "the target ends with '/'",
                to.len() - 1..to.len(),
            )
            .hint("end it with a name, e.g. 'archive/{name}'"));
        }
        if !text.is_empty() {
            parts.push(Part::Text(text));
        }
        Ok(Self { parts })
    }

    /// The target of the old path `rel` (relative to the working directory, with `/`) and the groups of its match.
    pub(crate) fn expand(&self, rel: &str, captures: Option<&regex::Captures<'_>>) -> String {
        let (dir, name) = rel.rsplit_once('/').unwrap_or(("", rel));
        let (stem, ext) = match name.rfind('.') {
            Some(dot) if dot > 0 => (&name[..dot], &name[dot + 1..]),
            _ => (name, ""),
        };
        let mut out = String::new();
        for part in &self.parts {
            match part {
                Part::Text(text) => out.push_str(text),
                Part::Placeholder(p) => out.push_str(match p {
                    Placeholder::Dir => dir,
                    Placeholder::Name => name,
                    Placeholder::Stem => stem,
                    Placeholder::Ext => ext,
                    Placeholder::Path => rel,
                }),
                Part::Group(n) => {
                    if let Some(group) = captures.and_then(|c| c.get(*n)) {
                        out.push_str(group.as_str());
                    }
                }
            }
        }
        out
    }
}

/// A placeholder or a group in braces, at `span` of the template.
fn placeholder(
    name: &str,
    groups: &Groups,
    span: std::ops::Range<usize>,
) -> Result<Part, QueryError> {
    if name.is_empty() {
        return Err(
            QueryError::new(ErrorCode::InvalidTemplate, "empty placeholder", span)
                .hint(groups.hint()),
        );
    }
    if let Some(p) = Placeholder::of(name) {
        return Ok(Part::Placeholder(p));
    }
    if name.bytes().all(|b| b.is_ascii_digit()) {
        return match name.parse::<usize>() {
            Ok(n) if (1..=groups.count).contains(&n) => Ok(Part::Group(n)),
            _ => Err(QueryError::new(
                ErrorCode::InvalidTemplate,
                format!("the group {name} doesn't exist"),
                span,
            )
            .hint(groups.hint())),
        };
    }
    match groups.names.iter().find(|(n, _)| n == name) {
        Some((_, n)) => Ok(Part::Group(*n)),
        None => Err(QueryError::new(
            ErrorCode::InvalidTemplate,
            format!("unknown placeholder '{name}'"),
            span,
        )
        .hint(groups.hint())),
    }
}

/// The target without empty segments and `.`, `..` removing the segment before it (kept at the start); `None` when
/// nothing is left.
pub(crate) fn normalize(target: &str) -> Option<Vec<&str>> {
    let mut segments: Vec<&str> = Vec::new();
    for segment in target.split('/') {
        match segment {
            "" | "." => {}
            ".." if segments.last().is_some_and(|s| *s != "..") => {
                segments.pop();
            }
            other => segments.push(other),
        }
    }
    (!segments.is_empty()).then_some(segments)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expand(to: &str, rel: &str) -> String {
        Template::parse(to, &Groups::default())
            .unwrap()
            .expand(rel, None)
    }

    fn error(to: &str, groups: &Groups) -> String {
        Template::parse(to, groups)
            .unwrap_err()
            .into_field_report("to", to)
            .to_text()
    }

    #[test]
    fn the_placeholders_of_the_old_path() {
        let all = "{dir}|{name}|{stem}|{ext}|{path}";
        assert_eq!(
            expand(all, "src/app/main.test.ts"),
            "src/app|main.test.ts|main.test|ts|src/app/main.test.ts"
        );
        assert_eq!(expand(all, "README"), "|README|README||README");
        assert_eq!(
            expand(all, ".gitignore"),
            "|.gitignore|.gitignore||.gitignore"
        );
        assert_eq!(expand(all, "src/build"), "src|build|build||src/build");
        assert_eq!(expand("{{{name}}}", "a.rs"), "{a.rs}");
    }

    #[test]
    fn groups_by_number_and_by_name() {
        let regex = regex::Regex::new(r"^(?:src/(?<base>.*)_test(x)?\.rs)$").unwrap();
        let groups = Groups::of(&regex);
        assert_eq!(groups.count, 2);
        let template = Template::parse("tests/{1}-{base}-{2}.rs", &groups).unwrap();
        let captures = regex.captures("src/eval_test.rs").unwrap();
        assert_eq!(
            template.expand("src/eval_test.rs", Some(&captures)),
            "tests/eval-eval-.rs"
        );
    }

    #[test]
    fn errors_with_their_positions() {
        let none = Groups::default();
        assert_eq!(
            error("{dir}/{base}.ts", &none),
            "error[invalid_template]: unknown placeholder 'base'\n  {dir}/{base}.ts\n        ^^^^^^\n  hint: the placeholders are dir, name, stem, ext, path; groups need a 'pattern'\n"
        );
        let regex = regex::Regex::new(r"(.*)(?<word>x)").unwrap();
        assert_eq!(
            error("{3}.rs", &Groups::of(&regex)),
            "error[invalid_template]: the group 3 doesn't exist\n  {3}.rs\n  ^^^\n  hint: the placeholders are dir, name, stem, ext, path; the groups of 'pattern' are 1, 2, word\n"
        );
        assert!(error("lib/{}.ts", &none).contains("empty placeholder\n  lib/{}.ts\n      ^^\n"));
        assert!(error("{name.bak", &none).contains("unclosed '{'\n  {name.bak\n  ^^^^^^^^^\n"));
        assert!(error("name}.bak", &none).contains("unmatched '}'\n  name}.bak\n      ^\n"));
        assert!(
            error("/tmp/{x}", &none).contains("the target must be relative\n  /tmp/{x}\n  ^\n")
        );
        assert!(
            error("archive/", &none).contains("the target ends with '/'\n  archive/\n         ^\n")
        );
        // the braces before the end
        assert!(error("{x}/", &none).contains("unknown placeholder 'x'"));
    }

    #[test]
    fn normalized_targets() {
        assert_eq!(normalize("/main.tsx"), Some(vec!["main.tsx"]));
        assert_eq!(normalize("a/./b//c"), Some(vec!["a", "b", "c"]));
        assert_eq!(normalize("a/../b"), Some(vec!["b"]));
        assert_eq!(normalize("../../x"), Some(vec!["..", "..", "x"]));
        assert_eq!(normalize("src/.."), None);
        assert_eq!(normalize(""), None);
    }
}
