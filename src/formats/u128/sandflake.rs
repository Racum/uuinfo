use crate::schema::{Args, IDInfo};
use crate::utils::{bits128, milliseconds_to_seconds_and_iso8601, repeat_char};
use chrono::{Duration, Utc};
use std::fmt::Write;
use uuid::Uuid;

const SANDFLAKE_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

// The closest round date to Sandflake's first implementation:
const MIN_MS: i64 = 1_420_070_400_000; // 2015-01-01T00:00:00Z

/// RFC4648 base32 encode (left-aligned, MSB first).
fn encode_base32(bytes: &[u8; 16]) -> String {
    let mut out = String::with_capacity(26);
    let mut acc: u64 = 0;
    let mut nbits: u32 = 0;
    for &b in bytes {
        acc = (acc << 8) | b as u64;
        nbits += 8;
        while nbits >= 5 {
            nbits -= 5;
            if let Some(&c) = SANDFLAKE_ALPHABET.get(((acc >> nbits) & 0x1F) as usize) {
                out.push(c as char);
            }
        }
    }
    if nbits > 0
        && let Some(&c) = SANDFLAKE_ALPHABET.get(((acc << (5 - nbits)) & 0x1F) as usize)
    {
        out.push(c as char);
    }
    out
}

/// RFC4648 base32 decode (left-aligned, MSB first) with custom alphabet.
fn decode_base32(id: &str) -> Option<[u8; 16]> {
    if id.len() != 26 {
        return None;
    }
    let mut acc: u64 = 0;
    let mut nbits: u32 = 0;
    let mut bytes: Vec<u8> = Vec::with_capacity(16);
    for c in id.bytes() {
        let digit = SANDFLAKE_ALPHABET.iter().position(|&a| a == c)? as u64;
        acc = (acc << 5) | digit;
        nbits += 5;
        if nbits >= 8 {
            nbits -= 8;
            bytes.push((acc >> nbits) as u8);
        }
    }
    bytes.try_into().ok()
}

pub fn parse_sandflake(args: &Args) -> Option<IDInfo> {
    let id = args.id.trim();
    let (bytes, id_type, parsed) = match decode_base32(id) {
        Some(bytes) => (bytes, "Sandflake", "from base32, custom alphabet"),
        None => (*Uuid::try_parse(id).ok()?.as_bytes(), "Sandflake wrapped in UUID", "from hex"),
    };
    let id_int: u128 = u128::from_be_bytes(bytes);
    let timestamp_raw = bits128(id_int, 0, 48) as u64;
    let worker = bits128(id_int, 48, 32);
    let sequence = bits128(id_int, 80, 24);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, 0);
    let max_ms = (Utc::now() + Duration::days(10 * 365)).timestamp_millis();
    let high_confidence = (MIN_MS..=max_ms).contains(&(timestamp_raw as i64));

    Some(IDInfo {
        id_type: id_type.to_string(),
        standard: encode_base32(&bytes),
        integer: Some(id_int),
        uuid_wrap: Some(Uuid::from_bytes(bytes).hyphenated().to_string()),
        parsed: Some(parsed.to_string()),
        size: 128,
        entropy: 24,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        sequence: Some(sequence),
        node1: Some(format!("{} (Worker ID)", worker)),
        hex: Some(hex::encode(bytes)),
        bits: Some(bytes.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(repeat_char('3', 48) + &repeat_char('4', 32) + &repeat_char('6', 24) + &repeat_char('2', 24)),
        high_confidence,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sandflake() {
        let args = Args {
            id: "05E4ECYW2GZ66B8AFZZZZMKFPR".to_string(),
            ..Default::default()
        };
        let result = parse_sandflake(&args).expect("Should parse sandflake");
        assert_eq!(result.id_type, "Sandflake");
        assert_eq!(result.parsed.unwrap(), "from base32, custom alphabet");
        assert_eq!(result.datetime.unwrap(), "2017-05-27T00:00:00.020Z");
        assert!(result.high_confidence);
    }

    #[test]
    fn test_parse_sandflake_wrapped_in_uuid() {
        let args = Args {
            id: "015c4733-dc14-3e63-2d0a-7fffffd26fb6".to_string(),
            ..Default::default()
        };
        let result = parse_sandflake(&args).expect("Should unwrap sandflake from UUID");
        assert_eq!(result.id_type, "Sandflake wrapped in UUID");
        assert_eq!(result.parsed.unwrap(), "from hex");
        assert_eq!(result.standard, "05E4ECYW2GZ66B8AFZZZZMKFPR");
        assert_eq!(result.datetime.unwrap(), "2017-05-27T00:00:00.020Z");
    }
}
