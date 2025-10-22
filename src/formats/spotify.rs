use std::fmt::Write;
use uuid::Uuid;

use crate::schema::{Args, IDInfo};

pub fn parse_spotify(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 22 {
        return None;
    }
    let id_int = base62::decode(&args.id).ok()?;
    let uuid = Uuid::from_bytes(id_int.to_be_bytes());

    Some(IDInfo {
        id_type: "Spotify ID".to_string(),
        standard: args.id.to_string(),
        integer: Some(id_int),
        uuid_wrap: Some(uuid.to_string()),
        parsed: Some("from base62".to_string()),
        size: 128,
        entropy: 128,
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some((0..128).map(|_| "2").collect::<String>()),
        high_confidence: true,
        ..Default::default()
    })
}
