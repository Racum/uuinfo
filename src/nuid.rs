use std::fmt::Write;

use crate::schema::{Args, IDInfo};

const ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn parse_nuid(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 22 || !args.id.chars().all(|c| ALPHABET.contains(c)) {
        return None;
    }
    Some(IDInfo {
        id_type: "NUID".to_string(),
        standard: args.id.to_string(),
        size: 176,
        entropy: 96,
        node1: Some(args.id[12..22].to_string()),
        hex: Some(hex::encode(args.id.as_bytes())),
        bits: Some(args.id.as_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(format!("{}{}", (0..(12 * 8)).map(|_| "2").collect::<String>(), (0..(10 * 8)).map(|_| "4").collect::<String>())),
        ..Default::default()
    })
}
