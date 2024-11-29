extern crate basen;
use crate::schema::{Args, IDInfo};
use nid::{Nanoid, ParseError};

pub fn parse_nanoid(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() > 36 {
        return None;
    }
    let version: Option<String>;
    let _: Option<Nanoid> = match Nanoid::try_from_str(&args.id) {
        Ok(value) => {
            version = Some("Default alphabet and length".to_string());
            Some(value)
        }
        Err(error) => match error {
            ParseError::InvalidLength { expected: _, actual } => {
                version = Some(format!("Default alphabet, custom length ({})", actual));
                None
            }
            _ => {
                return None;
            }
        },
    };
    Some(IDInfo {
        id_type: "Nano ID".to_string(),
        version,
        standard: args.id.to_string(),
        integer: None,
        short_uuid: None,
        base64: None,
        uuid_wrap: None,
        size: 0,
        entropy: 0,
        datetime: None,
        timestamp: None,
        sequence: None,
        node1: None,
        node2: None,
        hex: None,
        bits: None,
        color_map: None,
    })
}
