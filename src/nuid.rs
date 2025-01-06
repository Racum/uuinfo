use crate::schema::{Args, IDInfo};
use crate::utils::factor_size_hex_bits_color_from_text;

const ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn parse_nuid(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 22 || !args.id.chars().all(|c| ALPHABET.contains(c)) {
        return None;
    }
    let (_, hex, bits, _) = factor_size_hex_bits_color_from_text(&args.id);
    Some(IDInfo {
        id_type: "NUID".to_string(),
        standard: args.id.to_string(),
        parsed: Some("as ASCII".to_string()),
        size: 176,
        entropy: 96,
        node1: Some(args.id[12..22].to_string()),
        hex,
        bits,
        color_map: Some(format!("{}{}", (0..(12 * 8)).map(|_| "2").collect::<String>(), (0..(10 * 8)).map(|_| "4").collect::<String>())),
        ..Default::default()
    })
}
