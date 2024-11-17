use ulid::Ulid;
use uuid::Uuid;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

pub fn parse_ulid(args: &Args) -> Option<IDInfo> {
    let ulid = match Ulid::from_string(&args.id) {
        Ok(value) => value,
        Err(_) => return None,
    };

    let uuid = Uuid::from_bytes(ulid.to_bytes());
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(ulid.timestamp_ms(), None);

    Some(IDInfo {
        known: true,
        id_type: "ULID".to_string(),
        version: None,
        standard: args.id.to_string(),
        integer: Some(ulid.0),
        short_uuid: None,
        base64: None,
        uuid_like: Some(uuid.to_string()),
        size: 128,
        entropy: 80,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        sequence: None,
        node1: None,
        node2: None,
        hex: Some(hex::encode(ulid.to_bytes())),
        bits: Some(
            ulid.to_bytes()
                .iter()
                .map(|&c| format!("{c:08b}"))
                .collect(),
        ),
        color_map: Some("33333333333333333333333333333333333333333333333322222222222222222222222222222222222222222222222222222222222222222222222222222222".to_string()),
    })
}
