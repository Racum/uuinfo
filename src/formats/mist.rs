use std::fmt::Write;

use crate::schema::{Args, IDInfo};
use crate::utils::{bits64, repeat_char};

pub fn parse_mist(args: &Args) -> Option<IDInfo> {
    let id_int = args.id.trim().parse::<u64>().ok()?;
    let counter = bits64(id_int, 1, 47);
    let salt1 = bits64(id_int, 48, 8);
    let salt2 = bits64(id_int, 56, 8);

    Some(IDInfo {
        id_type: "Mist".to_string(),
        standard: id_int.to_string(),
        integer: Some(id_int as u128),
        parsed: Some("as integer".to_string()),
        size: 64,
        sequence: Some(counter as u128),
        node1: Some(format!("{} (Salt 1)", salt1)),
        node2: Some(format!("{} (Salt 2)", salt2)),
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(repeat_char('0', 1) + &repeat_char('6', 47) + &repeat_char('4', 8) + &repeat_char('5', 8)),
        high_confidence: counter > 0,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mist() {
        let result = parse_mist(&Args {
            id: "171671".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.id_type, "Mist");
        assert_eq!(result.size, 64);
        assert_eq!(result.sequence, Some(2));
        assert_eq!(result.node1, Some("158 (Salt 1)".to_string()));
        assert_eq!(result.node2, Some("151 (Salt 2)".to_string()));
        assert!(result.high_confidence);
    }

    #[test]
    fn test_parse_mist_zero_counter_low_confidence() {
        let result = parse_mist(&Args {
            id: "255".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.sequence, Some(0));
        assert!(!result.high_confidence);
    }

    #[test]
    fn test_parse_mist_non_numeric() {
        assert!(
            parse_mist(&Args {
                id: "not-a-number".to_string(),
                ..Default::default()
            })
            .is_none()
        );
    }
}
