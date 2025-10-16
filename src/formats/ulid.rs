use std::fmt::Write;
use ulid::Ulid;
use uuid::Uuid;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

pub fn parse_ulid(args: &Args) -> Option<IDInfo> {
    let mut id_type = "ULID";
    let mut parsed = "from Crockford's base32";
    let mut from_base32 = true;
    let ulid = match Ulid::from_string(&args.id) {
        Ok(value) => value,
        Err(_) => {
            id_type = "ULID wrapped in UUID";
            parsed = "from hex";
            from_base32 = false;
            Ulid::from(Uuid::try_parse(&args.id).ok()?)
        }
    };
    let uuid = Uuid::from_bytes(ulid.to_bytes());
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(ulid.timestamp_ms(), None);

    Some(IDInfo {
        id_type: id_type.to_string(),
        standard: ulid.to_string(),
        integer: Some(ulid.0),
        uuid_wrap: Some(uuid.to_string()),
        parsed: Some(parsed.to_string()),
        size: 128,
        entropy: 80,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        hex: Some(hex::encode(ulid.to_bytes())),
        bits: Some(ulid.to_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("33333333333333333333333333333333333333333333333322222222222222222222222222222222222222222222222222222222222222222222222222222222".to_string()),
        high_confidence: from_base32,
        ..Default::default()
    })
}

fn julid_sequence(integer: u128) -> u128 {
    integer << 48 >> 112
}

pub fn parse_julid(args: &Args) -> Option<IDInfo> {
    let mut julid = parse_ulid(args)?;
    julid.id_type = "Julid".to_string();
    julid.color_map = Some("33333333333333333333333333333333333333333333333366666666666666662222222222222222222222222222222222222222222222222222222222222222".to_string());
    julid.entropy = 64;
    julid.sequence = Some(julid_sequence(julid.integer?));
    Some(julid)
}

pub fn parse_ulid_any(args: &Args) -> Option<IDInfo> {
    let ulid = parse_ulid(args)?;
    if julid_sequence(ulid.integer?) < 128 {
        return parse_julid(args);
    }
    Some(ulid)
}
