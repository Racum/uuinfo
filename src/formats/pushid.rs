use base64::engine::GeneralPurpose;
use base64::{alphabet, engine, Engine as _};
use std::fmt::Write;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

const ALPHABET: &str = "-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz";

fn pushid_base64_engine() -> GeneralPurpose {
    let alphabet = alphabet::Alphabet::new(ALPHABET).unwrap();
    let crazy_config = engine::GeneralPurposeConfig::new()
        .with_decode_allow_trailing_bits(true)
        .with_encode_padding(false)
        .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);
    engine::GeneralPurpose::new(&alphabet, crazy_config)
}

pub fn parse_pushid(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 20 {
        return None;
    }
    let id_bytes = pushid_base64_engine().decode(&args.id).ok()?;
    let mut ts_buffer: Vec<u8> = vec![0, 0];
    ts_buffer.extend(&id_bytes[0..6]);
    let ts_bytes: [u8; 8] = ts_buffer.try_into().ok()?;
    let timestamp_raw = u64::from_be_bytes(ts_bytes);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, None);

    Some(IDInfo {
        id_type: "PushID (Firebase)".to_string(),
        standard: args.id.to_string(),
        parsed: Some("from base64".to_string()),
        size: 120,
        entropy: 96,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        hex: Some(hex::encode(&id_bytes)),
        bits: Some(id_bytes.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("333333333333333333333333333333333333333333333333222222222222222222222222222222222222222222222222222222222222222222222222".to_string()),
        ..Default::default()
    })
}
