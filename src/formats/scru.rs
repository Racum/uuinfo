use scru64::Scru64Id;
use scru128::Scru128Id;
use std::fmt::Write;
use uuid::Uuid;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

pub fn parse_scru128(args: &Args) -> Option<IDInfo> {
    let mut id_type = "SCRU128";
    let mut parsed = "from base36";
    let mut from_base36 = true;
    let scru = match Scru128Id::try_from_str(&args.id) {
        Ok(value) => value,
        Err(_) => {
            let uuid = Uuid::try_parse(&args.id).ok()?;
            id_type = "SCRU128 wrapped in UUID";
            parsed = "from hex";
            from_base36 = false;
            Scru128Id::from_u128(uuid.as_u128())
        }
    };

    let uuid = Uuid::from_bytes(scru.as_bytes().to_owned());
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(scru.timestamp(), None);

    Some(IDInfo {
        id_type: id_type.to_string(),
        standard: scru.to_string(),
        integer: Some(scru.to_u128()),
        uuid_wrap: Some(uuid.to_string()),
        parsed: Some(parsed.to_string()),
        size: 128,
        entropy: 80,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        hex: Some(hex::encode(scru.as_bytes())),
        bits: Some(scru.as_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("33333333333333333333333333333333333333333333333322222222222222222222222222222222222222222222222222222222222222222222222222222222".to_string()),
        high_confidence: from_base36,
        ..Default::default()
    })
}

pub fn parse_scru64(args: &Args) -> Option<IDInfo> {
    let binding = args.id.parse::<Scru64Id>();
    let scru = &binding.ok()?;
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(scru.timestamp() * 256, None);

    Some(IDInfo {
        id_type: "SCRU64".to_string(),
        standard: args.id.to_string(),
        integer: Some(scru.to_u64() as u128),
        parsed: Some("from base36".to_string()),
        size: 64,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        node1: Some(format!("{} (Node ID)", scru.node_ctr())),
        hex: Some(hex::encode(scru.to_u64().to_be_bytes())),
        bits: Some(scru.to_u64().to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("0033333333333333333333333333333333333333444444444444444444444444".to_string()),
        high_confidence: true,
        ..Default::default()
    })
}
