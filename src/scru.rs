use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;
use scru128::Scru128Id;
use scru64::Scru64Id;
use uuid::Uuid;

pub fn parse_scru128(args: &Args) -> Option<IDInfo> {
    let scru = match Scru128Id::try_from_str(&args.id) {
        Ok(value) => value,
        Err(_) => return None,
    };
    let uuid = Uuid::from_bytes(scru.as_bytes().to_owned());
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(scru.timestamp(), None);

    Some(IDInfo {
        known: true,
        id_type: "SCRU128".to_string(),
        version: None,
        standard: args.id.to_string(),
        integer: Some(scru.to_u128()),
        short_uuid: None,
        base64: None,
        uuid_wrap: Some(uuid.to_string()),
        size: 128,
        entropy: 80,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        sequence: None,
        node1: None,
        node2:None,
        hex: Some(hex::encode(scru.as_bytes())),
        bits: Some(
            scru.as_bytes()
                .iter()
                .map(|&c| format!("{c:08b}"))
                .collect(),
        ),
        color_map: Some("33333333333333333333333333333333333333333333333322222222222222222222222222222222222222222222222222222222222222222222222222222222".to_string()),
    })
}

pub fn parse_scru64(args: &Args) -> Option<IDInfo> {
    let binding = args.id.parse::<Scru64Id>();
    let scru = match &binding {
        Ok(value) => value,
        Err(_) => return None,
    };
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(scru.timestamp() * 256, None);

    Some(IDInfo {
        known: true,
        id_type: "SCRU64".to_string(),
        version: None,
        standard: args.id.to_string(),
        integer: Some(scru.to_u64() as u128),
        short_uuid: None,
        base64: None,
        uuid_wrap: None,
        size: 64,
        entropy: 0,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        sequence: None,
        node1: Some(format!("{} (Node ID)", scru.node_ctr())),
        node2: None,
        hex: Some(hex::encode(scru.to_u64().to_be_bytes())),
        bits: Some(
            scru.to_u64()
                .to_be_bytes()
                .iter()
                .map(|&c| format!("{c:08b}"))
                .collect(),
        ),
        color_map: Some(
            "0033333333333333333333333333333333333333444444444444444444444444".to_string(),
        ),
    })
}
