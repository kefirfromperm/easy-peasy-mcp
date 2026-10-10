//! Values of the comparison targets: sizes (QUERY.md#sizes), dates (QUERY.md#dates) and the intervals of the
//! comparison operators.

use std::sync::LazyLock;

use jiff::civil::{Date, Time};
use jiff::tz::{Offset, TimeZone};
use jiff::{SignedDuration, Timestamp};
use regex::Regex;

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

/// A date of a query as an interval of seconds since the Unix epoch: `[start, end)`, or the moment `start` when
/// `start == end`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateValue {
    pub start: f64,
    pub end: f64,
}

static ABSOLUTE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(\d{4})-(\d{2})-(\d{2})(?:T(\d{2}):(\d{2})(?::(\d{2})(\.\d{1,9})?)?(Z|[+-]\d{2}:\d{2})?)?$")
        .expect("a valid regex")
});

/// Seconds since the Unix epoch, with the fraction.
pub fn seconds(ts: Timestamp) -> f64 {
    #[allow(clippy::cast_precision_loss)] // seconds of this era fit into 53 bits
    let whole = ts.as_second() as f64;
    whole + f64::from(ts.subsec_nanosecond()) / 1e9
}

/// Whether `text` is a relative date: `-` and a digit.
pub fn is_relative(text: &str) -> bool {
    text.starts_with('-')
}

/// Parses a date of the query: ISO 8601 in the local zone `tz` unless an offset is given, or `-N` with a unit
/// relative to `now`.
pub fn parse_date(text: &str, now: Timestamp, tz: &TimeZone) -> Option<DateValue> {
    if is_relative(text) {
        let digits = &text[1..text.len().checked_sub(1)?];
        let n: i64 = digits
            .parse()
            .ok()
            .filter(|_| digits.bytes().all(|b| b.is_ascii_digit()))?;
        let unit: i64 = match text.chars().last()? {
            's' => 1,
            'm' => 60,
            'h' => 3600,
            'd' => 86_400,
            'w' => 604_800,
            _ => return None,
        };
        let moment = now
            .checked_sub(SignedDuration::from_secs(n.checked_mul(unit)?))
            .ok()?;
        let t = seconds(moment);
        return Some(DateValue { start: t, end: t });
    }
    let caps = ABSOLUTE.captures(text)?;
    let num = |i: usize| {
        caps.get(i)
            .map(|m| m.as_str().parse::<i32>().expect("digits"))
    };
    let date = Date::new(
        i16::try_from(num(1)?).ok()?,
        i8::try_from(num(2)?).ok()?,
        i8::try_from(num(3)?).ok()?,
    )
    .ok()?;
    let zone = match caps.get(8).map(|m| m.as_str()) {
        None => tz.clone(),
        Some("Z") => TimeZone::UTC,
        Some(offset) => {
            let sign = if offset.starts_with('-') { -1 } else { 1 };
            let hours: i32 = offset[1..3].parse().ok()?;
            let minutes: i32 = offset[4..6].parse().ok()?;
            TimeZone::fixed(Offset::from_seconds(sign * (hours * 3600 + minutes * 60)).ok()?)
        }
    };
    let Some(hour) = num(4) else {
        let start = seconds(date.to_zoned(zone.clone()).ok()?.timestamp());
        let end = seconds(date.tomorrow().ok()?.to_zoned(zone).ok()?.timestamp());
        return Some(DateValue { start, end });
    };
    let fraction = caps.get(7).map_or(Some(0), |m| {
        format!("{:0<9}", &m.as_str()[1..]).parse::<i32>().ok()
    })?;
    let time = Time::new(
        i8::try_from(hour).ok()?,
        i8::try_from(num(5)?).ok()?,
        i8::try_from(num(6).unwrap_or(0)).ok()?,
        fraction,
    )
    .ok()?;
    let start = seconds(date.to_datetime(time).to_zoned(zone).ok()?.timestamp());
    let end = match (caps.get(6), caps.get(7)) {
        (None, _) => start + 60.0,
        (Some(_), None) => start + 1.0,
        (Some(_), Some(_)) => start,
    };
    Some(DateValue { start, end })
}

/// The interval of `<op> a`, or of `in a..b` when `b` is given (QUERY.md#dates).
pub fn date_bounds(op: Op, a: DateValue, b: Option<DateValue>) -> Bounds {
    let moment = a.start == a.end;
    let bounds = |min, max| Bounds {
        min,
        max,
        negated: false,
    };
    match (op, b) {
        (Op::In, Some(b)) => bounds(Some((a.start, true)), Some((b.end, b.start == b.end))),
        (Op::Ne, _) => Bounds {
            negated: true,
            ..date_bounds(Op::Eq, a, None)
        },
        (Op::Gt, _) => bounds(Some((a.end, !moment)), None),
        (Op::Ge, _) => bounds(Some((a.start, true)), None),
        (Op::Lt, _) => bounds(None, Some((a.start, false))),
        (Op::Le, _) => bounds(None, Some((a.end, moment))),
        _ => bounds(Some((a.start, true)), Some((a.end, moment))),
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

    fn now() -> Timestamp {
        "2026-10-08T12:00:00Z".parse().unwrap()
    }

    fn ts(s: &str) -> f64 {
        seconds(s.parse().unwrap())
    }

    fn date(text: &str) -> Option<DateValue> {
        parse_date(text, now(), &TimeZone::UTC)
    }

    #[test]
    fn absolute_dates() {
        let day = date("2026-10-01").unwrap();
        assert_eq!(
            (day.start, day.end),
            (ts("2026-10-01T00:00:00Z"), ts("2026-10-02T00:00:00Z"))
        );
        let minute = date("2026-10-01T14:30").unwrap();
        assert_eq!(
            (minute.start, minute.end - minute.start),
            (ts("2026-10-01T14:30:00Z"), 60.0)
        );
        let second = date("2026-10-01T14:30:05Z").unwrap();
        assert_eq!(second.end - second.start, 1.0);
        let offset = date("2026-10-01T14:30+02:00").unwrap();
        assert_eq!(offset.start, ts("2026-10-01T12:30:00Z"));
        let fraction = date("2026-10-01T14:30:05.5Z").unwrap();
        assert_eq!(
            (fraction.start, fraction.end),
            (ts("2026-10-01T14:30:05.5Z"), fraction.start)
        );
    }

    #[test]
    fn local_days_follow_daylight_saving() {
        let berlin = TimeZone::get("Europe/Berlin").unwrap();
        let day = parse_date("2026-10-25", now(), &berlin).unwrap();
        assert_eq!(day.start, ts("2026-10-24T22:00:00Z"));
        assert_eq!(day.end - day.start, 25.0 * 3600.0);
    }

    #[test]
    fn relative_dates() {
        let week = date("-1w").unwrap();
        assert_eq!(
            (week.start, week.end),
            (ts("2026-10-01T12:00:00Z"), ts("2026-10-01T12:00:00Z"))
        );
        assert_eq!(date("-30m").unwrap().start, ts("2026-10-08T11:30:00Z"));
        assert_eq!(date("-2h").unwrap().start, ts("2026-10-08T10:00:00Z"));
        assert_eq!(date("-10s").unwrap().start, ts("2026-10-08T11:59:50Z"));
    }

    #[test]
    fn invalid_dates() {
        for bad in [
            "-1y",
            "-d",
            "-1.5d",
            "2026-13-01",
            "2026-02-30",
            "2026-10-1",
            "2026-10-01T25:00",
            "2026-10-01Z",
            "1d",
        ] {
            assert_eq!(date(bad), None, "{bad}");
        }
    }

    #[test]
    fn date_intervals() {
        let day = date("2026-10-01").unwrap();
        let (start, end) = (day.start, day.end);
        let b = |op| date_bounds(op, day, None);
        assert!(b(Op::Eq).contains(start) && !b(Op::Eq).contains(end));
        assert!(!b(Op::Ne).contains(start) && b(Op::Ne).contains(end));
        assert!(b(Op::Gt).contains(end) && !b(Op::Gt).contains(end - 1.0));
        assert!(b(Op::Ge).contains(start) && !b(Op::Ge).contains(start - 1.0));
        assert!(b(Op::Lt).contains(start - 1.0) && !b(Op::Lt).contains(start));
        assert!(b(Op::Le).contains(end - 1.0) && !b(Op::Le).contains(end));
        let moment = date("-7d").unwrap();
        let m = |op| date_bounds(op, moment, None);
        assert!(!m(Op::Gt).contains(moment.start) && m(Op::Gt).contains(moment.start + 1.0));
        assert!(m(Op::Le).contains(moment.start));
        let range = date_bounds(Op::In, moment, Some(date("-3d").unwrap()));
        assert!(range.contains(moment.start) && range.contains(date("-3d").unwrap().start));
    }
}
