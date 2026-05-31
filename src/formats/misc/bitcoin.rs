use base58::FromBase58;
use sha2::{Digest, Sha256};
use std::fmt::Write;

use crate::schema::{Args, IDInfo};
use crate::utils::repeat_char;

const BECH32_ALPHABET: &[u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";

fn bech32_char_value(c: u8) -> Option<u8> {
    BECH32_ALPHABET.iter().position(|&x| x == c).map(|v| v as u8)
}

pub fn parse_bitcoin(args: &Args) -> Option<IDInfo> {
    let id = &args.id;
    if id.starts_with("bc1") || id.starts_with("BC1") {
        return parse_bitcoin_bech32(args);
    }
    let bytes = id.from_base58().ok()?;
    if bytes.len() != 25 {
        return None;
    }
    let payload = bytes.get(..21)?;
    let checksum = bytes.get(21..25)?;
    let hash = Sha256::digest(Sha256::digest(payload));
    if checksum != hash.get(..4)? {
        return None;
    }
    let version_byte = *bytes.first()?;
    let version = match version_byte {
        0x00 => "Legacy (P2PKH)",
        0x05 => "Nested SegWit (P2SH)",
        _ => return None,
    };
    let size = (bytes.len() * 8) as u16;
    let id_type = if id == "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa" {
        "Bitcoin Address (from Satoshi Nakamoto)"
    } else {
        "Bitcoin Address"
    };

    Some(IDInfo {
        id_type: id_type.to_string(),
        version: Some(version.to_string()),
        standard: id.clone(),
        parsed: Some("from base58check".to_string()),
        size,
        entropy: 160,
        node1: Some(format!("{} (Checksum)", hex::encode(checksum))),
        hex: Some(hex::encode(&bytes)),
        bits: Some(bytes.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(repeat_char('1', 8) + &repeat_char('2', 160) + &repeat_char('4', 32)),
        high_confidence: true,
        ..Default::default()
    })
}

fn parse_bitcoin_bech32(args: &Args) -> Option<IDInfo> {
    let id = args.id.to_lowercase();
    if id.len() < 14 || id.len() > 74 {
        return None;
    }

    let hrp_end = id.rfind('1')?;
    let hrp = id.get(..hrp_end)?;
    if hrp != "bc" {
        return None;
    }
    let data_part = id.get(hrp_end + 1..)?;
    if data_part.len() < 6 {
        return None;
    }
    let values: Vec<u8> = data_part.bytes().map(bech32_char_value).collect::<Option<Vec<u8>>>()?;
    let witness_version = *values.first()?;
    let version = match witness_version {
        0 => "Native SegWit (Bech32)".to_string(),
        1 => "Taproot (P2TR)".to_string(),
        2..=16 => format!("SegWit v{}", witness_version),
        _ => return None,
    };
    let checksum_chars = data_part.get(data_part.len() - 6..)?;
    let checksum_values = values.get(values.len() - 6..)?;
    let witness_prog_values = values.get(1..values.len() - 6)?;
    let witness_prog_bytes = convert_bits(witness_prog_values, 5, 8, false)?;
    if witness_prog_bytes.len() < 2 || witness_prog_bytes.len() > 40 {
        return None;
    }
    if witness_version == 0 && witness_prog_bytes.len() != 20 && witness_prog_bytes.len() != 32 {
        return None;
    }
    let checksum_bytes = convert_bits(checksum_values, 5, 8, true)?;
    let mut all_bytes = vec![witness_version];
    all_bytes.extend_from_slice(&witness_prog_bytes);
    all_bytes.extend_from_slice(&checksum_bytes);
    let version_bits: usize = 8;
    let program_bits = witness_prog_bytes.len() * 8;
    let checksum_bits = checksum_bytes.len() * 8;
    let size = (all_bytes.len() * 8) as u16;
    let entropy = program_bits as u16;

    Some(IDInfo {
        id_type: "Bitcoin Address".to_string(),
        version: Some(version),
        standard: id.clone(),
        parsed: Some("from bech32".to_string()),
        size,
        entropy,
        node1: Some(format!("{} (Checksum, {} in hex)", checksum_chars, hex::encode(&checksum_bytes))),
        hex: Some(hex::encode(&all_bytes)),
        bits: Some(all_bytes.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(repeat_char('1', version_bits) + &repeat_char('2', program_bits) + &repeat_char('4', checksum_bits)),
        high_confidence: true,
        ..Default::default()
    })
}

fn convert_bits(data: &[u8], from: u32, to: u32, pad: bool) -> Option<Vec<u8>> {
    let mut acc: u32 = 0;
    let mut bits: u32 = 0;
    let mut result = Vec::new();
    let max = (1u32 << to) - 1;
    for &value in data {
        acc = (acc << from) | u32::from(value);
        bits += from;
        while bits >= to {
            bits -= to;
            result.push(((acc >> bits) & max) as u8);
        }
    }
    if pad {
        if bits > 0 {
            result.push(((acc << (to - bits)) & max) as u8);
        }
    } else if bits >= from || ((acc << (to - bits)) & max) != 0 {
        return None;
    }
    Some(result)
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
    fn test_parse_bitcoin_p2pkh() {
        let args = make_args("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
        let result = parse_bitcoin(&args).unwrap();
        assert_eq!(result.id_type, "Bitcoin Address (from Satoshi Nakamoto)");
        assert!(result.version.unwrap().contains("Legacy"));
    }

    #[test]
    fn test_parse_bitcoin_bech32() {
        let args = make_args("bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4");
        let result = parse_bitcoin(&args).unwrap();
        assert_eq!(result.id_type, "Bitcoin Address");
        assert!(result.version.unwrap().contains("Native SegWit"));
    }

    #[test]
    fn test_parse_bitcoin_p2sh() {
        let args = make_args("3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy");
        let result = parse_bitcoin(&args).unwrap();
        assert_eq!(result.id_type, "Bitcoin Address");
        assert!(result.version.unwrap().contains("Nested SegWit"));
    }

    #[test]
    fn test_parse_bitcoin_taproot() {
        let args = make_args("bc1p5cyxnuxmeuwuvkwfem96lqzszee2t0p688raqku9m3f8uafvhvqqhkz45z");
        let result = parse_bitcoin(&args).unwrap();
        assert_eq!(result.id_type, "Bitcoin Address");
        assert!(result.version.unwrap().contains("Taproot"));
    }

    #[test]
    fn test_reject_invalid_checksum() {
        let args = make_args("1A1zP1eP5QGefi2DMPTfTL5SLmv7Divfxx");
        assert!(parse_bitcoin(&args).is_none());
    }

    #[test]
    fn test_reject_non_bitcoin() {
        let args = make_args("notabitcoinaddress");
        assert!(parse_bitcoin(&args).is_none());
    }
}
