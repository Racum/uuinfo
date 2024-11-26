extern crate basen;
use basen::BASE36;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

pub fn parse_cuid1(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 25 {
        return None;
    }
    if &args.id[0..1] != "c" {
        return None;
    }
    let timestamp_raw: u64 = match BASE36.decode_var_len(&args.id[1..9]) {
        Some(value) => value,
        None => return None,
    };
    let sequence: u64 = match BASE36.decode_var_len(&args.id[9..13]) {
        Some(value) => value,
        None => return None,
    };
    let fingerprint: u64 = match BASE36.decode_var_len(&args.id[13..17]) {
        Some(value) => value,
        None => return None,
    };
    let random_data: u64 = match BASE36.decode_var_len(&args.id[17..25]) {
        Some(value) => value,
        None => return None,
    };

    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, None);

    Some(IDInfo {
        known: true,
        id_type: "CUID".to_string(),
        version: Some("1".to_string()),
        standard: args.id.to_string(),
        integer: None,
        short_uuid: None,
        base64: None,
        uuid_wrap: None,
        size: 0,
        entropy: 0,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        sequence: Some(sequence as u128),
        node1: Some(format!("{} (Fingerprint)", fingerprint)),
        node2: Some(format!("{} (Random data)", random_data)),
        hex: None,
        bits: None,
        color_map: None,
    })
}

pub fn parse_cuid2(args: &Args) -> Option<IDInfo> {
    if !cuid2::is_cuid2(&args.id) {
        return None;
    }
    Some(IDInfo {
        known: true,
        id_type: "CUID".to_string(),
        version: Some("2".to_string()),
        standard: args.id.to_string(),
        integer: None,
        short_uuid: None,
        base64: None,
        uuid_wrap: None,
        size: 0,
        entropy: 0,
        datetime: None,
        timestamp: None,
        sequence: None,
        node1: None,
        node2: None,
        hex: None,
        bits: None,
        color_map: None,
    })
}
