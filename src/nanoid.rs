use crate::schema::{Args, IDInfo};

pub const NANOID_ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz";

pub fn parse_nanoid(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() < 2 || args.id.chars().count() > 36 {
        return None;
    }
    let mut alphabet_info = "Default alphabet";
    let alphabet = match &args.alphabet {
        Some(alpha) => {
            alphabet_info = "Custom alphabet";
            alpha
        }
        None => NANOID_ALPHABET,
    };
    if !args.id.chars().all(|c| alphabet.contains(c)) {
        return None;
    }
    let version = if args.id.chars().count() == 21 {
        Some(format!("{}, default length", alphabet_info))
    } else {
        Some(format!("{}, custom length ({})", alphabet_info, args.id.chars().count()))
    };

    Some(IDInfo {
        id_type: "Nano ID".to_string(),
        version,
        standard: args.id.to_string(),
        ..Default::default()
    })
}
