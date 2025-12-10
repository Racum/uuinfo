use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;
use std::fmt::Write;

use base32::Alphabet;

pub fn parse_orderlyid(args: &Args) -> Option<IDInfo> {
    // Parts formanting, validation, parsing:
    let parts: &Vec<&str> = &args.id.split('_').collect();
    if parts.len() != 2 {
        return None;
    };
    let prefix = parts[0];
    let parts: &Vec<&str> = &parts[1].split('-').collect();
    let value = parts[0];
    if prefix.len() == 1 || prefix.len() > 31 || value.len() != 32 {
        return None;
    };
    let mut checksum = "no checksum";
    if parts.len() == 2 {
        if parts[1].len() != 4 {
            return None;
        }
        checksum = "with checksum";
    }
    let parsed_bytes = base32::decode(Alphabet::Crockford, value)?;

    // Timestamp, flags and tenant:
    let mut buffer1: Vec<u8> = vec![0; 16];
    buffer1[..9].copy_from_slice(&parsed_bytes[0..9]);
    let ts_flags_tenant = u128::from_be_bytes(buffer1.try_into().ok()?);
    let timestamp_raw = ts_flags_tenant >> 80;
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw as u64, Some(1577836800000));
    let flags = ts_flags_tenant << 48 >> 120;
    let tenant = ts_flags_tenant << 56 >> 112;
    let version = match flags >> 6 {
        0 => "1",
        _ => "unknown",
    };
    let privacy_bucket = match flags << 2 >> 7 {
        1 => "privacy on",
        _ => "privacy off",
    };

    // Sequence, shard and random:
    let mut buffer2: Vec<u8> = vec![0; 16];
    buffer2[..11].copy_from_slice(&parsed_bytes[9..20]);
    let seq_shard_random = u128::from_be_bytes(buffer2.try_into().ok()?);
    let sequence = seq_shard_random >> 116;
    let shard = seq_shard_random << 12 >> 112;

    Some(IDInfo {
        // id_type: "OrderlyID".to_string(),
        id_type: format!("OrderlyID, type {}", prefix),
        version: Some(format!("Version {}, {}, {}", version, privacy_bucket, checksum)),
        standard: args.id.to_string(),
        parsed: Some("from Crockford's base32".to_string()),
        size: 160,
        entropy: 60,
        datetime: Some(datetime),
        timestamp: Some(timestamp.to_string()),
        node1: Some(format!("{} (Tenant)", tenant)),
        node2: Some(format!("{} (Shard)", shard)),
        sequence: Some(sequence as u128),
        hex: Some(hex::encode(parsed_bytes.clone())),
        bits: Some(parsed_bytes.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(format!(
            "{}{}{}{}{}{}{}",
            (0..48).map(|_| "3").collect::<String>(),
            (0..3).map(|_| "1").collect::<String>(),
            (0..5).map(|_| "0").collect::<String>(),
            (0..16).map(|_| "4").collect::<String>(),
            (0..12).map(|_| "6").collect::<String>(),
            (0..16).map(|_| "5").collect::<String>(),
            (0..60).map(|_| "2").collect::<String>(),
        )),
        high_confidence: true,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_tenant_shard_seq_0_no_checksum() {
        let result = parse_orderlyid(&Args {
            id: "order_00jc1gmm00000000000028t5cy4tqkff".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.version.unwrap(), "Version 1, privacy off, no checksum".to_string());
        assert_eq!(result.timestamp, Some("1735689600.000".to_string()));
        assert_eq!(result.node1, Some("0 (Tenant)".to_string()));
        assert_eq!(result.node2, Some("0 (Shard)".to_string()));
        assert_eq!(result.sequence, Some(0));
        assert_eq!(result.hex.unwrap()[24..40], "0123456789abcdef".to_string());
    }

    #[test]
    fn test_tenant_42_shard_7_seq_15_no_checksum() {
        let result = parse_orderlyid(&Args {
            id: "user_00jc1gmmfc000ag0y007xbdyxz5fxzqd".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.version.unwrap(), "Version 1, privacy off, no checksum".to_string());
        assert_eq!(result.timestamp, Some("1735689600.123".to_string()));
        assert_eq!(result.node1, Some("42 (Tenant)".to_string()));
        assert_eq!(result.node2, Some("7 (Shard)".to_string()));
        assert_eq!(result.sequence, Some(15));
        assert_eq!(result.hex.unwrap()[24..40], "7eadbeefcafefeed".to_string());
    }

    #[test]
    fn test_tenant_1_shard_65535_max_seq_wrap_0_checksum_enabled() {
        let result = parse_orderlyid(&Args {
            id: "shipment_00jc1gmqww0000801zzzzzzzzzzzzzzz-a0f4".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.version.unwrap(), "Version 1, privacy off, with checksum".to_string());
        assert_eq!(result.timestamp, Some("1735689600.999".to_string()));
        assert_eq!(result.node1, Some("1 (Tenant)".to_string()));
        assert_eq!(result.node2, Some("65535 (Shard)".to_string()));
        assert_eq!(result.sequence, Some(0));
        assert_eq!(result.hex.unwrap()[24..40], "ffffffffffffffff".to_string());
    }

    #[test]
    fn test_privacy_flag_set_tenant_0_shard_0_seq_123() {
        let result = parse_orderlyid(&Args {
            id: "event_00jc1gwnt0g00007p000qkffnf6yzayd".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.version.unwrap(), "Version 1, privacy on, no checksum".to_string());
        assert_eq!(result.timestamp, Some("1735689666.000".to_string()));
        assert_eq!(result.node1, Some("0 (Tenant)".to_string()));
        assert_eq!(result.node2, Some("0 (Shard)".to_string()));
        assert_eq!(result.sequence, Some(123));
        assert_eq!(result.hex.unwrap()[24..40], "0bcdefabcdefabcd".to_string());
    }

    #[test]
    fn test_invalid_bad_checksum() {
        let result = parse_orderlyid(&Args {
            id: "order_00jc1gmm00000000000028t5cy4tqkff-xxxx".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.version.unwrap(), "Version 1, privacy off, with checksum".to_string());
        assert_eq!(result.timestamp, Some("1735689600.000".to_string()));
        assert_eq!(result.node1, Some("0 (Tenant)".to_string()));
        assert_eq!(result.node2, Some("0 (Shard)".to_string()));
        assert_eq!(result.sequence, Some(0));
        assert_eq!(result.hex.unwrap()[24..40], "0123456789abcdef".to_string());
    }
}
