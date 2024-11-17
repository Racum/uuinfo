use timeflake_rs::Timeflake;
extern crate base62;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

pub fn parse_timeflake_core(hex_id: &str) -> Option<IDInfo> {
    let timeflake = match Timeflake::parse(hex_id) {
        Ok(value) => value,
        Err(_) => return None,
    };

    let (timestamp, datetime) =
        milliseconds_to_seconds_and_iso8601(timeflake.timestamp.as_millis() as u64, None);

    Some(IDInfo {
        known: true,
        id_type: "Timeflake".to_string(),
        version: None,
        standard: format!("{:0>22}", base62::encode(timeflake.as_u128())),
        integer: Some(timeflake.as_u128()),
        short_uuid: None,
        base64: None,
        uuid_like: Some(timeflake.as_uuid().to_string()),
        size: 128,
        entropy: 80,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        sequence: None,
        node1: None,
        node2: None,
        hex: Some(hex_id.to_string()),
        bits: Some(
            timeflake
                .as_u128()
                .to_be_bytes()
                .iter()
                .map(|&c| format!("{c:08b}"))
                .collect(),
        ),
        color_map: Some("33333333333333333333333333333333333333333333333322222222222222222222222222222222222222222222222222222222222222222222222222222222".to_string()),
    })
}

#[allow(dead_code)]
pub fn parse_timeflake_hex(args: &Args) -> Option<IDInfo> {
    parse_timeflake_core(&args.id)
}

pub fn parse_timeflake_base62(args: &Args) -> Option<IDInfo> {
    match base62::decode(&args.id) {
        Ok(value) => parse_timeflake_core(&hex::encode(value.to_be_bytes())),
        Err(_) => None,
    }
}
