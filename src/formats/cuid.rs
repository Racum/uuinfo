use basen::BASE36;

use crate::schema::{Args, IDInfo};
use crate::utils::{factor_size_hex_bits_color_from_text, milliseconds_to_seconds_and_iso8601, repeat_char};

pub fn parse_cuid1(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 25 || &args.id[0..1] != "c" {
        return None;
    }
    let timestamp_raw: u64 = BASE36.decode_var_len(&args.id[1..9])?;
    let sequence: u64 = BASE36.decode_var_len(&args.id[9..13])?;
    let fingerprint: u64 = BASE36.decode_var_len(&args.id[13..17])?;
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, None);
    let (size, hex, bits, _) = factor_size_hex_bits_color_from_text(&args.id);

    Some(IDInfo {
        id_type: "CUID".to_string(),
        version: Some("1".to_string()),
        standard: args.id.to_string(),
        parsed: Some("as ASCII, with base36 parts".to_string()),
        size,
        entropy: 64,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        sequence: Some(sequence as u128),
        node1: Some(format!("{} (Fingerprint)", fingerprint)),
        hex,
        bits,
        color_map: Some(format!(
            "{}{}{}{}{}",
            repeat_char('1', 8),
            repeat_char('3', 64),
            repeat_char('6', 32),
            repeat_char('4', 32),
            repeat_char('2', 64),
        )),
        high_confidence: true,
        ..Default::default()
    })
}

pub fn parse_cuid2(args: &Args) -> Option<IDInfo> {
    let id_len = args.id.chars().count();
    if !(2..=32).contains(&id_len) || !cuid2::is_cuid2(&args.id) {
        return None;
    }
    let (size, hex, bits, color_map) = factor_size_hex_bits_color_from_text(&args.id);

    Some(IDInfo {
        id_type: "CUID".to_string(),
        version: Some("2".to_string()),
        standard: args.id.to_string(),
        parsed: Some("as ASCII, with base36 hash".to_string()),
        size,
        entropy: size,
        hex,
        bits,
        color_map,
        high_confidence: id_len == 24,
        ..Default::default()
    })
}
