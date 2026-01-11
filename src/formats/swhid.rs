use crate::schema::{Args, IDInfo};
use std::fmt::Write;

pub fn parse_swhid(args: &Args) -> Option<IDInfo> {
    let meta_parts: &Vec<&str> = &args.id.split(';').collect();
    let parts: Vec<_> = meta_parts.first()?.split(':').collect();
    if *parts.first()? != "swh" {
        return None;
    }
    if *parts.get(1)? != "1" {
        return None;
    }
    let obj_type = match *parts.get(2)? {
        "snp" => "Snapshot",
        "rel" => "Release",
        "rev" => "Revision",
        "dir" => "Directory",
        "cnt" => "Content",
        _ => return None,
    };
    let hex_id = parts.get(3)?;
    if hex_id.chars().count() != 40 {
        return None;
    }
    let id_bytes = hex::decode(hex_id).ok()?;

    Some(IDInfo {
        id_type: "SWHID (Software Hash ID)".to_string(),
        version: Some(format!("Schema: {}, object type: {}", parts.get(1)?, obj_type)),
        standard: format!("swh:1:{}:{}", parts.get(2)?, hex_id),
        parsed: Some("from hex".to_string()),
        size: 160,
        entropy: 160,
        hex: Some(hex::encode(&id_bytes)),
        bits: Some(id_bytes.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some((0..160).map(|_| "2").collect::<String>()),
        high_confidence: true,
        ..Default::default()
    })
}
