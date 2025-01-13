use imei;
use mac_address::MacAddress;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::{fmt::Write, str::FromStr};

use crate::schema::{Args, IDInfo};
use crate::utils::factor_size_hex_bits_color_from_text;

pub fn parse_ipv4(args: &Args) -> Option<IDInfo> {
    let ip = Ipv4Addr::from_str(&args.id).ok()?;
    let mut version: Option<String> = None;
    if ip.is_loopback() {
        version = Some("Loopback".to_string());
    }
    if ip.is_private() {
        version = Some("Private".to_string());
    }

    Some(IDInfo {
        id_type: "IPv4 Address".to_string(),
        version,
        standard: args.id.clone(),
        integer: Some(ip.to_bits() as u128),
        parsed: Some("from integer parts".to_string()),
        size: 32,
        hex: Some(hex::encode(ip.to_bits().to_be_bytes())),
        bits: Some(ip.to_bits().to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some((0..32).map(|_| "0").collect::<String>()),
        ..Default::default()
    })
}

pub fn parse_ipv6(args: &Args) -> Option<IDInfo> {
    let ip = Ipv6Addr::from_str(&args.id).ok()?;
    let mut version: Option<String> = None;
    if ip.is_loopback() {
        version = Some("Loopback".to_string());
    }

    Some(IDInfo {
        id_type: "IPv6 Address".to_string(),
        version,
        standard: args.id.clone().to_lowercase(),
        integer: Some(ip.to_bits()),
        parsed: Some("from hex parts".to_string()),
        size: 128,
        hex: Some(hex::encode(ip.to_bits().to_be_bytes())),
        bits: Some(ip.to_bits().to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some((0..128).map(|_| "0").collect::<String>()),
        ..Default::default()
    })
}

fn fit_u64(bytes: Vec<u8>) -> u64 {
    let mut buffer: Vec<u8> = vec![0; 8 - bytes.len()];
    buffer.extend(bytes);
    let all_bytes: [u8; 8] = buffer.try_into().ok().unwrap();
    u64::from_be_bytes(all_bytes)
}

pub fn parse_mac(args: &Args) -> Option<IDInfo> {
    let mac = MacAddress::from_str(&args.id).ok()?;
    let mac_int = fit_u64((mac.bytes()[0..6]).to_vec());
    let mac_lower = args.id.clone().to_lowercase();
    let prefix = fit_u64((mac.bytes()[0..3]).to_vec());
    let sequence = fit_u64((mac.bytes()[3..6]).to_vec());

    Some(IDInfo {
        id_type: "MAC Address".to_string(),
        standard: mac_lower.clone(),
        integer: Some(mac_int as u128),
        parsed: Some("from hex parts".to_string()),
        size: 48,
        node1: Some(format!("{}, hex: {} (Manufacturer)", prefix, &mac_lower[..8])),
        sequence: Some(sequence as u128),
        hex: Some(hex::encode(mac.bytes())),
        bits: Some(mac.bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(format!("{}{}", (0..24).map(|_| "4").collect::<String>(), (0..24).map(|_| "6").collect::<String>(),)),
        ..Default::default()
    })
}

pub fn parse_imei(args: &Args) -> Option<IDInfo> {
    let no_dashes = args.id.replace("-", "");
    if no_dashes.chars().count() != 15 || !imei::valid(&no_dashes) {
        return None;
    }
    let id_int = no_dashes.trim().parse::<u128>().ok()?;
    let (size, hex, bits, _) = factor_size_hex_bits_color_from_text(&no_dashes);
    let standard = format!("{}-{}-{}-{}", &no_dashes[0..2], &no_dashes[2..8], &no_dashes[8..14], &no_dashes[14..15]);

    Some(IDInfo {
        id_type: "IMEI".to_string(),
        standard: standard.clone(),
        integer: Some(id_int),
        parsed: Some("as ASCII, no dashes".to_string()),
        size,
        node1: Some(format!("{} (Type Allocation Code)", &no_dashes[0..8])),
        node2: Some(format!("{} (Check Digit)", &no_dashes[14..15])),
        sequence: Some(no_dashes[8..14].parse::<u128>().unwrap()),
        hex,
        bits,
        color_map: Some(format!(
            "{}{}{}",
            (0..64).map(|_| "4").collect::<String>(),
            (0..48).map(|_| "6").collect::<String>(),
            (0..8).map(|_| "5").collect::<String>(),
        )),
        ..Default::default()
    })
}
