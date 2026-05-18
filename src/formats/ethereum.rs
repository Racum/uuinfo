use std::fmt::Write;
use tiny_keccak::{Hasher, Keccak};

use crate::schema::{Args, IDInfo};
use crate::utils::repeat_char;

fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(data);
    hasher.finalize(&mut output);
    output
}

fn eip55_checksum(hex_part: &str) -> String {
    let lower = hex_part.to_lowercase();
    let hash = keccak256(lower.as_bytes());
    let hash_hex = hex::encode(hash);
    let mut result = String::with_capacity(40);
    for (i, c) in lower.chars().enumerate() {
        if c.is_ascii_digit() {
            result.push(c);
        } else {
            let nibble = hash_hex.as_bytes().get(i).copied().unwrap_or(b'0');
            if nibble >= b'8' {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c);
            }
        }
    }
    result
}

fn check_eip55(hex_part: &str) -> Option<bool> {
    if hex_part == hex_part.to_lowercase() || hex_part == hex_part.to_uppercase() {
        return None;
    }
    Some(hex_part == eip55_checksum(hex_part))
}

pub fn parse_ethereum(args: &Args) -> Option<IDInfo> {
    let id = &args.id;
    if !id.starts_with("0x") && !id.starts_with("0X") {
        return None;
    }
    let hex_part = id.get(2..)?;
    if hex_part.len() != 40 {
        return None;
    }
    let bytes = hex::decode(hex_part).ok()?;
    let eip55 = check_eip55(hex_part);
    let version = match eip55 {
        Some(true) => "EIP-55 (valid checksum)",
        Some(false) => "EIP-55 (invalid checksum)",
        None => "No checksum",
    };

    Some(IDInfo {
        id_type: "Ethereum Address".to_string(),
        version: Some(version.to_string()),
        standard: format!("0x{}", hex_part),
        parsed: Some("from hex".to_string()),
        size: 160,
        entropy: 160,
        hex: Some(hex::encode(&bytes)),
        bits: Some(bytes.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(repeat_char('2', 160)),
        high_confidence: eip55 != Some(false),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Output;

    fn make_args(id: &str) -> Args {
        Args {
            id: id.to_string(),
            output: Output::Card,
            force: None,
            everything: false,
            compare: false,
            alphabet: None,
            relative: false,
            salt: None,
            epoch: None,
        }
    }

    #[test]
    fn test_parse_ethereum() {
        let args = make_args("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
        let result = parse_ethereum(&args).unwrap();
        assert_eq!(result.id_type, "Ethereum Address");
        assert_eq!(result.size, 160);
    }

    #[test]
    fn test_eip55_valid() {
        let args = make_args("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
        let result = parse_ethereum(&args).unwrap();
        assert_eq!(result.version.unwrap(), "EIP-55 (valid checksum)");
    }

    #[test]
    fn test_eip55_invalid() {
        let args = make_args("0xd8da6BF26964aF9D7eEd9e03E53415D37aA96045");
        let result = parse_ethereum(&args).unwrap();
        assert_eq!(result.version.unwrap(), "EIP-55 (invalid checksum)");
        assert!(!result.high_confidence);
    }

    #[test]
    fn test_all_lowercase_no_checksum() {
        let args = make_args("0xd8da6bf26964af9d7eed9e03e53415d37aa96045");
        let result = parse_ethereum(&args).unwrap();
        assert_eq!(result.version.unwrap(), "No checksum");
    }

    #[test]
    fn test_all_uppercase_no_checksum() {
        let args = make_args("0xD8DA6BF26964AF9D7EED9E03E53415D37AA96045");
        let result = parse_ethereum(&args).unwrap();
        assert_eq!(result.version.unwrap(), "No checksum");
    }

    #[test]
    fn test_reject_invalid_ethereum() {
        let args = make_args("0xINVALID");
        assert!(parse_ethereum(&args).is_none());
    }
}
