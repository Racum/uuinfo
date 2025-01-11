use std::fmt::Write;

use crate::schema::{Args, IDInfo};
use crate::utils::{bits64, milliseconds_to_seconds_and_iso8601};

const S32_CHAR: &str = "234567abcdefghijklmnopqrstuvwxyz";

pub fn s32decode(s: String) -> Option<u64> {
    let mut i: usize = 0;
    for c in s.chars() {
        i = i * 32 + S32_CHAR.chars().position(|x| x == c)?;
    }
    Some(i as u64)
}

pub fn parse_tid(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 13 {
        return None;
    }
    let id_int = s32decode(args.id.clone())?;
    let timestamp_raw = bits64(id_int, 1, 53);
    let clock_id = bits64(id_int, 54, 10);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw / 1_000, None);

    Some(IDInfo {
        id_type: "TID (AT Protocol, Bluesky)".to_string(),
        standard: args.id.clone(),
        integer: Some(id_int.into()),
        parsed: Some("from base32".to_string()),
        size: 64,
        entropy: 0,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        node1: Some(format!("{} (Clock ID)", clock_id)),
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("0333333333333333333333333333333333333333333333333333334444444444".to_string()),
        ..Default::default()
    })
}
