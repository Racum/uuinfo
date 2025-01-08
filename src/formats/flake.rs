use std::fmt::Write;
use uuid::Uuid;

use crate::schema::{Args, IDInfo};
use crate::utils::{bits128, milliseconds_to_seconds_and_iso8601};

pub fn parse_flake(args: &Args) -> Option<IDInfo> {
    let uuid: Uuid;
    let mut id_type = "Flake (Boundary)";
    let mut parsed = "from base62";
    let id_int = match args.id.chars().count() {
        18 => match base62::decode(&args.id) {
            Ok(value) => {
                uuid = Uuid::from_bytes(value.to_be_bytes());
                value
            }
            Err(_) => return None,
        },
        _ => match Uuid::parse_str(&args.id) {
            Ok(value) => {
                id_type = "Flake (Boundary) wrapped in UUID";
                parsed = "from hex";
                uuid = value;
                value.as_u128()
            }
            Err(_) => return None,
        },
    };

    let timestamp_raw = bits128(id_int, 0, 64);
    let worker_id = bits128(id_int, 64, 48);
    let sequence = bits128(id_int, 112, 16);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw as u64, None);

    Some(IDInfo {
        id_type: id_type.to_string(),
        standard: base62::encode(id_int).to_string(),
        integer: Some(id_int),
        uuid_wrap: Some(uuid.to_string()),
        parsed: Some(parsed.to_string()),
        size: 128,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        sequence: Some(sequence),
        node1: Some(worker_id.to_string()),
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("33333333333333333333333333333333333333333333333333333333333333334444444444444444444444444444444444444444444444446666666666666666".to_string()),
        ..Default::default()
    })
}
