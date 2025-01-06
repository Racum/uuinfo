use std::fmt::Write;
use ulid::Ulid;
use uuid::Uuid;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

const PREFIX_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz_";

pub fn parse_typeid(args: &Args) -> Option<IDInfo> {
    let parts: &Vec<&str> = &args.id.split('_').collect();
    let prefix: String;
    let value: &str;
    match parts.split_last() {
        Some((last, elements)) => {
            prefix = elements.join("_");
            value = last;
        }
        None => return None,
    }
    if prefix.chars().count() > 63 || !prefix.chars().all(|c| PREFIX_ALPHABET.contains(c)) {
        return None;
    }
    let ulid = Ulid::from_string(value).ok()?;
    let uuid: Uuid = Uuid::from_bytes(ulid.to_bytes());
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(ulid.timestamp_ms(), None);

    Some(IDInfo {
        id_type: "TypeID".to_string(),
        standard: args.id.to_string(),
        integer: Some(ulid.0),
        uuid_wrap: Some(uuid.to_string()),
        parsed: Some("from base32, suffix only".to_string()),
        size: 128,
        entropy: 74,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        node1: Some(prefix),
        hex: Some(hex::encode(uuid.as_bytes())),
        bits: Some(uuid.as_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("33333333333333333333333333333333333333333333333311112222222222220022222222222222222222222222222222222222222222222222222222222222".to_string()),
        ..Default::default()
    })
}
