use base32::Alphabet;
use std::fmt::Write;

use crate::schema::{Args, IDInfo};
use crate::utils::{bits128, milliseconds_to_seconds_and_iso8601};

pub fn parse_xid(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 20 {
        return None;
    }
    let xid_bytes = base32::decode(Alphabet::Rfc4648HexLower { padding: true }, &args.id)?;
    let mut xid_extra_bytes: Vec<u8> = [0u8, 0u8, 0u8, 0u8].to_vec();
    xid_extra_bytes.extend_from_slice(&xid_bytes);
    let xid_int: u128 = u128::from_be_bytes(xid_extra_bytes.try_into().unwrap());
    let timestamp_raw = bits128(xid_int, 32, 32);
    let machine_id = bits128(xid_int, 64, 24);
    let process_id = bits128(xid_int, 88, 16);
    let sequence = bits128(xid_int, 104, 24);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw as u64 * 1000, None);

    Some(IDInfo {
        id_type: "Xid".to_string(),
        standard: args.id.to_string(),
        integer: Some(xid_int),
        parsed: Some("from base32hex".to_string()),
        size: 96,
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        sequence: Some(sequence),
        node1: Some(format!("{} (Machine ID)", machine_id)),
        node2: Some(format!("{} (Process ID)", process_id)),
        hex: Some(hex::encode(xid_bytes.clone())),
        bits: Some(xid_bytes.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("333333333333333333333333333333334444444444444444444444445555555555555555666666666666666666666666".to_string()),
        ..Default::default()
    })
}
