//! Rejects valid query syntax which this server version doesn't implement yet with `not_implemented`.
//!
//! Implemented: the empty query, or exactly one condition on `name` or `path` with `=`, `has`, `starts`, `ends` or `~`,
//! with the modifier `/i` only and without labels.

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
            (0, Tok::Word(w))
                if matches!(Target::from_keyword(w), Some(Target::Name | Target::Path)) =>
            {
                continue;
            }
            (1, Tok::Op(o) | Tok::Word(o))
                if matches!(
                    Op::from_symbol(o),
                    Some(Op::Eq | Op::Has | Op::Starts | Op::Ends | Op::Wildcard)
                ) =>
            {
                continue;
            }
            (2, Tok::Str { .. }) => continue,
            (3.., Tok::Modifier(m)) if m.eq_ignore_ascii_case("i") => continue,
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
    use crate::query::{lexer::lex, parser::parse};

    fn check_str(q: &str) -> Option<(String, Range<usize>)> {
        let lexed = lex(q);
        let expr = parse(q, &lexed).unwrap();
        check(q, &lexed, expr.as_ref()).map(|e| (e.message, e.span))
    }

    #[test]
    fn the_subset_passes() {
        for q in [
            "",
            " \n\t",
            "name ~ '*.rs'",
            "NAME = \"a\"",
            "name has 'a' and not (path starts 'b' or name ends 'c' /i) xor name = 'd'",
            "not not path ~ 'a'",
        ] {
            assert_eq!(check_str(q), None, "{q}");
        }
    }

    #[test]
    fn other_targets_and_operators() {
        assert_eq!(
            check_str("size > 1"),
            Some(("the target 'size' is not implemented yet".into(), 0..4))
        );
        assert_eq!(
            check_str("name = 'a' or (path ~ 'b' and text has 'c')"),
            Some(("the target 'text' is not implemented yet".into(), 30..34))
        );
        assert_eq!(
            check_str("name = 'a' and name =~ 'a'"),
            Some(("the operator '=~' is not implemented yet".into(), 20..22))
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
    fn groups_modifiers_labels_comments() {
        assert_eq!(
            check_str("name = 'a' and line(text has 'a')"),
            Some((
                "the group line( ... ) is not implemented yet".into(),
                15..19
            ))
        );
        assert_eq!(
            check_str("near(text has 'a', text has 'b', 1)").unwrap().0,
            "the group near( ... ) is not implemented yet"
        );
        assert_eq!(
            check_str("name = 'a' /i/w"),
            Some(("the modifier '/w' is not implemented yet".into(), 13..15))
        );
        assert_eq!(
            check_str("name = 'a' /e(xml, 1)"),
            Some(("the modifier '/e' is not implemented yet".into(), 11..13))
        );
        assert_eq!(
            check_str("name = 'a' @x"),
            Some(("the label '@x' is not implemented yet".into(), 11..13))
        );
        assert_eq!(
            check_str("name = 'a' # x"),
            Some(("the comment '#' is not implemented yet".into(), 11..14))
        );
        assert_eq!(
            check_str("# only").unwrap().0,
            "the comment '#' is not implemented yet"
        );
    }
}
