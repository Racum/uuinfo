//! Minimal UTC date/time helpers, std-only (replaces chrono).
//!
//! uuinfo only needs "now" and Unix time -> RFC 3339 formatting, always in
//! UTC, so there is no time zone, parsing or leap-second handling here.

use std::time::{SystemTime, UNIX_EPOCH};

pub const MS_PER_DAY: i64 = 86_400_000;
const SECS_PER_DAY: i64 = 86_400;

/// Year range supported by the formatter. Same bounds as chrono, so IDs
/// that used to render as "Invalid" still do.
const MIN_YEAR: i64 = -262_144;
const MAX_YEAR: i64 = 262_143;

/// Milliseconds since the Unix epoch (negative if the clock is before 1970).
pub fn now_ms() -> i64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_millis() as i64,
        Err(e) => -(e.duration().as_millis() as i64),
    }
}

/// Seconds since the Unix epoch.
pub fn now_secs() -> i64 {
    now_ms().div_euclid(1000)
}

/// Current year, in UTC.
pub fn current_year() -> i64 {
    civil_from_days(now_ms().div_euclid(MS_PER_DAY)).0
}

/// Converts days since 1970-01-01 to a (year, month, day) in the proleptic
/// Gregorian calendar. Howard Hinnant's `civil_from_days` algorithm:
/// https://howardhinnant.github.io/date_algorithms.html#civil_from_days
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468; // Shift epoch to 0000-03-01.
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097); // Day of era: [0, 146096]
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365; // Year of era: [0, 399]
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // Day of year, from March 1st: [0, 365]
    let mp = (5 * doy + 2) / 153; // Month, from March: [0, 11]
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let year = yoe + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

/// Formats a Unix timestamp as RFC 3339 in UTC ("Z"), with `digits` (0-9)
/// fractional digits. Years outside 0000-9999 get an explicit sign, like
/// chrono. Returns `None` when the year is out of the supported range.
pub fn format_rfc3339(secs: i64, nanos: u32, digits: u32) -> Option<String> {
    if nanos >= 1_000_000_000 || digits > 9 {
        return None;
    }
    let days = secs.div_euclid(SECS_PER_DAY);
    let sod = secs.rem_euclid(SECS_PER_DAY);
    let (year, month, day) = civil_from_days(days);
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return None;
    }
    let year = match year {
        0..=9999 => format!("{year:04}"),
        _ => format!("{year:+05}"),
    };
    let (h, m, s) = (sod / 3600, sod % 3600 / 60, sod % 60);
    let frac = match digits {
        0 => String::new(),
        _ => format!(".{:0width$}", nanos / 10u32.pow(9 - digits), width = digits as usize),
    };
    Some(format!("{year}-{month:02}-{day:02}T{h:02}:{m:02}:{s:02}{frac}Z"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_from_days() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(-1), (1969, 12, 31));
        assert_eq!(civil_from_days(11_016), (2000, 2, 29));
        assert_eq!(civil_from_days(11_017), (2000, 3, 1));
        assert_eq!(civil_from_days(-25_567), (1900, 1, 1));
        assert_eq!(civil_from_days(-719_468), (0, 3, 1));
    }

    #[test]
    fn test_format_rfc3339() {
        assert_eq!(format_rfc3339(0, 0, 3).as_deref(), Some("1970-01-01T00:00:00.000Z"));
        assert_eq!(format_rfc3339(-1, 0, 0).as_deref(), Some("1969-12-31T23:59:59Z"));
        assert_eq!(format_rfc3339(951_782_400, 123_456_789, 9).as_deref(), Some("2000-02-29T00:00:00.123456789Z"));
        assert_eq!(format_rfc3339(281_474_976_710, 655_000_000, 3).as_deref(), Some("+10889-08-02T05:31:50.655Z"));
        assert_eq!(format_rfc3339(-62_198_755_200, 0, 0).as_deref(), Some("-0001-01-01T00:00:00Z"));
    }

    #[test]
    fn test_format_rfc3339_out_of_range() {
        assert_eq!(format_rfc3339(8_210_298_412_799, 0, 3).as_deref(), Some("+262143-12-31T23:59:59.000Z"));
        assert_eq!(format_rfc3339(8_210_298_412_800, 0, 3), None);
        assert_eq!(format_rfc3339(16_477_688_742_971_526, 0, 3), None);
        assert_eq!(format_rfc3339(0, 1_000_000_000, 3), None);
    }

    #[test]
    fn test_now() {
        assert!(now_ms() > 1_700_000_000_000);
        assert!(now_secs() > 1_700_000_000);
        assert!(current_year() >= 2025);
    }
}
