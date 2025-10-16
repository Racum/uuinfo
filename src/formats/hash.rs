use std::fmt::Write;

use crate::schema::{Args, IDInfo};

pub fn parse_hash(args: &Args) -> Option<IDInfo> {
    match args.id.chars().count() {
        32 | 40 | 56 | 64 | 96 | 128 => (),
        _ => return None,
    }
    let id_bytes = hex::decode(args.id.clone()).ok()?;
    let bits = id_bytes.len() * 8;
    let version = match bits {
        128 => Some("Probably MD5".to_string()),
        160 => Some("Probably SHA-1".to_string()),
        224 => Some("Probably SHA-224".to_string()),
        256 => Some("Probably SHA-256".to_string()),
        384 => Some("Probably SHA-384".to_string()),
        512 => Some("Probably SHA-512".to_string()),
        _ => None,
    };

    Some(IDInfo {
        id_type: "Hex-encoded Hash".to_string(),
        version,
        standard: args.id.to_string(),
        parsed: Some("from hex".to_string()),
        size: bits as u16,
        entropy: bits as u16,
        hex: Some(hex::encode(&id_bytes)),
        bits: Some(id_bytes.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some((0..bits).map(|_| "2").collect::<String>()),
        high_confidence: false,
        ..Default::default()
    })
}
