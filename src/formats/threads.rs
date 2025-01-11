use base64::engine::GeneralPurpose;
use base64::{alphabet, engine, Engine as _};
use std::fmt::Write;

use crate::schema::{Args, IDInfo};
use crate::utils::{bits64, milliseconds_to_seconds_and_iso8601};

fn pushid_base64_engine() -> GeneralPurpose {
    let alphabet = alphabet::Alphabet::new(alphabet::URL_SAFE.as_str()).unwrap();
    let crazy_config = engine::GeneralPurposeConfig::new()
        .with_decode_allow_trailing_bits(true)
        .with_encode_padding(false)
        .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);
    engine::GeneralPurpose::new(&alphabet, crazy_config)
}

pub fn parse_threads(args: &Args) -> Option<IDInfo> {
    let id_bytes = pushid_base64_engine().decode(&args.id).ok()?;
    let id_int = u64::from_be_bytes(id_bytes.as_slice().try_into().ok()?);
    let timestamp_raw = bits64(id_int, 0, 43);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, Some(1314219988000));

    Some(IDInfo {
        id_type: "Threads Post ID".to_string(),
        standard: args.id.clone(),
        integer: Some(id_int as u128),
        parsed: Some("from base64".to_string()),
        size: 64,
        entropy: 21,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("3333333333333333333333333333333333333333333222222222222222222222".to_string()),
        ..Default::default()
    })
}
