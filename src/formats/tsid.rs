use std::fmt::Write;
use tsid::TSID;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

pub fn parse_tsid(args: &Args) -> Option<IDInfo> {
    let tsid_id: TSID = match TSID::try_from(args.id.as_str()) {
        Ok(value) => value,
        Err(_) => {
            let id_int: u64 = args.id.trim().parse::<u64>().ok()?;
            TSID::from(id_int)
        }
    };
    let id_int: u64 = tsid_id.number();
    let raw_timestamp = tsid_id.timestamp().timestamp_millis() as u64;
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(raw_timestamp, None);

    Some(IDInfo {
        id_type: "TSID".to_string(),
        standard: format!("{}", tsid_id),
        integer: Some(id_int.into()),
        parsed: Some("from Crockford's base32".to_string()),
        size: 64,
        entropy: 22,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("3333333333333333333333333333333333333333332222222222222222222222".to_string()),
        ..Default::default()
    })
}
