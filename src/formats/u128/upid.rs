use std::fmt::Write;
use upid::Upid;
use uuid::Uuid;

use crate::schema::{Args, IDInfo};
use crate::utils::{epoch_ms, milliseconds_to_seconds_and_iso8601, repeat_char};

pub fn parse_upid(args: &Args) -> Option<IDInfo> {
    let mut id_type = "UPID";
    let mut parsed = "from Crockford's base32";
    let mut from_base32 = true;
    let upid = match Upid::from_string(&args.id) {
        Ok(value) => value,
        Err(_) => {
            id_type = "UPID wrapped in UUID";
            parsed = "from hex";
            from_base32 = false;
            Upid::from(Uuid::try_parse(&args.id).ok()?.as_u128())
        }
    };

    let uuid = Uuid::from_bytes(upid.to_bytes());
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(upid.milliseconds(), epoch_ms(args, 0));

    Some(IDInfo {
        id_type: id_type.to_string(),
        version: Some("A (default)".to_string()),
        standard: upid.to_string(),
        integer: Some(upid.0),
        uuid_wrap: Some(uuid.to_string()),
        parsed: Some(parsed.to_string()),
        size: 128,
        entropy: 64,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        node1: Some(upid.prefix()),
        hex: Some(hex::encode(upid.to_bytes())),
        bits: Some(upid.to_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(repeat_char('3', 40) + &repeat_char('2', 64) + &repeat_char('4', 20) + &repeat_char('1', 4)),
        high_confidence: from_base32,
        ..Default::default()
    })
}
