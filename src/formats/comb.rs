use crate::schema::{Args, IDInfo};
use crate::utils::{milliseconds_to_seconds_and_iso8601, repeat_char};
use chrono::{DateTime, Duration, NaiveDate, Utc};
use std::fmt::Write;
use uuid::Uuid;

const WINDOW_PAST_DAYS: i64 = 30 * 365;
const WINDOW_FUTURE_DAYS: i64 = 10 * 365;

// Why 2000-01-01?
// - 2000-01-01: Closest "round" date.
// - 2000-02-17: Windows 2000 released (broke sequential GUIDs).
// - 2001-12-12: Book "NET Enterprise Design with Visual Basic .NET and SQL Server 2000" published.
// - 2002-03-08: Informit article "The Cost of GUIDs as Primary Keys" published (cites book).
const MIN_VALID_TS: i64 = 946_684_800_000; // 2000-01-01T00:00:00Z, in ms

fn read_u48_be(bytes: &[u8]) -> i64 {
    bytes.iter().fold(0i64, |acc, &b| (acc << 8) | b as i64)
}

/// Legacy SqlDateTimeStrategy: last 6 bytes hold 2 bytes of days since
/// 1900-01-01 (BE) and 4 bytes of 1/300s ticks since midnight (BE).
/// Returns a timestamp in milliseconds since the Unix epoch.
fn legacy_ts(bytes: &[u8; 16]) -> Option<i64> {
    let days = u16::from_be_bytes([bytes[10], bytes[11]]) as i64;
    let ticks = u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]) as i64;
    let base = NaiveDate::from_ymd_opt(1900, 1, 1)?.and_hms_opt(0, 0, 0)?;
    let naive = base + Duration::days(days) + Duration::milliseconds(ticks * 1000 / 300);
    Some(DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc).timestamp_millis())
}

pub fn parse_comb(args: &Args) -> Option<IDInfo> {
    let uuid = Uuid::parse_str(&args.id).ok()?;
    let bytes = uuid.as_bytes();
    // Every COMB variation is a valid UUIDv4:
    if bytes[6] & 0xF0 != 0x40 || bytes[8] & 0xC0 != 0x80 {
        return None;
    }
    let candidates: [(&str, i64); 3] = [
        ("PostgreSQL", read_u48_be(bytes.get(0..6)?)),
        ("MS-SQL", read_u48_be(bytes.get(10..16)?)),
        ("Legacy", legacy_ts(bytes)?),
    ];
    let now = Utc::now().timestamp_millis();
    let low = now - Duration::days(WINDOW_PAST_DAYS).num_milliseconds();
    let high = now + Duration::days(WINDOW_FUTURE_DAYS).num_milliseconds();
    let in_window: Vec<&(&str, i64)> = candidates.iter().filter(|(_, ts)| (low..=high).contains(ts)).collect();

    let (version, ts, high_confidence) = match in_window.first() {
        // Exactly one strategy lands in the window: confident pick.
        Some(&&(v, t)) if in_window.len() == 1 => (v, t, true),
        // Several land in the window: pick the highest priority, low confidence.
        Some(&&(v, t)) => (v, t, false),
        // None land in the window: best-effort highest priority, low confidence.
        None => {
            let &(v, t) = candidates.first()?;
            (v, t, false)
        }
    };
    // Reject anything before the algorithm could exist, regardless of confidence.
    if ts < MIN_VALID_TS {
        return None;
    }

    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(ts as u64, 0);
    // PostgreSQL keeps the timestamp in the first 6 bytes, the others in the last 6.
    let mut color_chars: Vec<char> = if version == "PostgreSQL" {
        repeat_char('3', 48) + &repeat_char('2', 80)
    } else {
        repeat_char('2', 80) + &repeat_char('3', 48)
    }
    .chars()
    .collect();
    // COMB keeps the UUID layout, so the reserved bits are not data:
    for i in (48..52).chain(64..66) {
        if let Some(c) = color_chars.get_mut(i) {
            *c = '0';
        }
    }
    let color_map: String = color_chars.into_iter().collect();

    Some(IDInfo {
        id_type: "COMB".to_string(),
        version: Some(version.to_string()),
        standard: uuid.hyphenated().to_string(),
        integer: Some(u128::from_be_bytes(*bytes)),
        size: 128,
        entropy: 74,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        hex: Some(hex::encode(bytes)),
        bits: Some(bytes.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(color_map),
        high_confidence,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn comb_args(id: &str) -> Args {
        Args {
            id: id.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_comb_mssql() {
        // Only the MS-SQL strategy lands in the window -> confident.
        let id = parse_comb(&comb_args("d7a5115f-220b-4928-8c3e-019e793cefb1")).unwrap();
        assert_eq!(id.id_type, "COMB".to_string());
        assert_eq!(id.version, Some("MS-SQL".to_string()));
        assert_eq!(id.datetime, Some("2026-05-30T14:14:57.201Z".to_string()));
        assert!(id.high_confidence);
    }

    #[test]
    fn test_comb_postgresql() {
        // Only the PostgreSQL strategy lands in the window -> confident.
        let id = parse_comb(&comb_args("019e793c-efb1-4848-9b57-4534777cb348")).unwrap();
        assert_eq!(id.id_type, "COMB".to_string());
        assert_eq!(id.version, Some("PostgreSQL".to_string()));
        assert_eq!(id.datetime, Some("2026-05-30T14:14:57.201Z".to_string()));
        assert!(id.high_confidence);
    }

    #[test]
    fn test_comb_legacy() {
        // Only the Legacy strategy lands in the window -> confident.
        let id = parse_comb(&comb_args("af38d57a-7843-4f99-b6d2-b45a00ead1e8")).unwrap();
        assert_eq!(id.id_type, "COMB".to_string());
        assert_eq!(id.version, Some("Legacy".to_string()));
        assert_eq!(id.datetime, Some("2026-05-30T14:14:57.200Z".to_string()));
        assert!(id.high_confidence);
    }
}
