//! The metadata of the files in the output (OUTPUT.md#meta).

use jiff::tz::TimeZone;

use crate::search::Meta;

/// A size in the text format: bytes under 1K, otherwise one decimal in the largest unit of 1024 it is at least 1 of.
pub fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 3] = ["K", "M", "G"];
    if bytes < 1024 {
        return format!("{bytes}B");
    }
    #[allow(clippy::cast_precision_loss)] // one decimal is shown
    let mut value = bytes as f64 / 1024.0;
    let mut unit = 0;
    // a value which is shown as 1024.0 is 1.0 of the next unit
    while unit + 1 < UNITS.len() && (value * 10.0).round() >= 10240.0 {
        value /= 1024.0;
        unit += 1;
    }
    format!("{value:.1}{}", UNITS[unit])
}

/// A date in the text format: `2026-10-08 14:03` in the time zone.
pub fn date_text(seconds: f64, time_zone: &TimeZone) -> String {
    zoned(seconds, time_zone).map_or_else(String::new, |z| z.strftime("%Y-%m-%d %H:%M").to_string())
}

/// A date in JSON: ISO 8601 with the offset of the time zone, whole seconds.
pub fn date_json(seconds: f64, time_zone: &TimeZone) -> String {
    zoned(seconds, time_zone).map_or_else(String::new, |z| {
        z.strftime("%Y-%m-%dT%H:%M:%S%:z").to_string()
    })
}

/// The date at the whole second.
#[allow(clippy::cast_possible_truncation)] // dates are far from the limits of i64
fn zoned(seconds: f64, time_zone: &TimeZone) -> Option<jiff::Zoned> {
    jiff::Timestamp::from_second(seconds.floor() as i64)
        .ok()
        .map(|t| t.to_zoned(time_zone.clone()))
}

/// The values in the text format: `file 2.1K 2026-10-08 14:03 rust source`, the missing ones left out.
pub fn text(meta: &Meta, time_zone: &TimeZone) -> String {
    let mut parts = vec![meta.entry_type.keyword().to_string()];
    parts.extend(meta.size.map(human_size));
    #[allow(clippy::cast_precision_loss)] // whole seconds of dates are exact
    parts.extend(meta.modified.map(|m| date_text(m as f64, time_zone)));
    parts.extend(meta.lang.clone());
    parts.extend(meta.kind.map(|k| k.keyword().to_string()));
    parts.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_in_units_of_1024() {
        let k = 1024_u64;
        for (bytes, text) in [
            (0, "0B"),
            (512, "512B"),
            (k - 1, "1023B"),
            (k, "1.0K"),
            (2150, "2.1K"),
            (k * k - 1, "1.0M"),
            (k * k * 3 / 2, "1.5M"),
            (3 * k * k * k, "3.0G"),
            (5000 * k * k * k, "5000.0G"),
        ] {
            assert_eq!(human_size(bytes), text, "{bytes}");
        }
    }

    #[test]
    fn dates_in_the_time_zone() {
        let utc = jiff::tz::TimeZone::UTC;
        // 2026-10-08 14:03:59.5 UTC
        let secs = 1_791_468_239.5;
        assert_eq!(date_text(secs, &utc), "2026-10-08 14:03");
        assert_eq!(date_json(secs, &utc), "2026-10-08T14:03:59+00:00");
        let plus_two = jiff::tz::TimeZone::fixed(jiff::tz::offset(2));
        assert_eq!(date_json(secs, &plus_two), "2026-10-08T16:03:59+02:00");
    }
}
