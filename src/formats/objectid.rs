use std::fmt::Write;

use crate::schema::{Args, IDInfo};
use crate::utils::{bits128, milliseconds_to_seconds_and_iso8601};

pub fn parse_objectid(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 24 {
        return None;
    }
    let oid_bytes = hex::decode(args.id.clone()).ok()?;
    let mut oid_extra_bytes: Vec<u8> = [0u8, 0u8, 0u8, 0u8].to_vec();
    oid_extra_bytes.extend_from_slice(&oid_bytes);
    let oid_int: u128 = u128::from_be_bytes(oid_extra_bytes.try_into().unwrap());
    let timestamp_raw = bits128(oid_int, 32, 32);
    let sequence = bits128(oid_int, 104, 24);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw as u64 * 1000, None);

    Some(IDInfo {
        id_type: "MongoDB ObjectId".to_string(),
        standard: args.id.to_string(),
        integer: Some(oid_int),
        parsed: Some("from hex".to_string()),
        size: 96,
        entropy: 40,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        sequence: Some(sequence),
        hex: Some(hex::encode(oid_bytes.clone())),
        bits: Some(oid_bytes.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("333333333333333333333333333333332222222222222222222222222222222222222222666666666666666666666666".to_string()),
        ..Default::default()
    })
}
