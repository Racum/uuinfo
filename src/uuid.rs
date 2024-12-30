use base64::{engine::general_purpose::URL_SAFE, engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use short_uuid::{CustomTranslator, ShortUuidCustom};
use std::fmt::Write;
use uuid::{Uuid, Variant};
use uuid25::Uuid25;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

pub const SHORT_UUID_ALPHABET: &str = "23456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
pub const COLOR_MAP_UUID_GENERIC: &str = "22222222222222222222222222222222222222222222222211112222222222220022222222222222222222222222222222222222222222222222222222222222";
pub const COLOR_MAP_UUID_NO_RFC: &str = "22222222222222222222222222222222222222222222222222222222222222220022222222222222222222222222222222222222222222222222222222222222";
pub const COLOR_MAP_UUID_16: &str = "33333333333333333333333333333333333333333333333311113333333333330066666666666666444444444444444444444444444444444444444444444444";
pub const COLOR_MAP_UUID_7: &str = "33333333333333333333333333333333333333333333333311112222222222220022222222222222222222222222222222222222222222222222222222222222";

pub fn parse_uuid(args: &Args) -> Option<IDInfo> {
    let uuid = match Uuid::try_parse(&args.id) {
        Ok(value) => value,
        Err(_) => return None,
    };

    let id_type: String;
    let mut version: Option<String> = None;
    let mut entropy: u16 = 0;
    let mut color_map: Option<String> = Some(COLOR_MAP_UUID_GENERIC.to_string());
    let mut datetime: Option<String> = None;
    let mut timestamp: Option<String> = None;
    let translator = CustomTranslator::new(SHORT_UUID_ALPHABET).unwrap();
    let short = ShortUuidCustom::from_uuid(&uuid, &translator).to_string();

    if uuid.is_nil() {
        id_type = "Nil UUID (all zeros)".to_string();
    } else if uuid.is_max() {
        id_type = "Max UUID (all ones)".to_string();
    } else {
        match uuid.get_variant() {
            Variant::NCS => {
                id_type = "NCS UUID".to_string();
                color_map = Some(COLOR_MAP_UUID_NO_RFC.to_string());
            }
            Variant::Microsoft => {
                id_type = "Microsoft GUID".to_string();
                color_map = Some(COLOR_MAP_UUID_NO_RFC.to_string());
            }
            Variant::RFC4122 => {
                id_type = match uuid.get_version_num() {
                    1..6 => "UUID (RFC-4122)".to_string(),
                    6..9 => "UUID (RFC-9562)".to_string(),
                    _ => "UUID".to_string(),
                };
                version = match uuid.get_version_num() {
                    1 => Some("1 (timestamp and node)".to_string()),
                    2 => Some("2 (DCE security)".to_string()),
                    3 => Some("3 (MD5 hash)".to_string()),
                    4 => Some("4 (random)".to_string()),
                    5 => Some("5 (SHA-1 hash)".to_string()),
                    6 => Some("6 (sortable timestamp and node)".to_string()),
                    7 => Some("7 (sortable timestamp and random)".to_string()),
                    8 => Some("8 (custom)".to_string()),
                    ver => Some(format!("{ver} (out of spec)")),
                };
                entropy = match uuid.get_version_num() {
                    1 | 6 => 0,
                    7 => 74,
                    _ => 122,
                };
                color_map = match uuid.get_version_num() {
                    1 | 6 => Some(COLOR_MAP_UUID_16.to_string()),
                    7 => Some(COLOR_MAP_UUID_7.to_string()),
                    _ => Some(COLOR_MAP_UUID_GENERIC.to_string()),
                };
            }
            _ => {
                id_type = "Unknown UUID-like".to_string();
                color_map = Some(COLOR_MAP_UUID_NO_RFC.to_string());
            }
        }
    }

    if uuid.get_variant() == Variant::RFC4122 {
        if let Some(ts) = uuid.get_timestamp() {
            let secs: i64 = ts.to_unix().0.try_into().unwrap();
            let nanos: u32 = ts.to_unix().1;
            let ms = (secs * 1000) as u64 + (nanos / 1_000_000) as u64;
            let formatted_time = milliseconds_to_seconds_and_iso8601(ms, None);
            timestamp = Some(formatted_time.0);
            datetime = Some(formatted_time.1);
        }
    }

    let node1: Option<String> = match uuid.get_node_id() {
        Some(value) => {
            let mut node1_buff: String = "".to_string();
            for (i, c) in value.into_iter().enumerate() {
                node1_buff.push_str(&hex::encode(vec![c]));
                if i < 5 {
                    node1_buff.push(':');
                }
            }
            Some(node1_buff)
        }
        None => None,
    };

    let sequence: Option<u128> = match uuid.get_version_num() {
        1 | 6 => {
            let sequence_bytes = &uuid.as_bytes()[8..10];
            let mut first_byte = sequence_bytes[0];
            first_byte <<= 2;
            first_byte >>= 2;
            Some(u16::from_be_bytes([first_byte, sequence_bytes[1]]).into())
        }
        _ => None,
    };

    Some(IDInfo {
        id_type,
        version,
        standard: uuid.to_string(),
        integer: Some(uuid.as_u128()),
        short_uuid: Some(short.to_string()),
        base64: Some(URL_SAFE.encode(uuid.to_bytes_le())),
        size: 128,
        entropy,
        datetime,
        timestamp,
        sequence,
        node1,
        hex: Some(hex::encode(uuid.as_bytes())),
        bits: Some(uuid.as_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map,
        ..Default::default()
    })
}

pub fn parse_short_uuid(args: &Args) -> Option<IDInfo> {
    let translator = CustomTranslator::new(SHORT_UUID_ALPHABET).unwrap();
    let suuid = match ShortUuidCustom::parse_str(&args.id, &translator) {
        Ok(value) => value,
        Err(_) => return None,
    };
    let uuid_str = match suuid.clone().to_uuid(&translator) {
        Ok(value) => value.to_string(),
        Err(_) => return None,
    };

    let mut new_args: Args = args.clone();
    new_args.id = uuid_str.clone();
    let mut id_info = match parse_uuid(&new_args) {
        Some(value) => value,
        None => return None,
    };
    id_info.id_type = format!("ShortUUID of {}", id_info.id_type);
    id_info.standard = args.id.to_string();
    id_info.uuid_wrap = Some(uuid_str);

    Some(id_info)
}

pub fn parse_base64_uuid(args: &Args) -> Option<IDInfo> {
    let mut padded = true;
    let uuid = match URL_SAFE.decode(&args.id) {
        Ok(value) => match Uuid::from_slice_le(value.as_slice()) {
            Ok(value) => value,
            Err(_) => return None,
        },
        Err(_) => match URL_SAFE_NO_PAD.decode(&args.id) {
            Ok(value) => {
                padded = false;
                match Uuid::from_slice_le(value.as_slice()) {
                    Ok(value) => value,
                    Err(_) => return None,
                }
            }
            Err(_) => return None,
        },
    };

    let mut new_args: Args = args.clone();
    new_args.id = uuid.to_string();
    let mut id_info = match parse_uuid(&new_args) {
        Some(value) => value,
        None => return None,
    };

    if padded {
        id_info.id_type = format!("Padded Base64 of {}", id_info.id_type);
    } else {
        id_info.id_type = format!("Unpadded Base64 of {}", id_info.id_type);
    }
    id_info.standard = id_info.base64.clone().unwrap();
    id_info.base64 = None;
    id_info.uuid_wrap = Some(uuid.to_string());

    Some(id_info)
}

pub fn parse_uuid25(args: &Args) -> Option<IDInfo> {
    let uuid_str = match Uuid25::parse(&args.id) {
        Ok(value) => value.to_hyphenated().to_string(),
        Err(_) => return None,
    };
    let mut new_args: Args = args.clone();
    new_args.id = uuid_str.clone();
    let mut id_info = match parse_uuid(&new_args) {
        Some(value) => value,
        None => return None,
    };
    id_info.id_type = format!("Uuid25 of {}", id_info.id_type);
    id_info.standard = args.id.to_string();
    id_info.uuid_wrap = Some(uuid_str);
    Some(id_info)
}
