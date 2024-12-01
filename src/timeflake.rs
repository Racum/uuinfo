use std::fmt::Write;
use timeflake_rs::Timeflake;
extern crate base62;
use uuid::Uuid;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

pub fn parse_timeflake_core(hex_id: &str) -> Option<IDInfo> {
    let timeflake = match Timeflake::parse(hex_id) {
        Ok(value) => value,
        Err(_) => return None,
    };

    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timeflake.timestamp.as_millis() as u64, None);

    Some(IDInfo {
        id_type: "Timeflake".to_string(),
        version: None,
        standard: format!("{:0>22}", base62::encode(timeflake.as_u128())),
        integer: Some(timeflake.as_u128()),
        short_uuid: None,
        base64: None,
        uuid_wrap: Some(timeflake.as_uuid().to_string()),
        size: 128,
        entropy: 80,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        sequence: None,
        node1: None,
        node2: None,
        hex: Some(hex_id.to_string()),
        bits: Some(timeflake.as_u128().to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("33333333333333333333333333333333333333333333333322222222222222222222222222222222222222222222222222222222222222222222222222222222".to_string()),
    })
}

pub fn parse_timeflake_hex(args: &Args) -> Option<IDInfo> {
    parse_timeflake_core(&args.id)
}

pub fn parse_timeflake_base62(args: &Args) -> Option<IDInfo> {
    match base62::decode(&args.id) {
        Ok(value) => parse_timeflake_core(&hex::encode(value.to_be_bytes())),
        Err(_) => None,
    }
}

pub fn parse_timeflake_uuid(args: &Args) -> Option<IDInfo> {
    let new_args: &mut Args = &mut args.clone();
    new_args.id = match Uuid::parse_str(&args.id) {
        Ok(value) => hex::encode(value.as_bytes()),
        Err(_) => return None,
    };
    let mut id_info = match parse_timeflake_hex(new_args) {
        Some(value) => value,
        None => return None,
    };
    id_info.id_type = "Timeflake wrapped in UUID".to_string();
    Some(id_info)
}

pub fn parse_timeflake_any(args: &Args) -> Option<IDInfo> {
    match args.id.chars().count() {
        22 => parse_timeflake_base62(args),
        32 => parse_timeflake_hex(args),
        36 => parse_timeflake_uuid(args),
        _ => None,
    }
}
