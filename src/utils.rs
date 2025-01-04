use chrono::{DateTime, SecondsFormat, Utc};
use std::fmt::Write;

pub fn bits64(value: u64, offset: u8, length: u8) -> u64 {
    value << offset >> (64 - length)
}

pub fn bits128(value: u128, offset: u8, length: u8) -> u128 {
    value << offset >> (128 - length)
}

pub fn milliseconds_to_seconds_and_iso8601(ms: u64, epoch: Option<u64>) -> (String, String) {
    let timestamp: f64 = (ms + epoch.unwrap_or(0)) as f64 / 1_000.0;
    let secs = timestamp.trunc() as i64;
    let nanos = (timestamp.fract() * 1_000.0).round() as u32 * 1_000_000;
    match DateTime::from_timestamp(secs, nanos) {
        Some(dt) => {
            let datetime = dt.to_rfc3339_opts(SecondsFormat::Millis, true);
            (format!("{:.3}", timestamp), datetime.to_string())
        }
        None => (format!("{:.3}", timestamp), "Invalid".to_string()),
    }
}

pub fn nanoseconds_to_iso8601(ns: u64) -> (String, String) {
    let timestamp: f64 = ns as f64 / 1_000_000_000.0;
    let secs = timestamp.trunc() as i64;
    let nanos: u32 = timestamp.fract().round() as u32;
    let dt: DateTime<Utc> = DateTime::from_timestamp(secs, nanos).unwrap();
    let datetime = dt.to_rfc3339_opts(SecondsFormat::Millis, true);
    (format!("{:.3}", timestamp), datetime.to_string())
}

pub fn factor_size_hex_bits_color_from_text(text: &str) -> (u16, Option<String>, Option<String>, Option<String>) {
    (
        (text.chars().count() * 8) as u16,
        Some(hex::encode(text.as_bytes())),
        Some(text.as_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        Some((0..(text.chars().count() * 8)).map(|_| "2").collect::<String>()),
    )
}

#[cfg(test)]
mod bits64 {
    use super::*;

    #[test]
    fn test_bits64_first_bit() {
        assert_eq!(bits64(0u64, 0, 1), 0);
        assert_eq!(bits64(u64::pow(2, 63), 0, 1), 1);
    }

    #[test]
    fn test_bits64_last_bit() {
        assert_eq!(bits64(0u64, 63, 1), 0);
        assert_eq!(bits64(1u64, 63, 1), 1);
    }

    #[test]
    fn test_bits64_middle_bits() {
        assert_eq!(bits64(6442450944u64, 31, 2), 3); // All zeros with 2 ones in the middle.
        assert_eq!(bits64(18446744067267100671u64, 31, 2), 0); // All ones with 2 zeros in the middle.
    }

    #[test]
    fn test_bits64_halfs() {
        assert_eq!(bits64(u64::MAX, 0, 32), u32::MAX as u64);
        assert_eq!(bits64(u64::MAX, 32, 32), u32::MAX as u64);
    }

    #[test]
    fn test_time_formats_unix_epoch() {
        let (ts, dt) = milliseconds_to_seconds_and_iso8601(1420070400000, None);
        assert_eq!(ts, "1420070400.000");
        assert_eq!(dt, "2015-01-01T00:00:00.000Z");
    }

    #[test]
    fn test_time_formats_custom_epoch() {
        let (ts, dt) = milliseconds_to_seconds_and_iso8601(0, Some(1420070400000));
        assert_eq!(ts, "1420070400.000");
        assert_eq!(dt, "2015-01-01T00:00:00.000Z");
    }

    #[test]
    fn test_time_formats_epochalypse() {
        let (ts, dt) = milliseconds_to_seconds_and_iso8601(u64::pow(2, 31) * 1000, None);
        assert_eq!(ts, "2147483648.000");
        assert_eq!(dt, "2038-01-19T03:14:08.000Z");
    }

    #[test]
    fn test_time_formats_far_future() {
        let (ts, dt) = milliseconds_to_seconds_and_iso8601(99999999999000, None);
        assert_eq!(ts, "99999999999.000");
        assert_eq!(dt, "5138-11-16T09:46:39.000Z");
        let (ts, dt) = milliseconds_to_seconds_and_iso8601(281474976710655, None);
        assert_eq!(ts, "281474976710.655");
        assert_eq!(dt, "+10889-08-02T05:31:50.655Z");
    }

    #[test]
    fn test_invalid_timestamp() {
        let (ts, dt) = milliseconds_to_seconds_and_iso8601(16477688742971526000, None);
        assert_eq!(ts, "16477688742971526.000");
        assert_eq!(dt, "Invalid");
    }

    #[test]
    fn test_nanoseconds_to_iso8601() {
        let (ts, dt) = nanoseconds_to_iso8601(1734971723000000000);
        assert_eq!(ts, "1734971723.000");
        assert_eq!(dt, "2024-12-23T16:35:23.000Z");
    }

    #[test]
    fn test_factor_size_hex_bits_color_from_text() {
        let (size, hex, bits, color_map) = factor_size_hex_bits_color_from_text("Hello");
        assert_eq!(size, 40);
        assert_eq!(hex, Some("48656c6c6f".to_string()));
        assert_eq!(bits, Some("0100100001100101011011000110110001101111".to_string()));
        assert_eq!(color_map, Some("2222222222222222222222222222222222222222".to_string()));
    }
}
