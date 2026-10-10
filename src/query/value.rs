//! Values of the comparison targets: sizes (QUERY.md#sizes) and the intervals of the comparison operators.

use super::ast::Op;
use super::eval::Bounds;

/// A size in bytes: `integer [ "." integer ] [ B | K | M | G ]`, units in any case, powers of 1024.
pub fn parse_size(text: &str) -> Option<f64> {
    let (number, unit) = match text.char_indices().last()? {
        (i, c) if c.is_ascii_alphabetic() => (&text[..i], Some(c.to_ascii_uppercase())),
        _ => (text, None),
    };
    let factor = match unit {
        None | Some('B') => 1.0,
        Some('K') => 1024.0,
        Some('M') => 1024.0 * 1024.0,
        Some('G') => 1024.0 * 1024.0 * 1024.0,
        Some(_) => return None,
    };
    let (int, frac) = number.split_once('.').unwrap_or((number, "0"));
    if int.is_empty()
        || frac.is_empty()
        || !(int.bytes().chain(frac.bytes()).all(|b| b.is_ascii_digit()))
    {
        return None;
    }
    number.parse::<f64>().ok().map(|n| n * factor)
}

/// The interval of `<op> v`, or of `in from..to` when `to` is given.
pub fn bounds(op: Op, from: f64, to: Option<f64>) -> Bounds {
    let point = |negated| Bounds {
        min: Some((from, true)),
        max: Some((from, true)),
        negated,
    };
    match (op, to) {
        (Op::In, Some(to)) => Bounds {
            min: Some((from, true)),
            max: Some((to, true)),
            negated: false,
        },
        (Op::Ne, _) => point(true),
        (Op::Lt, _) => Bounds {
            min: None,
            max: Some((from, false)),
            negated: false,
        },
        (Op::Le, _) => Bounds {
            min: None,
            max: Some((from, true)),
            negated: false,
        },
        (Op::Gt, _) => Bounds {
            min: Some((from, false)),
            max: None,
            negated: false,
        },
        (Op::Ge, _) => Bounds {
            min: Some((from, true)),
            max: None,
            negated: false,
        },
        _ => point(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes() {
        assert_eq!(parse_size("0"), Some(0.0));
        assert_eq!(parse_size("512"), Some(512.0));
        assert_eq!(parse_size("512B"), Some(512.0));
        assert_eq!(parse_size("10K"), Some(10240.0));
        assert_eq!(parse_size("10k"), Some(10240.0));
        assert_eq!(parse_size("1.5M"), Some(1_572_864.0));
        assert_eq!(parse_size("2G"), Some(2_147_483_648.0));
        for bad in ["10MB", "1.M", ".5K", "1.5.2", "-1K", "1T", "K", ""] {
            assert_eq!(parse_size(bad), None, "{bad}");
        }
    }

    #[test]
    fn intervals() {
        let b = |op, v| bounds(op, v, None);
        assert!(b(Op::Eq, 5.0).contains(5.0) && !b(Op::Eq, 5.0).contains(6.0));
        assert!(!b(Op::Ne, 5.0).contains(5.0) && b(Op::Ne, 5.0).contains(6.0));
        assert!(b(Op::Lt, 5.0).contains(4.0) && !b(Op::Lt, 5.0).contains(5.0));
        assert!(b(Op::Le, 5.0).contains(5.0) && !b(Op::Le, 5.0).contains(6.0));
        assert!(b(Op::Gt, 5.0).contains(6.0) && !b(Op::Gt, 5.0).contains(5.0));
        assert!(b(Op::Ge, 5.0).contains(5.0) && !b(Op::Ge, 5.0).contains(4.0));
        let range = bounds(Op::In, 1.0, Some(3.0));
        assert!(range.contains(1.0) && range.contains(3.0) && !range.contains(3.5));
    }
}
