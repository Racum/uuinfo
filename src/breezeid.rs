use crate::schema::{Args, IDInfo};
use crate::utils::factor_size_hex_bits_color_from_text;

const ALPHABET: &str = "AEIUQWTYPSDFGRHJKLZXCVBNM2346789";

fn try_parse_color_dashes(id: &str, alphabet: &str) -> Option<(String, u16)> {
    let mut color_map: String = Default::default();
    let mut dashes: u16 = 0;
    for (i, c) in id.chars().enumerate() {
        if (i + 1) % 5 == 0 {
            if c == '-' {
                color_map.push_str("00000000");
                dashes += 1;
            } else {
                return None;
            }
        } else if alphabet.contains(c) {
            color_map.push_str("22222222");
        } else {
            return None;
        }
    }
    Some((color_map, dashes))
}

pub fn parse_breezeid(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() < 4 || args.id.chars().count() > 128 || args.id.ends_with('-') {
        return None;
    }
    let (color_map, dashes, version) = match try_parse_color_dashes(&args.id, ALPHABET) {
        Some(value) => (value.0, value.1, Some("Default alphabet".to_string())),
        None => match try_parse_color_dashes(&args.id, &ALPHABET.to_lowercase()) {
            Some(value) => (value.0, value.1, Some("Lowercase alphabet".to_string())),
            None => return None,
        },
    };

    let (size, hex, bits, _) = factor_size_hex_bits_color_from_text(&args.id);
    Some(IDInfo {
        id_type: "Breeze ID".to_string(),
        standard: args.id.to_string(),
        version,
        size,
        entropy: size - (dashes * 8),
        hex,
        bits,
        color_map: Some(color_map),
        ..Default::default()
    })
}
