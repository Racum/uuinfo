use ksuid::Ksuid;
use std::fmt::Write;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

pub fn parse_ksuid(args: &Args) -> Option<IDInfo> {
    let version: Option<String>;
    let ksuid = match Ksuid::from_base62(&args.id) {
        Ok(value) => {
            version = Some("Base62-encoded".to_string());
            value
        }
        Err(_) => match Ksuid::from_hex(&args.id) {
            Ok(value) => {
                version = Some("Hex-encoded".to_string());
                value
            }
            Err(_) => return None,
        },
    };

    let ms = (ksuid.time().unix_timestamp_nanos() / 1_000_000) as u64;
    let formatted_time = milliseconds_to_seconds_and_iso8601(ms, None);
    let timestamp = Some(formatted_time.0);
    let datetime = Some(formatted_time.1);

    Some(IDInfo {
        id_type: "KSUID".to_string(),
        version,
        standard: ksuid.to_base62(),
        size: 160,
        entropy: 128,
        datetime,
        timestamp,
        hex: Some(ksuid.to_hex()),
        bits: Some(ksuid.as_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("3333333333333333333333333333333322222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222".to_string()),
        ..Default::default()
    })
}
