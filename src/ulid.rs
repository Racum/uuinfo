use std::fmt::Write;
use ulid::Ulid;
use uuid::Uuid;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

pub fn parse_ulid(args: &Args) -> Option<IDInfo> {
    let mut id_type = "ULID";
    let ulid = match Ulid::from_string(&args.id) {
        Ok(value) => value,
        Err(_) => {
            let uuid = match Uuid::try_parse(&args.id) {
                Ok(value) => value,
                Err(_) => return None,
            };
            id_type = "ULID wrapped in UUID";
            Ulid::from(uuid)
        }
    };

    let uuid = Uuid::from_bytes(ulid.to_bytes());
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(ulid.timestamp_ms(), None);

    Some(IDInfo {
        id_type: id_type.to_string(),
        version: None,
        standard: ulid.to_string(),
        integer: Some(ulid.0),
        short_uuid: None,
        base64: None,
        uuid_wrap: Some(uuid.to_string()),
        size: 128,
        entropy: 80,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        sequence: None,
        node1: None,
        node2: None,
        hex: Some(hex::encode(ulid.to_bytes())),
        bits: Some(ulid.to_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("33333333333333333333333333333333333333333333333322222222222222222222222222222222222222222222222222222222222222222222222222222222".to_string()),
    })
}
