extern crate basen;
use crate::schema::{Args, IDInfo};

pub const NANOID_ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz";

pub fn parse_nanoid(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() < 2 || args.id.chars().count() > 36 {
        return None;
    }
    if !args.id.chars().all(|c| NANOID_ALPHABET.contains(c)) {
        return None;
    }
    let version = if args.id.chars().count() == 21 {
        Some("Default alphabet and length".to_string())
    } else {
        Some(format!("Default alphabet, custom length ({})", args.id.chars().count()))
    };

    Some(IDInfo {
        id_type: "Nano ID".to_string(),
        version,
        standard: args.id.to_string(),
        integer: None,
        short_uuid: None,
        base64: None,
        uuid_wrap: None,
        size: 0,
        entropy: 0,
        datetime: None,
        timestamp: None,
        sequence: None,
        node1: None,
        node2: None,
        hex: None,
        bits: None,
        color_map: None,
    })
}
