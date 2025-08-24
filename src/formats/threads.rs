use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::fmt::Write;

use crate::schema::{Args, IDInfo};
use crate::utils::{bits64, milliseconds_to_seconds_and_iso8601};

pub fn parse_threads(args: &Args) -> Option<IDInfo> {
    let id_int: u64;
    let parsed: Option<String>;
    if args.id.chars().count() <= 12 {
        // This is basically an Instagram snowflake wrapped in base64 with the leading A’s removed.
        let padded_id = format!("{:A>12}", &args.id);
        let id_bytes = &URL_SAFE_NO_PAD.decode(&padded_id).ok()?[1..9];
        id_int = u64::from_be_bytes(id_bytes.try_into().ok()?);
        parsed = Some("from base64".to_string());
    } else {
        id_int = args.id.trim().parse().ok()?;
        parsed = Some("as integer".to_string());
    }
    let timestamp_raw = bits64(id_int, 0, 41);
    let shard_id = bits64(id_int, 41, 13);
    let sequence = bits64(id_int, 54, 10);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, Some(1314220021721));

    Some(IDInfo {
        id_type: "Thread ID (Meta Threads)".to_string(),
        standard: args.id.clone(),
        integer: Some(id_int as u128),
        parsed,
        size: 64,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        node1: Some(format!("{} (Shard ID)", shard_id)),
        sequence: Some(sequence as u128),
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("3333333333333333333333333333333333333333344444444444446666666666".to_string()),
        ..Default::default()
    })
}
