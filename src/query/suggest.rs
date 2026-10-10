//! "did you mean" suggestions for unknown words in the query and the request.

/// Words agents use for the keywords of the spec.
const SYNONYMS: &[(&str, &str)] = &[
    ("content", "text"),
    ("contents", "text"),
    ("body", "text"),
    ("line_text", "text"),
    ("filename", "name"),
    ("file", "name"),
    ("basename", "name"),
    ("ext", "name"),
    ("extension", "name"),
    ("dir", "path"),
    ("directory", "path"),
    ("folder", "path"),
    ("folder", "dir"),
    ("directory", "dir"),
    ("symlink", "link"),
    ("regular", "file"),
    ("mtime", "modified"),
    ("ctime", "created"),
    ("atime", "accessed"),
    ("language", "lang"),
    ("length", "len"),
    ("contains", "has"),
    ("include", "has"),
    ("includes", "has"),
    ("like", "~"),
    ("glob", "~"),
    ("matches", "=~"),
    ("match", "=~"),
    ("regex", "=~"),
    ("startswith", "starts"),
    ("starts_with", "starts"),
    ("start", "starts"),
    ("endswith", "ends"),
    ("ends_with", "ends"),
    ("end", "ends"),
    ("==", "="),
    ("eq", "="),
    ("is", "="),
    ("<>", "!="),
    ("ne", "!="),
];

/// The candidate closest to `word`: a known synonym, an edit distance of at most 2, or a candidate contained in
/// the word (`filename` → `name`).
pub fn did_you_mean<'a>(word: &str, candidates: &[&'a str]) -> Option<&'a str> {
    let lower = word.to_lowercase();
    if let Some(c) = SYNONYMS
        .iter()
        .filter(|(from, _)| *from == lower)
        .find_map(|(_, to)| candidates.iter().find(|c| *c == to))
    {
        return Some(c);
    }
    let best = candidates
        .iter()
        .map(|c| (edit_distance(&lower, c), *c))
        .filter(|(d, c)| *d <= 2 && *d < c.chars().count())
        .min_by_key(|(d, _)| *d)
        .map(|(_, c)| c);
    best.or_else(|| {
        candidates
            .iter()
            .filter(|c| c.chars().count() >= 3 && lower.contains(**c))
            .max_by_key(|c| c.len())
            .copied()
    })
}

/// Levenshtein distance in characters.
fn edit_distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut cur = vec![i + 1; b.len() + 1];
        for (j, cb) in b.iter().enumerate() {
            let cost = usize::from(ca != *cb);
            cur[j + 1] = (prev[j] + cost).min(prev[j + 1] + 1).min(cur[j] + 1);
        }
        prev = cur;
    }
    prev[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGETS: &[&str] = &["path", "name", "type", "text", "len", "lang"];

    #[test]
    fn synonyms() {
        assert_eq!(did_you_mean("content", TARGETS), Some("text"));
        assert_eq!(
            did_you_mean("contains", &["=", "has", "starts"]),
            Some("has")
        );
    }

    #[test]
    fn typos() {
        assert_eq!(did_you_mean("nmae", TARGETS), Some("name"));
        assert_eq!(did_you_mean("PATH", TARGETS), Some("path"));
    }

    #[test]
    fn contained_candidates() {
        assert_eq!(did_you_mean("filepath", TARGETS), Some("path"));
    }

    #[test]
    fn nothing_similar() {
        assert_eq!(did_you_mean("colour", &["dirs", "query", "output"]), None);
        assert_eq!(did_you_mean("x", &["=", "has"]), None);
    }
}
