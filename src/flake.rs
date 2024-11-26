extern crate base62;
use uuid::Uuid;

use crate::schema::{Args, IDInfo};
use crate::utils::{bits128, milliseconds_to_seconds_and_iso8601};

pub fn parse_flake(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 18 {
        return None;
    }
    let id_int = match base62::decode(&args.id) {
        Ok(value) => value,
        Err(_) => return None,
    };

    let timestamp_raw = bits128(id_int, 0, 64);
    let worker_id = bits128(id_int, 64, 48);
    let sequence = bits128(id_int, 112, 16);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw as u64, None);

    Some(IDInfo {
        known: true,
        id_type: "Flake (Boundary)".to_string(),
        version: None,
        standard: args.id.to_string(),
        integer: Some(id_int),
        short_uuid: None,
        base64: None,
        uuid_wrap: Some(Uuid::from_bytes(id_int.to_be_bytes()).to_string()),
        size: 128,
        entropy: 0,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        sequence: Some(sequence),
        node1: Some(worker_id.to_string()),
        node2: None,
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(
            id_int
                .to_be_bytes()
                .iter()
                .map(|&c| format!("{c:08b}"))
                .collect(),
        ),
        color_map: Some("33333333333333333333333333333333333333333333333333333333333333334444444444444444444444444444444444444444444444446666666666666666".to_string()),
    })
}
