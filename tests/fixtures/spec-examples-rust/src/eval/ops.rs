//! The operators of the language.

use super::Value;
use crate::error::{Error, Result};
use crate::parser::BinOp;

/// `false` and `nil` are false, everything else is true.
pub fn truthy(value: &Value) -> bool {
    !matches!(value, Value::Nil | Value::Bool(false))
}

pub fn binary(op: BinOp, left: Value, right: Value) -> Result<Value> {
    use Value::{Bool, Number, Str};
    Ok(match (op, left, right) {
        (BinOp::Add, Number(a), Number(b)) => Number(a + b),
        (BinOp::Sub, Number(a), Number(b)) => Number(a - b),
        (BinOp::Mul, Number(a), Number(b)) => Number(a * b),
        (BinOp::Div, Number(a), Number(b)) => Number(a / b),
        (BinOp::Rem, Number(a), Number(b)) => Number(a % b),
        (BinOp::Add, Str(a), Str(b)) => Str(a + &b),
        (BinOp::Lt, Number(a), Number(b)) => Bool(a < b),
        (BinOp::Le, Number(a), Number(b)) => Bool(a <= b),
        (BinOp::Gt, Number(a), Number(b)) => Bool(a > b),
        (BinOp::Ge, Number(a), Number(b)) => Bool(a >= b),
        (BinOp::Eq, a, b) => Bool(a == b),
        (BinOp::Ne, a, b) => Bool(a != b),
        (BinOp::And, a, b) => if truthy(&a) { b } else { a },
        (BinOp::Or, a, b) => if truthy(&a) { a } else { b },
        (op, a, b) => return Err(Error::Type(format!("can't apply {op:?} to {a} and {b}"))),
    })
}
