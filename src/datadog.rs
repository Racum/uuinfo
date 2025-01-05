use std::fmt::Write;

use crate::schema::{Args, IDInfo};
use crate::utils::{bits128, milliseconds_to_seconds_and_iso8601};

pub fn parse_datadog(args: &Args) -> Option<IDInfo> {
    let id_bytes = hex::decode(args.id.clone()).ok()?;
    let id_bytes: [u8; 16] = id_bytes.try_into().ok()?;
    let id_int: u128 = u128::from_be_bytes(id_bytes);
    let raw_ts = bits128(id_int, 0, 32);
    let zeroes = bits128(id_int, 32, 32);
    if zeroes > 0 {
        return None;
    }

    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601((raw_ts * 1_000) as u64, None);

    Some(IDInfo {
        id_type: "Datadog Trace ID".to_string(),
        standard: args.id.to_string(),
        integer: Some(id_int),
        size: 128,
        entropy: 64,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("33333333333333333333333333333333000000000000000000000000000000002222222222222222222222222222222222222222222222222222222222222222".to_string()),
        ..Default::default()
    })
}
