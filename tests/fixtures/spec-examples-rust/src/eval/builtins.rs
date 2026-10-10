//! The functions every script can call.

use super::Value;
use crate::error::{Error, Result};

pub const NAMES: [&str; 12] = ["len", "upper", "lower", "trim", "abs", "floor", "ceil", "sqrt", "min", "max", "pow", "concat"];

/// Calls a builtin function.
pub fn call(name: &str, args: &[Value]) -> Result<Value> {
    use Value::{Number, Str};
    Ok(match (name, args) {
        ("len", [Str(s)]) => Number(s.chars().count() as f64),
        ("upper", [Str(s)]) => Str(s.to_uppercase()),
        ("lower", [Str(s)]) => Str(s.to_lowercase()),
        ("trim", [Str(s)]) => Str(s.trim().to_string()),
        ("abs", [Number(x)]) => Number(x.abs()),
        ("floor", [Number(x)]) => Number(x.floor()),
        ("ceil", [Number(x)]) => Number(x.ceil()),
        ("sqrt", [Number(x)]) => Number(x.sqrt()),
        ("min", [Number(a), Number(b)]) => Number(a.min(*b)),
        ("max", [Number(a), Number(b)]) => Number(a.max(*b)),
        ("pow", [Number(a), Number(b)]) => Number(a.powf(*b)),
        ("concat", [Str(a), Str(b)]) => Str(format!("{a}{b}")),
        _ => return Err(Error::Type(format!("wrong arguments for {name}"))),
    })
}
