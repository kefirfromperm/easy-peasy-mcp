//! The metadata of the files in the output (OUTPUT.md#meta).

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
