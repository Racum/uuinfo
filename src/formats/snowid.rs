use std::fmt::Write;

use crate::schema::{Args, IDInfo};
use crate::utils::{bits64, milliseconds_to_seconds_and_iso8601};

pub fn parse_snowid(args: &Args) -> Option<IDInfo> {
    let mut from_base62 = false;
    let id_int: u64 = match args.id.trim().parse::<u64>() {
        Ok(value) => value,
        Err(_) => match base62::decode(&args.id) {
            Ok(value) => {
                from_base62 = true;
                value.try_into().ok()?
            }
            Err(_) => return None,
        },
    };
    let timestamp_raw = bits64(id_int, 0, 42);
    let node_id = bits64(id_int, 42, 10);
    let sequence = bits64(id_int, 52, 12);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, Some(1704067200000));

    Some(IDInfo {
        id_type: "SnowID".to_string(),
        standard: base62::encode(id_int),
        integer: Some(id_int as u128),
        parsed: Some(if from_base62 { "from base62".to_string() } else { "as integer".to_string() }),
        size: 64,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        node1: Some(format!("{} (Node ID)", node_id)),
        sequence: Some(sequence as u128),
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("3333333333333333333333333333333333333333334444444444666666666666".to_string()),
        high_confidence: from_base62,
        ..Default::default()
    })
}
