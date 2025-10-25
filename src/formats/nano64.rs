use std::fmt::Write;

use crate::schema::{Args, IDInfo};
use crate::utils::{bits64, milliseconds_to_seconds_and_iso8601};

pub fn parse_nano64(args: &Args) -> Option<IDInfo> {
    let hex_id = args.id.replace("-", "").to_uppercase();
    if hex_id.chars().count() != 16 {
        return None;
    }
    let id_bytes = hex::decode(&hex_id).ok()?;
    let id_int: u64 = u64::from_be_bytes(id_bytes.clone().try_into().unwrap());
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(bits64(id_int, 0, 44), None);

    Some(IDInfo {
        id_type: "Nano64".to_string(),
        standard: format!("{}-{}", &hex_id[0..11], &hex_id[11..16]),
        integer: Some(id_int.into()),
        parsed: Some("from hex".to_string()),
        size: 64,
        entropy: 20,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        hex: Some(hex::encode(id_bytes.clone())),
        bits: Some(id_bytes.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("3333333333333333333333333333333333333333333322222222222222222222".to_string()),
        high_confidence: true,
        ..Default::default()
    })
}
