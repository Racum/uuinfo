use crate::schema::{Args, IDInfo};
use crate::utils::factor_size_hex_bits_color_from_text;

pub const NANOID_ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz";

pub fn parse_nanoid(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() < 2 || args.id.chars().count() > 36 {
        return None;
    }
    let mut alphabet_info = "Default alphabet";
    let mut default_alpha = true;
    let alphabet = match &args.alphabet {
        Some(alpha) => {
            alphabet_info = "Custom alphabet";
            default_alpha = false;
            alpha
        }
        None => NANOID_ALPHABET,
    };
    if !args.id.chars().all(|c| alphabet.contains(c)) {
        return None;
    }
    let mut default_len = false;
    let version = if args.id.chars().count() == 21 {
        default_len = true;
        Some(format!("{}, default length", alphabet_info))
    } else {
        Some(format!("{}, custom length ({})", alphabet_info, args.id.chars().count()))
    };
    let (size, hex, bits, color_map) = factor_size_hex_bits_color_from_text(&args.id);

    Some(IDInfo {
        id_type: "Nano ID".to_string(),
        version,
        standard: args.id.to_string(),
        parsed: Some("as ASCII".to_string()),
        size,
        entropy: size,
        hex,
        bits,
        color_map,
        high_confidence: default_alpha && default_len,
        ..Default::default()
    })
}
