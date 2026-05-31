use std::fmt::Write;
use uuid::Uuid;

use crate::schema::{Args, IDInfo};
use crate::utils::{bits128, epoch_ms, milliseconds_to_seconds_and_iso8601, repeat_char};

pub fn parse_flake(args: &Args) -> Option<IDInfo> {
    let uuid: Uuid;
    let mut id_type = "Flake (Boundary)";
    let mut parsed = "from base62";
    let mut from_base62 = false;
    let id_int = match args.id.chars().count() {
        18 => match base62::decode(&args.id) {
            Ok(value) => {
                uuid = Uuid::from_bytes(value.to_be_bytes());
                from_base62 = true;
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
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw as u64, epoch_ms(args, 0));
    if datetime.starts_with("Invalid") {
        return None;
    }

    Some(IDInfo {
        id_type: id_type.to_string(),
        standard: base62::encode(id_int),
        integer: Some(id_int),
        uuid_wrap: Some(uuid.to_string()),
        parsed: Some(parsed.to_string()),
        size: 128,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        sequence: Some(sequence),
        node1: Some(worker_id.to_string()),
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(repeat_char('3', 64) + &repeat_char('4', 48) + &repeat_char('6', 16)),
        high_confidence: from_base62 && id_int >> 108 == 0,
        ..Default::default()
    })
}
