use crate::utils::factor_size_hex_bits_color_from_text;
use basen::Base;

use crate::schema::{Args, IDInfo};

pub const UPPER_BASE36: Base<36> = Base::new(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap();

pub fn parse_slack(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() < 3 || args.id.chars().count() > 20 {
        return None;
    }
    let mut offset = 1;
    let version = match &args.id.chars().next()? {
        'A' => "App",
        'B' => "Bot",
        'C' | 'G' => "Channel",
        'D' => "Direct message",
        'E' => "Enterprise",
        'F' => "File",
        'T' => "Team",
        'U' => "User",
        'W' => match &args.id.chars().nth(1)? {
            'f' => {
                offset = 2;
                "Workflow"
            }
            _ => "User",
        },
        &_ => return None,
    };
    let prefix = &args.id[0..offset];
    let encoded = &args.id[offset..];
    let id_int: u64 = UPPER_BASE36.decode_var_len(encoded)?;
    let (size, hex, bits, _) = factor_size_hex_bits_color_from_text(&args.id);

    Some(IDInfo {
        id_type: "Slack ID".to_string(),
        version: Some(format!("{} ID", version)),
        standard: args.id.to_string(),
        integer: Some(id_int as u128),
        parsed: Some("as ASCII, with base62 parts".to_string()),
        size,
        entropy: (encoded.chars().count() * 8) as u16,
        node1: Some(format!("{} ({})", prefix, version)),
        hex,
        bits,
        color_map: Some(format!(
            "{}{}",
            (0..offset * 8).map(|_| "4").collect::<String>(),
            (0..(args.id.chars().count() - offset) * 8).map(|_| "2").collect::<String>(),
        )),
        high_confidence: true,
        ..Default::default()
    })
}
