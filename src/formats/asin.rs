use basen::Base;
use std::fmt::Write;

use crate::schema::{Args, IDInfo};

pub const UPPER_BASE36: Base<36> = Base::new(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap();

pub fn parse_asin(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 10 {
        return None;
    }
    let id_int: u64 = UPPER_BASE36.decode_var_len(&args.id)?;

    Some(IDInfo {
        id_type: "ASIN (Amazon)".to_string(),
        standard: args.id.to_string(),
        integer: Some(id_int as u128),
        parsed: Some("as integer, from base36".to_string()),
        size: 64,
        entropy: 0,
        sequence: Some(id_int as u128),
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("6666666666666666666666666666666666666666666666666666666666666666".to_string()),
        high_confidence: id_int >= 1117159523352576,
        ..Default::default()
    })
}
