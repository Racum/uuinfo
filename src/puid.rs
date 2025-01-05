use basen::BASE36;

use crate::schema::{Args, IDInfo};
use crate::utils::{factor_size_hex_bits_color_from_text, milliseconds_to_seconds_and_iso8601};

pub fn parse_puid(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 24 {
        return None;
    }
    let timestamp_raw: u64 = BASE36.decode_var_len(&args.id[0..8])?;
    let _ = hex::decode(&args.id[8..14]).ok()?;
    let process_id: u64 = BASE36.decode_var_len(&args.id[14..18])?;
    let sequence: u64 = BASE36.decode_var_len(&args.id[18..24])?;
    let machine_id = &args.id[8..14];
    let (size, hex, bits, _) = factor_size_hex_bits_color_from_text(&args.id);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, None);

    Some(IDInfo {
        id_type: "Puid".to_string(),
        standard: args.id.clone(),
        size,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        node1: Some(format!("{} (Machine ID)", machine_id)),
        node2: Some(format!("{} (Process ID)", process_id)),
        sequence: Some(sequence as u128),
        hex,
        bits,
        color_map: Some(format!(
            "{}{}{}{}",
            (0..64).map(|_| "3").collect::<String>(),
            (0..48).map(|_| "4").collect::<String>(),
            (0..32).map(|_| "5").collect::<String>(),
            (0..48).map(|_| "6").collect::<String>(),
        )),
        ..Default::default()
    })
}

pub fn parse_shortpuid(args: &Args) -> Option<IDInfo> {
    match args.id.chars().count() {
        12 | 14 => (),
        _ => return None,
    }
    let timestamp_raw: u64 = BASE36.decode_var_len(&args.id[0..12])?;
    let version: Option<String>;
    let node1: Option<String>;
    let color_map: Option<String>;

    if args.id.chars().count() == 14 {
        let node_id: u64 = BASE36.decode_var_len(&args.id[12..14])?;
        version = Some("Short puid with node ID".to_string());
        node1 = Some(format!("{} (Node ID)", node_id));
        color_map = Some(format!("{}{}", (0..96).map(|_| "3").collect::<String>(), (0..16).map(|_| "4").collect::<String>(),));
    } else {
        version = Some("Short puid without node ID".to_string());
        node1 = None;
        color_map = Some((0..96).map(|_| "3").collect::<String>());
    }
    let (size, hex, bits, _) = factor_size_hex_bits_color_from_text(&args.id);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw / 1_000_000, None);

    Some(IDInfo {
        id_type: "Puid".to_string(),
        version,
        standard: args.id.clone(),
        size,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        node1,
        hex,
        bits,
        color_map,
        ..Default::default()
    })
}

pub fn parse_puid_any(args: &Args) -> Option<IDInfo> {
    match args.id.chars().count() {
        24 => parse_puid(args),
        14 | 12 => parse_shortpuid(args),
        _ => None,
    }
}
