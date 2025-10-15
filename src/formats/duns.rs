use regex::Regex;
use std::fmt::Write;

use crate::schema::{Args, IDInfo};

pub fn parse_duns(args: &Args) -> Option<IDInfo> {
    if !Regex::new(r"^[0-9]{2}\-[0-9]{3}\-[0-9]{4}$").unwrap().is_match(&args.id) {
        return None;
    }
    let id_int = args.id.replace("-", "").trim().parse::<u32>().ok()?;

    Some(IDInfo {
        id_type: "DUNS Number".to_string(),
        standard: args.id.to_string(),
        integer: Some(id_int as u128),
        parsed: Some("as integer".to_string()),
        size: 32,
        entropy: 32,
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("2222222222222222222222222222222222222222222222222222222222222222".to_string()),
        ..Default::default()
    })
}
