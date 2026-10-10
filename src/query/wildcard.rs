//! Wildcards of `path ~` and `name ~` compiled to anchored regexes (QUERY.md#wildcards).

use std::fmt::Write as _;

use regex::Regex;

/// Why a wildcard is invalid: the hint of `invalid_wildcard`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WildcardError(pub String);

/// Compiles a wildcard which must match the whole value.
pub fn compile(pattern: &str, case_insensitive: bool) -> Result<Regex, WildcardError> {
    let chars: Vec<char> = pattern.chars().collect();
    let mut re = String::from(if case_insensitive { "(?is)^" } else { "(?s)^" });
    let mut pos = 0;
    translate(&chars, &mut pos, 0, &mut re)?;
    re.push('$');
    Ok(Regex::new(&re).expect("a translated wildcard is a valid regex"))
}

/// Translates `chars[*pos..]` until the end, or inside `{ }` (depth > 0) until `,` or `}`.
fn translate(
    chars: &[char],
    pos: &mut usize,
    depth: usize,
    re: &mut String,
) -> Result<(), WildcardError> {
    while let Some(&c) = chars.get(*pos) {
        match c {
            ',' | '}' if depth > 0 => return Ok(()),
            '*' if chars.get(*pos + 1) == Some(&'*') => {
                *pos += 2;
                if chars.get(*pos) == Some(&'/') {
                    *pos += 1;
                    re.push_str("(?:.*/)?");
                } else {
                    re.push_str(".*");
                }
            }
            '*' => {
                *pos += 1;
                re.push_str("[^/]*");
            }
            '?' => {
                *pos += 1;
                re.push_str("[^/]");
            }
            '[' => translate_set(chars, pos, re)?,
            '{' => {
                *pos += 1;
                re.push_str("(?:");
                loop {
                    translate(chars, pos, depth + 1, re)?;
                    match chars.get(*pos) {
                        Some(',') => {
                            *pos += 1;
                            re.push('|');
                        }
                        Some('}') => {
                            *pos += 1;
                            re.push(')');
                            break;
                        }
                        _ => return Err(WildcardError("unclosed '{'".into())),
                    }
                }
            }
            _ => {
                *pos += 1;
                push_literal(re, c);
            }
        }
    }
    Ok(())
}

/// Translates `[...]` starting at `chars[*pos] == '['`.
fn translate_set(chars: &[char], pos: &mut usize, re: &mut String) -> Result<(), WildcardError> {
    let mut i = *pos + 1;
    let negated = chars.get(i) == Some(&'!');
    if negated {
        i += 1;
    }
    let mut items: Vec<(char, char)> = Vec::new();
    let mut first = true;
    loop {
        let Some(&c) = chars.get(i) else {
            return Err(WildcardError("unclosed '['".into()));
        };
        if c == ']' && !first {
            break;
        }
        first = false;
        if chars.get(i + 1) == Some(&'-') && chars.get(i + 2).is_some_and(|&e| e != ']') {
            let end = chars[i + 2];
            if end < c {
                return Err(WildcardError(format!("invalid range '{c}-{end}'")));
            }
            items.push((c, end));
            i += 3;
        } else {
            items.push((c, c));
            i += 1;
        }
    }
    *pos = i + 1;
    re.push('[');
    if negated {
        re.push_str("^/");
    }
    for (a, b) in items {
        let _ = write!(re, "\\x{{{:x}}}", u32::from(a));
        if a != b {
            let _ = write!(re, "-\\x{{{:x}}}", u32::from(b));
        }
    }
    re.push(']');
    Ok(())
}

fn push_literal(re: &mut String, c: char) {
    let mut buf = [0; 4];
    re.push_str(&regex::escape(c.encode_utf8(&mut buf)));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn m(pattern: &str, value: &str) -> bool {
        compile(pattern, false).unwrap().is_match(value)
    }

    #[test]
    fn star_and_question_mark() {
        assert!(m("*.rs", "main.rs"));
        assert!(m("*.rs", ".rs"));
        assert!(!m("*.rs", "main.rs.bak"));
        assert!(!m("*.rs", "src/main.rs"));
        assert!(m("v?.json", "v1.json"));
        assert!(!m("v?.json", "v10.json"));
        assert!(!m("a?b", "a/b"));
    }

    #[test]
    fn double_star() {
        assert!(m("src/**/*.rs", "src/main.rs"));
        assert!(m("src/**/*.rs", "src/a/b/main.rs"));
        assert!(m("**/tests/**", "tests/a.rs"));
        assert!(m("**/tests/**", "x/tests/a/b.rs"));
        assert!(!m("tests/**", "x/tests/a.rs"));
        assert!(m("**/*.lock", "Cargo.lock"));
        assert!(m("a**", "a/b/c"));
    }

    #[test]
    fn sets() {
        assert!(m("v[13].json", "v3.json"));
        assert!(!m("v[13].json", "v2.json"));
        assert!(m("v[1-2].json", "v2.json"));
        assert!(m("v[!1].json", "v2.json"));
        assert!(!m("v[!1].json", "v1.json"));
        assert!(!m("a[!x]b", "a/b"));
        assert!(m("[]]", "]"));
        assert!(m("[a-]", "-"));
    }

    #[test]
    fn alternatives() {
        assert!(m("app.{ts,tsx}", "app.tsx"));
        assert!(!m("app.{ts,tsx}", "app.js"));
        assert!(m("{a,b{c,d}}", "bd"));
        assert!(m("{,x}y", "y"));
        assert!(m("a,b}", "a,b}"));
    }

    #[test]
    fn bracket_escapes() {
        assert!(m("[*]", "*"));
        assert!(!m("[*]", "a"));
        assert!(m("[?]", "?"));
        assert!(m("[[]x", "[x"));
        assert!(m("[{]*}", "{wip}"));
    }

    #[test]
    fn literals_are_escaped() {
        assert!(m("a.b+c(d)^$|\\", "a.b+c(d)^$|\\"));
        assert!(!m("a.b", "axb"));
    }

    #[test]
    fn case_insensitive() {
        assert!(compile("readme*", true).unwrap().is_match("README.md"));
        assert!(!compile("readme*", false).unwrap().is_match("README.md"));
        assert!(compile("[a-c]x", true).unwrap().is_match("BX"));
    }

    #[test]
    fn errors() {
        assert_eq!(
            compile("[a-", false).unwrap_err(),
            WildcardError("unclosed '['".into())
        );
        assert_eq!(
            compile("[]", false).unwrap_err(),
            WildcardError("unclosed '['".into())
        );
        assert_eq!(
            compile("{a,b", false).unwrap_err(),
            WildcardError("unclosed '{'".into())
        );
        assert_eq!(
            compile("[z-a]", false).unwrap_err(),
            WildcardError("invalid range 'z-a'".into())
        );
    }
}
