use std::fmt::Write;
use upid::Upid;
use uuid::Uuid;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

pub fn parse_upid(args: &Args) -> Option<IDInfo> {
    let mut id_type = "UPID";
    let upid = match Upid::from_string(&args.id) {
        Ok(value) => value,
        Err(_) => {
            let uuid = match Uuid::try_parse(&args.id) {
                Ok(value) => value,
                Err(_) => return None,
            };
            id_type = "UPID wrapped in UUID";
            Upid::from(uuid.as_u128())
        }
    };

    let uuid = Uuid::from_bytes(upid.to_bytes());
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(upid.milliseconds(), None);

    Some(IDInfo {
        id_type: id_type.to_string(),
        version: Some("A (default)".to_string()),
        standard: upid.to_string(),
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
        bits: Some(upid.to_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("33333333333333333333333333333333333333332222222222222222222222222222222222222222222222222222222222222222444444444444444444441111".to_string()),
    })
}
