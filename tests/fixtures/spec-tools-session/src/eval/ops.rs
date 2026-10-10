//! The comparison operators.

use regex::Regex;

/// `text =~ pattern`
pub fn matches(text: &str, pattern: &str) -> bool {
    Regex::new(pattern).unwrap().is_match(text)
}

/// `size > n` with a unit: `10K`, `5M`.
pub fn size(value: &str) -> u64 {
    let (number, unit) = value.split_at(value.len() - 1);
    let factor = match unit {
        "K" => 1024,
        "M" => 1024 * 1024,
        _ => return value.parse().unwrap(),
    };
    number.parse::<u64>().unwrap() * factor
}
