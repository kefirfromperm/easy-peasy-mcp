//! Rejects valid query syntax which this server version doesn't implement yet with `not_implemented`.
//!
//! Implemented: the empty query, or exactly one condition `name = <string>` or `name ~ <wildcard>`, without
//! modifiers and labels.

use std::ops::Range;

use super::ast::{Op, Target};
use super::lexer::{Lexed, Tok};
use crate::error::{ErrorCode, QueryError};

/// The first token of a syntactically valid query which is outside of the implemented subset.
pub fn check(query: &str, lexed: &Lexed) -> Option<QueryError> {
    let unsupported = first_unsupported_token(query, lexed);
    let comment = lexed
        .comments
        .first()
        .map(|span| (span.clone(), "the comment '#'".to_string()));
    let (span, feature) = match (unsupported, comment) {
        (Some(t), Some(c)) => {
            if c.0.start <= t.0.start {
                c
            } else {
                t
            }
        }
        (t, c) => t.or(c)?,
    };
    Some(QueryError::new(
        ErrorCode::NotImplemented,
        format!("{feature} is not implemented yet"),
        span,
    ))
}

fn first_unsupported_token(query: &str, lexed: &Lexed) -> Option<(Range<usize>, String)> {
    for (i, token) in lexed.tokens.iter().enumerate() {
        let text = &query[token.span.clone()];
        let feature = match (i, &token.tok) {
            (0, Tok::Word(w)) if Target::from_keyword(w) == Some(Target::Name) => continue,
            (1, Tok::Op(o)) if matches!(Op::from_symbol(o), Some(Op::Eq | Op::Wildcard)) => {
                continue;
            }
            (2, Tok::Str { .. }) => continue,
            (_, Tok::LParen) => "the bracket '('".to_string(),
            (_, Tok::Word(w)) if w.eq_ignore_ascii_case("line") => {
                "the group line( ... )".to_string()
            }
            (_, Tok::Word(w)) if w.eq_ignore_ascii_case("near") => {
                "the group near( ... )".to_string()
            }
            (_, Tok::Word(w)) if Target::from_keyword(w).is_some() && i == 0 => {
                format!("the target '{text}'")
            }
            (_, Tok::Modifier(_)) => format!("the modifier '{text}'"),
            (_, Tok::Label(_)) => format!("the label '{text}'"),
            (_, Tok::Op(_) | Tok::Word(_)) => format!("the operator '{text}'"),
            _ => format!("'{text}'"),
        };
        return Some((token.span.clone(), feature));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::lexer::lex;

    fn check_str(q: &str) -> Option<(String, Range<usize>)> {
        check(q, &lex(q)).map(|e| (e.message, e.span))
    }

    #[test]
    fn the_subset_passes() {
        assert_eq!(check_str("name ~ '*.rs'"), None);
        assert_eq!(check_str("NAME = \"a\""), None);
    }

    #[test]
    fn other_targets_and_operators() {
        assert_eq!(
            check_str("path ~ 'a'"),
            Some(("the target 'path' is not implemented yet".into(), 0..4))
        );
        assert_eq!(check_str("name has 'a'"), None);
        assert_eq!(check_str("name STARTS 'a'"), None);
        assert_eq!(check_str("name ends 'a'"), None);
        assert_eq!(
            check_str("name =~ 'a'"),
            Some(("the operator '=~' is not implemented yet".into(), 5..7))
        );
        assert_eq!(
            check_str("name ~~ 'a'").unwrap().0,
            "the operator '~~' is not implemented yet"
        );
        assert_eq!(
            check_str("empty").unwrap().0,
            "the target 'empty' is not implemented yet"
        );
    }

    #[test]
    fn logic_groups_modifiers_labels() {
        assert_eq!(
            check_str("not name = 'a'").unwrap().0,
            "the operator 'not' is not implemented yet"
        );
        assert_eq!(
            check_str("name = 'a' and name = 'b'"),
            Some(("the operator 'and' is not implemented yet".into(), 11..14))
        );
        assert_eq!(
            check_str("(name = 'a')").unwrap().0,
            "the bracket '(' is not implemented yet"
        );
        assert_eq!(
            check_str("line(text has 'a')").unwrap().0,
            "the group line( ... ) is not implemented yet"
        );
        assert_eq!(
            check_str("near(text has 'a', text has 'b', 1)").unwrap().0,
            "the group near( ... ) is not implemented yet"
        );
        assert_eq!(
            check_str("name = 'a' /i").unwrap().0,
            "the modifier '/i' is not implemented yet"
        );
        assert_eq!(
            check_str("name = 'a' @x").unwrap().0,
            "the label '@x' is not implemented yet"
        );
    }

    #[test]
    fn comments_and_the_empty_query() {
        assert_eq!(
            check_str("name = 'a' # x"),
            Some(("the comment '#' is not implemented yet".into(), 11..14))
        );
        assert_eq!(
            check_str("# only").unwrap().0,
            "the comment '#' is not implemented yet"
        );
        assert_eq!(check_str(""), None);
        assert_eq!(check_str(" \n\t"), None);
    }
}
