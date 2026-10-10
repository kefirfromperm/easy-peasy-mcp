//! Evaluates the expressions of the parser.

use crate::parser::ast::Expr;

/// The value of an expression.
pub fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Num(n) => *n,
    }
}

/// The value of a number written as text.
pub fn eval_text(text: &str) -> f64 {
    let n: f64 = text.trim().parse().unwrap();
    n
}
