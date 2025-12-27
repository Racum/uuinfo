use isbn::{Isbn10, Isbn13};
use std::str::FromStr;

use crate::id_format::pick_first_valid;
use crate::schema::{Args, IDInfo};
use crate::utils::factor_size_hex_bits_color_from_text;

pub fn parse_isbn13(args: &Args) -> Option<IDInfo> {
    let id = Isbn13::from_str(&args.id).ok()?;
    let binding = id.hyphenate().ok()?;
    let parts = binding.split('-').collect::<Vec<_>>();
    let node1_size = parts.first()?.chars().count() + parts.get(2)?.chars().count();
    let node2_size = parts.get(2)?.chars().count();
    let sequence_size = parts.get(3)?.chars().count();
    let (size, hex, bits, _) = factor_size_hex_bits_color_from_text(&id.to_string());

    Some(IDInfo {
        id_type: "ISBN-13".to_string(),
        standard: id.hyphenate().unwrap().to_string(),
        integer: id.to_string().parse::<u128>().ok(),
        parsed: Some("as ASCII, no dashes".to_string()),
        size,
        node1: Some(id.registration_group().unwrap_or("-").to_string()),
        node2: Some(format!("{} (Publisher ID)", parts.get(2)?)),
        sequence: parts.get(3)?.parse::<u128>().ok(),
        hex,
        bits,
        color_map: Some(format!(
            "{}{}{}{}",
            (0..(node1_size * 8)).map(|_| "4").collect::<String>(),
            (0..(node2_size * 8)).map(|_| "5").collect::<String>(),
            (0..(sequence_size * 8)).map(|_| "6").collect::<String>(),
            (0..8).map(|_| "0").collect::<String>(),
        )),
        high_confidence: true,
        ..Default::default()
    })
}

pub fn parse_isbn10(args: &Args) -> Option<IDInfo> {
    let id = Isbn10::from_str(&args.id).ok()?;
    let binding = id.hyphenate().ok()?;
    let parts = binding.split('-').collect::<Vec<_>>();
    let node1_size = parts.first()?.chars().count();
    let node2_size = parts.get(1)?.chars().count();
    let sequence_size = parts.get(2)?.chars().count();
    let (size, hex, bits, _) = factor_size_hex_bits_color_from_text(&id.to_string());

    Some(IDInfo {
        id_type: "ISBN-10".to_string(),
        standard: id.hyphenate().unwrap().to_string(),
        integer: id.to_string().parse::<u128>().ok(),
        parsed: Some("as ASCII, no dashes".to_string()),
        size,
        node1: Some(id.registration_group().unwrap_or("-").to_string()),
        node2: Some(format!("{} (Publisher ID)", parts.get(1)?)),
        sequence: parts.get(2)?.parse::<u128>().ok(),
        hex,
        bits,
        color_map: Some(format!(
            "{}{}{}{}",
            (0..(node1_size * 8)).map(|_| "4").collect::<String>(),
            (0..(node2_size * 8)).map(|_| "5").collect::<String>(),
            (0..(sequence_size * 8)).map(|_| "6").collect::<String>(),
            (0..8).map(|_| "0").collect::<String>(),
        )),
        high_confidence: true,
        ..Default::default()
    })
}

pub fn parse_isbn(args: &Args) -> Option<IDInfo> {
    pick_first_valid(args, vec![parse_isbn13, parse_isbn10])
}
