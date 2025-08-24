use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::fmt::Write;

use crate::schema::{Args, IDInfo};

/*
    The elements of YouTube’s alphabet match the URL_SAFE regex,
    but from outside the company, there is no way to know what is
    the actual order used, so, don’t trust the integer representation.
*/

pub fn parse_youtube(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 11 {
        return None;
    }
    let id_int: u64 = match URL_SAFE_NO_PAD.decode(&args.id) {
        Ok(value) => u64::from_be_bytes(value.as_slice().try_into().unwrap()),
        Err(_) => return None,
    };

    Some(IDInfo {
        id_type: "YouTube Video ID".to_string(),
        standard: args.id.to_string(),
        integer: Some(id_int.into()),
        parsed: Some("from base64".to_string()),
        size: 64,
        entropy: 64,
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("2222222222222222222222222222222222222222222222222222222222222222".to_string()),
        ..Default::default()
    })
}
