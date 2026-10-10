//! Rejects valid query syntax which this server version doesn't implement yet with `not_implemented`.
//!
//! Implemented: the empty query; conditions on `name` and `path` with `=`, `has`, `starts`, `ends`, `~` and the
//! modifier `/i`, combined with `not`, `and`, `xor`, `or` and brackets.

use std::ops::Range;

use super::ast::{Expr, Node, NodeKind, Op, Target};
use super::lexer::Lexed;
use crate::error::{ErrorCode, QueryError};

/// The first construct of a syntactically valid query, by position, which is outside of the implemented subset.
pub fn check(query: &str, lexed: &Lexed, expr: Option<&Expr>) -> Option<QueryError> {
    let mut found: Vec<(Range<usize>, String)> = lexed
        .comments
        .iter()
        .map(|span| (span.clone(), "the comment '#'".to_string()))
        .collect();
    if let Some(expr) = expr {
        expr.visit_nodes(&mut |node| unsupported(query, lexed, node, &mut found));
    }
    let (span, feature) = found.into_iter().min_by_key(|(span, _)| span.start)?;
    Some(QueryError::new(
        ErrorCode::NotImplemented,
        format!("{feature} is not implemented yet"),
        span,
    ))
}

/// Adds the unsupported parts of one node.
fn unsupported(query: &str, lexed: &Lexed, node: &Node, found: &mut Vec<(Range<usize>, String)>) {
    let keyword = lexed.tokens[node.tokens.start].span.clone();
    match &node.kind {
        NodeKind::Line(_) => found.push((keyword, "the group line( ... )".into())),
        NodeKind::Near(_) => found.push((keyword, "the group near( ... )".into())),
        NodeKind::Condition(c) => {
            if !matches!(c.target, Target::Name | Target::Path) {
                let span = c.target_span.clone();
                found.push((span.clone(), format!("the target '{}'", &query[span])));
            } else if let Some((op, span)) = &c.op
                && !matches!(op, Op::Eq | Op::Has | Op::Starts | Op::Ends | Op::Wildcard)
            {
                found.push((
                    span.clone(),
                    format!("the operator '{}'", &query[span.clone()]),
                ));
            }
            for m in c.modifiers.iter().filter(|m| m.name != "i") {
                let span = m.span.start..m.span.start + 1 + m.name.len();
                found.push((span.clone(), format!("the modifier '{}'", &query[span])));
            }
        }
    }
    if let Some(label) = &node.label {
        let span = label.span.clone();
        found.push((span.clone(), format!("the label '{}'", &query[span])));
    }
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
