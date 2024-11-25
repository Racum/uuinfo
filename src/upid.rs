use upid::Upid;
use uuid::Uuid;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

pub fn parse_upid(args: &Args) -> Option<IDInfo> {
    let upid = match Upid::from_string(&args.id) {
        Ok(value) => value,
        Err(_) => return None,
    };

    let uuid = Uuid::from_bytes(upid.to_bytes());
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(upid.milliseconds(), None);

    Some(IDInfo {
        known: true,
        id_type: "UPID".to_string(),
        version: Some("a (default)".to_string()),
        standard: args.id.to_string(),
        integer: Some(upid.0),
        short_uuid: None,
        base64: None,
        uuid_wrap: Some(uuid.to_string()),
        size: 128,
        entropy: 64,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        sequence: None,
        node1: Some(upid.prefix()),
        node2: None,
        hex: Some(hex::encode(upid.to_bytes())),
        bits: Some(
            upid.to_bytes()
                .iter()
                .map(|&c| format!("{c:08b}"))
                .collect(),
        ),
        color_map: Some("33333333333333333333333333333333333333332222222222222222222222222222222222222222222222222222222222222222444444444444444444441111".to_string()),
    })
}
