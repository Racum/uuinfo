use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::schema::{Args, IDInfo};
use crate::utils::nanoseconds_to_iso8601;

const RECENT_DAYS_AGO: u64 = 365 * 10;
const RECENT_DAYS_AHEAD: u64 = 365;

#[derive(PartialEq)]
enum TimestampUnitAssumption {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
    Recent,
    Unknown,
}

fn parse_unix_core(args: &Args, assumption: TimestampUnitAssumption) -> Option<IDInfo> {
    let id_int: u64 = args.id.trim().parse::<u64>().ok()?;
    let (timestamp, datetime): (String, String);
    let version: String;
    let timestamp_ns: u64;

    match assumption {
        TimestampUnitAssumption::Unknown | TimestampUnitAssumption::Recent => {
            match id_int {
                0..100_000_000_000 => {
                    timestamp_ns = id_int * 1_000_000_000;
                    version = "Assuming seconds".to_string();
                }
                100_000_000_000..100_000_000_000_000 => {
                    timestamp_ns = id_int * 1_000_000;
                    version = "Assuming milliseconds".to_string();
                }
                100_000_000_000_000..10_000_000_000_000_000 => {
                    timestamp_ns = id_int * 1_000;
                    version = "Assuming microseconds".to_string();
                }
                10_000_000_000_000_000..u64::MAX | u64::MAX => {
                    timestamp_ns = id_int;
                    version = "Assuming nanoseconds".to_string();
                }
            }
            (timestamp, datetime) = nanoseconds_to_iso8601(timestamp_ns);

            if assumption == TimestampUnitAssumption::Recent {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
                let start: u64 = now - (1_000_000_000 * 60 * 60 * 24 * RECENT_DAYS_AGO);
                let end: u64 = now + (1_000_000_000 * 60 * 60 * 24 * RECENT_DAYS_AHEAD);
                if !(start < timestamp_ns && timestamp_ns < end) {
                    return None;
                }
            }
        }
        TimestampUnitAssumption::Seconds => match id_int.checked_mul(1_000_000_000) {
            Some(value) => {
                (timestamp, datetime) = nanoseconds_to_iso8601(value);
                version = "As seconds".to_string();
            }
            None => return None,
        },
        TimestampUnitAssumption::Milliseconds => match id_int.checked_mul(1_000_000) {
            Some(value) => {
                (timestamp, datetime) = nanoseconds_to_iso8601(value);
                version = "As milliseconds".to_string();
            }
            None => return None,
        },
        TimestampUnitAssumption::Microseconds => match id_int.checked_mul(1_000) {
            Some(value) => {
                (timestamp, datetime) = nanoseconds_to_iso8601(value);
                version = "As microseconds".to_string();
            }
            None => return None,
        },
        TimestampUnitAssumption::Nanoseconds => {
            version = "As nanoseconds".to_string();
            (timestamp, datetime) = nanoseconds_to_iso8601(id_int);
        }
    }

    Some(IDInfo {
        id_type: "Unix timestamp".to_string(),
        version: Some(version),
        standard: args.id.to_string(),
        integer: Some(id_int as u128),
        size: 64,
        parsed: Some("as integer".to_string()),
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("3333333333333333333333333333333333333333333333333333333333333333".to_string()),
        ..Default::default()
    })
}

pub fn parse_unix(args: &Args) -> Option<IDInfo> {
    parse_unix_core(args, TimestampUnitAssumption::Unknown)
}

pub fn parse_unix_s(args: &Args) -> Option<IDInfo> {
    parse_unix_core(args, TimestampUnitAssumption::Seconds)
}

pub fn parse_unix_ms(args: &Args) -> Option<IDInfo> {
    parse_unix_core(args, TimestampUnitAssumption::Milliseconds)
}

pub fn parse_unix_us(args: &Args) -> Option<IDInfo> {
    parse_unix_core(args, TimestampUnitAssumption::Microseconds)
}

pub fn parse_unix_ns(args: &Args) -> Option<IDInfo> {
    parse_unix_core(args, TimestampUnitAssumption::Nanoseconds)
}

pub fn parse_unix_recent(args: &Args) -> Option<IDInfo> {
    parse_unix_core(args, TimestampUnitAssumption::Recent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unix() {
        fn _assert(id: &str, version: &str) {
            let result = parse_unix(&Args {
                id: id.to_string(),
                ..Default::default()
            })
            .unwrap();
            assert_eq!(result.id_type, "Unix timestamp".to_string());
            assert_eq!(result.version.unwrap(), version.to_string());
            assert_eq!(result.timestamp.unwrap(), "1735395353.000".to_string());
        }

        _assert("1735395353", "Assuming seconds");
        _assert("1735395353000", "Assuming milliseconds");
        _assert("1735395353000000", "Assuming microseconds");
        _assert("1735395353000000000", "Assuming nanoseconds");
    }

    #[test]
    fn test_parse_unix_s() {
        let result = parse_unix_s(&Args {
            id: "1735395353".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.version.unwrap(), "As seconds".to_string());
        assert_eq!(result.timestamp.unwrap(), "1735395353.000".to_string());
    }

    #[test]
    fn test_parse_unix_ms() {
        let result = parse_unix_ms(&Args {
            id: "1735395353000".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.version.unwrap(), "As milliseconds".to_string());
        assert_eq!(result.timestamp.unwrap(), "1735395353.000".to_string());
    }

    #[test]
    fn test_parse_unix_us() {
        let result = parse_unix_us(&Args {
            id: "1735395353000000".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.version.unwrap(), "As microseconds".to_string());
        assert_eq!(result.timestamp.unwrap(), "1735395353.000".to_string());
    }

    #[test]
    fn test_parse_unix_ns() {
        let result = parse_unix_ns(&Args {
            id: "1735395353000000000".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.version.unwrap(), "As nanoseconds".to_string());
        assert_eq!(result.timestamp.unwrap(), "1735395353.000".to_string());
    }

    #[test]
    fn test_parse_unix_recent_some() {
        let result = parse_unix_recent(&Args {
            id: "1735395353".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.version.unwrap(), "Assuming seconds".to_string());
        assert_eq!(result.timestamp.unwrap(), "1735395353.000".to_string());
    }

    #[test]
    fn test_parse_unix_recent_none() {
        assert!(parse_unix_recent(&Args {
            id: "1000000000".to_string(),
            ..Default::default()
        })
        .is_none());
    }

    #[test]
    fn test_overflows() {
        let args = Args {
            id: u64::MAX.to_string(),
            ..Default::default()
        };
        assert!(parse_unix_s(&args).is_none());
        assert!(parse_unix_ms(&args).is_none());
        assert!(parse_unix_us(&args).is_none());
        assert!(parse_unix_ns(&args).is_some()); // Don't overflow.
    }
}
