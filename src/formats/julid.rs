use julid::Julid;
use std::fmt::Write;
use uuid::Uuid;

use crate::schema::{Args, IDInfo, IdFormat};

pub fn parse_julid(args: &Args) -> Option<IDInfo> {
    let mut id_type = "Julid";
    let mut parsed = "from Crockford's base32";
    let julid = match Julid::from_str(&args.id) {
        Ok(value) => value,
        Err(_) => {
            id_type = "Julid wrapped in UUID";
            parsed = "from hex";
            Julid::from_uuid(Uuid::try_parse(&args.id).ok()?).ok()?
        }
    };

    // there's not a bulletproof way to detect a Julid from a ULID, so we just say if there's any
    // bits set in the high end of the counter section it's probably a regular ULID
    if julid.counter().leading_zeros() < 8 && args.force != Some(IdFormat::Julid) {
        return None;
    }

    // UUIDv7
    let uuid = julid.as_uuid();

    // julid timestamp is in milliseconds, but we want seconds
    let timestamp = (julid.timestamp() as f64) / 1000.0;
    let timestamp = format!("{:.3}", timestamp);
    let datetime = julid.created_at().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    Some(IDInfo {
        id_type: id_type.to_string(),
        standard: julid.to_string(),
        integer: Some(u128::from_be_bytes(julid.as_bytes())),
        uuid_wrap: Some(uuid.to_string()),
        parsed: Some(parsed.to_string()),
        size: 128,
        entropy: 64,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        sequence: Some(julid.counter() as u128),
        hex: Some(hex::encode(julid.as_bytes())),
        bits: Some(julid.as_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("33333333333333333333333333333333333333333333333366666666666666662222222222222222222222222222222222222222222222222222222222222222".to_string()),
        ..Default::default()
    })
}
