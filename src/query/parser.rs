//! Recursive descent parser of the full query grammar (QUERY.md#grammar) with the errors of QUERY.md#errors.

use std::ops::Range;

use super::ast::{
    CondId, Condition, Expr, Modifier, Near, Node, NodeKind, Op, Spanned, StrLit, Target,
    TargetClass, Value,
};
use super::lexer::{Lexed, Tok, Token};
use super::suggest::did_you_mean;
use crate::error::{ErrorCode, QueryError};

/// Parses the tokens of `query`; `None` for an empty query.
pub fn parse(query: &str, lexed: &Lexed) -> Result<Option<Expr>, QueryError> {
    let mut parser = Parser {
        query,
        tokens: &lexed.tokens,
        pos: 0,
    };
    if parser.tokens.is_empty() {
        return Ok(None);
    }
    let expr = parser.or_expr()?;
    if let Some(token) = parser.peek() {
        let hint = if token.tok == Tok::RParen {
            "unmatched ')'"
        } else {
            MISSING_AND
        };
        return Err(parser.unexpected(token, hint));
    }
    Ok(Some(expr))
}

const MISSING_AND: &str = "missing 'and' / 'or' between conditions?";

struct Parser<'a> {
    query: &'a str,
    tokens: &'a [Token],
    pos: usize,
}

fn is_keyword(token: &Token, keyword: &str) -> bool {
    matches!(&token.tok, Tok::Word(w) if w.eq_ignore_ascii_case(keyword))
}

fn list(ops: &[Op]) -> String {
    ops.iter()
        .map(|o| o.symbol())
        .collect::<Vec<_>>()
        .join(", ")
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&'a Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn source(&self, span: &Range<usize>) -> &'a str {
        &self.query[span.clone()]
    }

    /// The token as quoted in a message: strings with their own quotes, others in single quotes.
    fn quoted(&self, token: &Token) -> String {
        match token.tok {
            Tok::Str { .. } => self.source(&token.span).to_string(),
            _ => format!("'{}'", self.source(&token.span)),
        }
    }

    fn unexpected(&self, token: &Token, hint: &str) -> QueryError {
        match &token.tok {
            Tok::Unterminated(quote) => QueryError::new(
                ErrorCode::UnterminatedString,
                "unterminated string",
                token.span.clone(),
            )
            .hint(format!("closing {} is missing", quote.char())),
            tok => {
                let hint = match tok {
                    Tok::Invalid('&') => "use 'and' instead of '&&'",
                    Tok::Invalid('|') => "use 'or' instead of '||'",
                    Tok::Invalid('@') => "a label starts with a letter or '_', e.g. @name",
                    _ => hint,
                };
                QueryError::new(
                    ErrorCode::UnexpectedToken,
                    format!("unexpected {}", self.quoted(token)),
                    token.span.clone(),
                )
                .hint(hint)
            }
        }
    }

    fn unexpected_end(&self, hint: impl Into<String>) -> QueryError {
        let end = self.query.len();
        QueryError::new(
            ErrorCode::UnexpectedEnd,
            "unexpected end of the query",
            end..end,
        )
        .hint(hint)
    }

    /// The hint when a condition is expected at the current position.
    fn condition_hint(&self) -> String {
        match self.pos.checked_sub(1).and_then(|i| self.tokens.get(i)) {
            Some(prev) if matches!(prev.tok, Tok::Word(_) | Tok::LParen | Tok::Comma) => {
                format!(
                    "a condition is expected after '{}'",
                    self.source(&prev.span)
                )
            }
            _ => "a condition is expected".to_string(),
        }
    }

    fn or_expr(&mut self) -> Result<Expr, QueryError> {
        self.chain("or", Self::xor_expr, Expr::Or)
    }

    fn xor_expr(&mut self) -> Result<Expr, QueryError> {
        self.chain("xor", Self::and_expr, Expr::Xor)
    }

    fn and_expr(&mut self) -> Result<Expr, QueryError> {
        self.chain("and", Self::unary, Expr::And)
    }

    fn chain(
        &mut self,
        keyword: &str,
        item: fn(&mut Self) -> Result<Expr, QueryError>,
        make: fn(Vec<Expr>) -> Expr,
    ) -> Result<Expr, QueryError> {
        let mut items = vec![item(self)?];
        while self.peek().is_some_and(|t| is_keyword(t, keyword)) {
            self.pos += 1;
            items.push(item(self)?);
        }
        Ok(if items.len() == 1 {
            items.pop().expect("one item")
        } else {
            make(items)
        })
    }

    fn unary(&mut self) -> Result<Expr, QueryError> {
        if self.peek().is_some_and(|t| is_keyword(t, "not")) {
            self.pos += 1;
            return Ok(Expr::Not(Box::new(self.unary()?)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, QueryError> {
        let Some(token) = self.peek() else {
            return Err(self.unexpected_end(self.condition_hint()));
        };
        let start = self.pos;
        let kind = match &token.tok {
            Tok::LParen => {
                self.pos += 1;
                let inner = self.or_expr()?;
                self.expect_rparen()?;
                return Ok(Expr::Group(Box::new(inner)));
            }
            Tok::Word(w) if w.eq_ignore_ascii_case("line") => {
                NodeKind::Line(Box::new(self.line()?))
            }
            Tok::Word(w) if w.eq_ignore_ascii_case("near") => NodeKind::Near(self.near()?),
            Tok::Word(w)
                if ["and", "or", "xor"]
                    .iter()
                    .any(|k| w.eq_ignore_ascii_case(k)) =>
            {
                return Err(self.unexpected(token, &self.condition_hint()));
            }
            Tok::Word(w) => match Target::from_keyword(w) {
                Some(target) => NodeKind::Condition(self.condition(target)?),
                None => return Err(Self::unknown_target(token, w)),
            },
            _ => return Err(self.unexpected(token, &self.condition_hint())),
        };
        let end = self.pos;
        let label = match self.peek() {
            Some(Token {
                tok: Tok::Label(name),
                span,
            }) => {
                self.pos += 1;
                Some(Spanned {
                    text: name.clone(),
                    span: span.clone(),
                })
            }
            _ => None,
        };
        Ok(Expr::Node(Box::new(Node {
            kind,
            label,
            tokens: start..end,
            id: CondId::Number(0),
        })))
    }

    fn unknown_target(token: &Token, word: &str) -> QueryError {
        let names: Vec<&str> = Target::ALL.iter().map(|t| t.keyword()).collect();
        let expected = format!("expected one of: {}", names.join(", "));
        let hint = match did_you_mean(word, &names) {
            Some(s) => format!("did you mean '{s}'? {expected}"),
            None => expected,
        };
        QueryError::new(
            ErrorCode::UnknownTarget,
            format!("unknown target '{word}'"),
            token.span.clone(),
        )
        .hint(hint)
    }

    fn expect_rparen(&mut self) -> Result<(), QueryError> {
        match self.next() {
            None => Err(self.unexpected_end("closing ')' is missing")),
            Some(Token {
                tok: Tok::RParen, ..
            }) => Ok(()),
            Some(token) => Err(self.unexpected(token, MISSING_AND)),
        }
    }

    /// Consumes `keyword` and `(`.
    fn open_group(&mut self) -> Result<(), QueryError> {
        let keyword = self.next().expect("the caller peeked the keyword");
        let hint = format!("'(' is expected after '{}'", self.source(&keyword.span));
        match self.next() {
            None => Err(self.unexpected_end(hint)),
            Some(Token {
                tok: Tok::LParen, ..
            }) => Ok(()),
            Some(token) => Err(self.unexpected(token, &hint)),
        }
    }

    fn line(&mut self) -> Result<Expr, QueryError> {
        self.open_group()?;
        let inner = self.or_expr()?;
        self.expect_rparen()?;
        Ok(inner)
    }

    fn expect_comma(&mut self, hint: &str) -> Result<(), QueryError> {
        match self.next() {
            None => Err(self.unexpected_end(hint)),
            Some(Token {
                tok: Tok::Comma, ..
            }) => Ok(()),
            Some(token) => Err(self.unexpected(token, hint)),
        }
    }

    fn near(&mut self) -> Result<Near, QueryError> {
        const SHAPE: &str = "near(A, B, N [, order] [, report])";
        self.open_group()?;
        let a = self.or_expr()?;
        self.expect_comma(&format!("',' is expected: {SHAPE}"))?;
        let b = self.or_expr()?;
        self.expect_comma(&format!("',' is expected: {SHAPE}"))?;
        let distance_hint = format!("a number of lines is expected: {SHAPE}");
        let distance = match self.next() {
            None => return Err(self.unexpected_end(distance_hint)),
            Some(Token {
                tok: Tok::Value(v),
                span,
            }) if v.bytes().all(|b| b.is_ascii_digit()) => Spanned {
                text: v.clone(),
                span: span.clone(),
            },
            Some(token) => return Err(self.unexpected(token, &distance_hint)),
        };
        let mut options = Vec::new();
        loop {
            match self.next() {
                None => return Err(self.unexpected_end("closing ')' is missing")),
                Some(Token {
                    tok: Tok::RParen, ..
                }) => break,
                Some(Token {
                    tok: Tok::Comma, ..
                }) => {
                    let option_hint = "an option is expected: any, ordered, all, closest, first";
                    match self.next() {
                        None => return Err(self.unexpected_end(option_hint)),
                        Some(Token {
                            tok: Tok::Word(w),
                            span,
                        }) => {
                            options.push(Spanned {
                                text: w.clone(),
                                span: span.clone(),
                            });
                        }
                        Some(token) => return Err(self.unexpected(token, option_hint)),
                    }
                }
                Some(token) => return Err(self.unexpected(token, "closing ')' is missing")),
            }
        }
        Ok(Near {
            a: Box::new(a),
            b: Box::new(b),
            distance,
            options,
        })
    }

    fn condition(&mut self, target: Target) -> Result<Condition, QueryError> {
        let target_token = self.next().expect("the caller peeked the target");
        let target_text = self.source(&target_token.span);
        let class = target.class();
        let mut condition = Condition {
            target,
            target_span: target_token.span.clone(),
            op: None,
            value: Value::None,
            modifiers: Vec::new(),
        };
        if class != TargetClass::Alone {
            let op = self.operator(target, target_text)?;
            condition.value = self.value(class, op.clone())?;
            condition.op = Some(op);
        }
        condition.modifiers = self.modifiers()?;
        Ok(condition)
    }

    fn operator(
        &mut self,
        target: Target,
        target_text: &str,
    ) -> Result<(Op, Range<usize>), QueryError> {
        let class = target.class();
        let allowed = Op::for_target(target);
        let shown: &[Op] = if class == TargetClass::Text {
            &Op::TEXT
        } else {
            allowed
        };
        let expected = format!("expected one of: {}", list(shown));
        let Some(token) = self.peek() else {
            return Err(self.unexpected_end(format!(
                "an operator is expected after '{target_text}': {}",
                list(shown)
            )));
        };
        let written = match &token.tok {
            Tok::Op(s) | Tok::Word(s) => s.as_str(),
            _ => {
                let hint = format!(
                    "an operator is expected after '{target_text}': {}",
                    list(shown)
                );
                return Err(self.unexpected(token, &hint));
            }
        };
        let Some(op) = Op::from_symbol(written) else {
            let symbols: Vec<&str> = shown.iter().map(|o| o.symbol()).collect();
            let hint = match did_you_mean(written, &symbols) {
                Some(s) => format!("did you mean '{s}'? {expected}"),
                None => expected,
            };
            return Err(QueryError::new(
                ErrorCode::UnknownOperator,
                format!("unknown operator '{written}'"),
                token.span.clone(),
            )
            .hint(hint));
        };
        if !allowed.contains(&op) {
            let hint = match op {
                Op::Wildcard if target == Target::Text => {
                    "wildcards work on 'path' and 'name'; use 'has'".to_string()
                }
                Op::Fuzzy if target == Target::Text => {
                    "fuzzy matching works on 'path' and 'name'".to_string()
                }
                _ => format!("expected one of: {}", list(allowed)),
            };
            return Err(QueryError::new(
                ErrorCode::OperatorNotAllowed,
                format!("operator '{written}' is not allowed for '{target_text}'"),
                token.span.clone(),
            )
            .hint(hint));
        }
        self.pos += 1;
        Ok((op, token.span.clone()))
    }

    fn value(
        &mut self,
        class: TargetClass,
        (op, op_span): (Op, Range<usize>),
    ) -> Result<Value, QueryError> {
        let op_text = self.source(&op_span);
        match class {
            TargetClass::Text => {
                let hint = format!("a string in quotes is expected after '{op_text}'");
                match self.next() {
                    None => {
                        Err(self.unexpected_end(format!("a string is expected after '{op_text}'")))
                    }
                    Some(Token {
                        tok: Tok::Str { value, quote },
                        span,
                    }) => Ok(Value::Str(StrLit {
                        value: value.clone(),
                        quote: *quote,
                        span: span.clone(),
                    })),
                    Some(token) => Err(self.unexpected(token, &hint)),
                }
            }
            TargetClass::Enum if op == Op::In => {
                let hint = format!("a list is expected after '{op_text}': in (a, b)");
                match self.next() {
                    None => return Err(self.unexpected_end(hint)),
                    Some(Token {
                        tok: Tok::LParen, ..
                    }) => {}
                    Some(token) => return Err(self.unexpected(token, &hint)),
                }
                let mut words = Vec::new();
                loop {
                    match self.next() {
                        None => return Err(self.unexpected_end("a value is expected")),
                        Some(Token {
                            tok: Tok::Word(w),
                            span,
                        }) => {
                            words.push(Spanned {
                                text: w.clone(),
                                span: span.clone(),
                            });
                        }
                        Some(token) => return Err(self.unexpected(token, "a value is expected")),
                    }
                    match self.next() {
                        None => return Err(self.unexpected_end("closing ')' is missing")),
                        Some(Token {
                            tok: Tok::Comma, ..
                        }) => {}
                        Some(Token {
                            tok: Tok::RParen, ..
                        }) => break,
                        Some(token) => return Err(self.unexpected(token, "',' or ')' is expected")),
                    }
                }
                Ok(Value::Words(words))
            }
            TargetClass::Enum => {
                let hint = format!("a value is expected after '{op_text}'");
                match self.next() {
                    None => Err(self.unexpected_end(hint)),
                    Some(Token {
                        tok: Tok::Word(w),
                        span,
                    }) => Ok(Value::Word(Spanned {
                        text: w.clone(),
                        span: span.clone(),
                    })),
                    Some(token) => Err(self.unexpected(token, &hint)),
                }
            }
            TargetClass::Compare if op == Op::In => {
                let hint = format!("a range is expected after '{op_text}': in a..b");
                let from = self.scalar(&hint)?;
                match self.next() {
                    None => return Err(self.unexpected_end(hint)),
                    Some(Token {
                        tok: Tok::Range, ..
                    }) => {}
                    Some(token) => return Err(self.unexpected(token, &hint)),
                }
                let to = self.scalar(&hint)?;
                Ok(Value::Range(from, to))
            }
            TargetClass::Compare => {
                let hint = format!("a value is expected after '{op_text}'");
                Ok(Value::Scalar(self.scalar(&hint)?))
            }
            TargetClass::Alone => Ok(Value::None),
        }
    }

    fn scalar(&mut self, hint: &str) -> Result<Spanned, QueryError> {
        match self.next() {
            None => Err(self.unexpected_end(hint)),
            Some(Token {
                tok: Tok::Value(v),
                span,
            }) => Ok(Spanned {
                text: v.clone(),
                span: span.clone(),
            }),
            Some(token) => Err(self.unexpected(token, hint)),
        }
    }

    fn modifiers(&mut self) -> Result<Vec<Modifier>, QueryError> {
        const KNOWN: &str = "modifiers: /i, /w, /m, /c, /n, /e(style, levels)";
        let mut modifiers = Vec::new();
        while let Some(token) = self.peek() {
            let Tok::Modifier(name) = &token.tok else {
                break;
            };
            let lower = name.to_ascii_lowercase();
            if !["i", "w", "m", "c", "n", "e"].contains(&lower.as_str()) {
                return Err(self.unexpected(token, KNOWN));
            }
            self.pos += 1;
            let mut span = token.span.clone();
            let mut args = None;
            if lower == "e" {
                let shape = "/e(style, levels), e.g. /e(backslash, 0..2)";
                let expect =
                    |parser: &mut Self, ok: fn(&Tok) -> bool| -> Result<&'a Token, QueryError> {
                        match parser.next() {
                            None => Err(parser.unexpected_end(shape)),
                            Some(t) if ok(&t.tok) => Ok(t),
                            Some(t) => Err(parser.unexpected(t, shape)),
                        }
                    };
                expect(self, |t| *t == Tok::LParen)?;
                let style = expect(self, |t| matches!(t, Tok::Word(_)))?;
                expect(self, |t| *t == Tok::Comma)?;
                let first = expect(self, |t| matches!(t, Tok::Word(_) | Tok::Value(_)))?;
                let mut levels_span = first.span.clone();
                if self.peek().is_some_and(|t| t.tok == Tok::Range) {
                    self.pos += 1;
                    levels_span.end = expect(self, |t| matches!(t, Tok::Value(_)))?.span.end;
                }
                let close = expect(self, |t| *t == Tok::RParen)?;
                span.end = close.span.end;
                args = Some((
                    Spanned {
                        text: self.source(&style.span).to_string(),
                        span: style.span.clone(),
                    },
                    Spanned {
                        text: self.source(&levels_span).to_string(),
                        span: levels_span,
                    },
                ));
            }
            modifiers.push(Modifier {
                name: lower,
                args,
                span,
            });
        }
        Ok(modifiers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::lexer::lex;

    fn parse_str(q: &str) -> Result<Option<Expr>, QueryError> {
        parse(q, &lex(q))
    }

    fn err(q: &str) -> (ErrorCode, String, Range<usize>, Option<String>) {
        let e = parse_str(q).unwrap_err();
        (e.code, e.message, e.span, e.hint)
    }

    fn node(e: &Expr) -> &Node {
        match e {
            Expr::Node(n) => n,
            other => panic!("not a node: {other:?}"),
        }
    }

    #[test]
    fn empty_query() {
        assert_eq!(parse_str("   ").unwrap(), None);
    }

    #[test]
    fn simple_condition() {
        let e = parse_str("NAME ~ '*.rs'").unwrap().unwrap();
        let n = node(&e);
        let NodeKind::Condition(c) = &n.kind else {
            panic!()
        };
        assert_eq!(c.target, Target::Name);
        assert_eq!(c.op.as_ref().unwrap().0, Op::Wildcard);
        assert!(matches!(&c.value, Value::Str(s) if s.value == "*.rs"));
        assert_eq!(n.tokens, 0..3);
    }

    #[test]
    fn precedence() {
        let e = parse_str("name = 'a' and name = 'b' or not name = 'c' xor name = 'd'")
            .unwrap()
            .unwrap();
        let Expr::Or(items) = e else { panic!() };
        assert!(matches!(items[0], Expr::And(_)));
        let Expr::Xor(x) = &items[1] else { panic!() };
        assert!(matches!(x[0], Expr::Not(_)));
    }

    #[test]
    fn full_grammar_parses() {
        for q in [
            "type = dir and empty",
            "size in 10K..1M and modified > -7d",
            "kind in (source, test) @k",
            "line(text has 'a' /i/w @x and not len > 3) @l",
            "near(text has 'a', text has 'b', 3, ordered, closest)",
            "text has `\"a\"` /e(backslash, 0..2) /c",
            "text has 'a' /e(xml, any)",
            "(name ~ '*.rs' or name ~ '*.go') and git = changed",
        ] {
            assert!(parse_str(q).is_ok(), "{q}: {:?}", parse_str(q));
        }
    }

    #[test]
    fn unexpected_token() {
        assert_eq!(
            err("name ~ '*.rs' 'x'"),
            (
                ErrorCode::UnexpectedToken,
                "unexpected 'x'".into(),
                14..17,
                Some(MISSING_AND.into())
            )
        );
        assert_eq!(err("name ~ 'a' text has 'x'").1, "unexpected 'text'");
        assert_eq!(err("name ~ 'a')").3.unwrap(), "unmatched ')'");
        assert_eq!(err("a && b").0, ErrorCode::UnknownTarget);
        assert_eq!(
            err("name = 'a' @1x"),
            (
                ErrorCode::UnexpectedToken,
                "unexpected '@'".into(),
                11..12,
                Some("a label starts with a letter or '_', e.g. @name".into())
            )
        );
        assert_eq!(
            err("name = 'a' && name = 'b'").3.unwrap(),
            "use 'and' instead of '&&'"
        );
        assert_eq!(err("and name = 'a'").3.unwrap(), "a condition is expected");
        assert_eq!(
            err("name = foo").3.unwrap(),
            "a string in quotes is expected after '='"
        );
        assert_eq!(
            err("text has 'a' /x").3.unwrap(),
            "modifiers: /i, /w, /m, /c, /n, /e(style, levels)"
        );
    }

    #[test]
    fn unexpected_end() {
        assert_eq!(
            err("name ~"),
            (
                ErrorCode::UnexpectedEnd,
                "unexpected end of the query".into(),
                6..6,
                Some("a string is expected after '~'".into())
            )
        );
        assert_eq!(
            err("name ~ '*.rs' and").3.unwrap(),
            "a condition is expected after 'and'"
        );
        assert_eq!(err("(name ~ 'a'").3.unwrap(), "closing ')' is missing");
        assert_eq!(
            err("name").3.unwrap(),
            "an operator is expected after 'name': =, has, starts, ends, ~, ~~, =~"
        );
    }

    #[test]
    fn unknown_target() {
        assert_eq!(
            err("filename = 'a.rs'"),
            (
                ErrorCode::UnknownTarget,
                "unknown target 'filename'".into(),
                0..8,
                Some(
                    "did you mean 'name'? expected one of: path, name, type, size, modified, created, accessed, empty, lang, kind, git, text, len"
                        .into()
                )
            )
        );
    }

    #[test]
    fn unknown_operator() {
        assert_eq!(
            err("name contains 'a'"),
            (
                ErrorCode::UnknownOperator,
                "unknown operator 'contains'".into(),
                5..13,
                Some("did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~".into())
            )
        );
        assert_eq!(
            err("size == 1").3.unwrap(),
            "did you mean '='? expected one of: =, !=, <, <=, >, >=, in"
        );
    }

    #[test]
    fn operator_not_allowed() {
        let (code, message, _, hint) = err("text ~ '*foo*'");
        assert_eq!(code, ErrorCode::OperatorNotAllowed);
        assert_eq!(message, "operator '~' is not allowed for 'text'");
        assert_eq!(
            hint.unwrap(),
            "wildcards work on 'path' and 'name'; use 'has'"
        );
    }

    #[test]
    fn unterminated_string() {
        assert_eq!(
            err("name = 'abc"),
            (
                ErrorCode::UnterminatedString,
                "unterminated string".into(),
                7..11,
                Some("closing ' is missing".into())
            )
        );
        assert_eq!(err("name = `a").3.unwrap(), "closing ` is missing");
    }

    #[test]
    fn labels() {
        let e = parse_str("name = 'a' @x").unwrap().unwrap();
        assert_eq!(node(&e).label.as_ref().unwrap().text, "x");
        assert_eq!(node(&e).tokens, 0..3);
    }
}
