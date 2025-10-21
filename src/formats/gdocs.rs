use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use std::fmt::Write;

use crate::schema::{Args, IDInfo};

pub fn parse_gdocs(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() != 44 {
        return None;
    }
    let id_bytes = URL_SAFE_NO_PAD.decode(&args.id).ok()?;
    if !(id_bytes.len() == 33
        // Starts with '110101' :
        && id_bytes[0] >> 2 == 53
        // Ends with '00':
        && id_bytes[id_bytes.len() - 1] << 6 >> 6 == 0)
    {
        return None;
    }

    Some(IDInfo {
        id_type: "Google Docs ID".to_string(),
        standard: args.id.to_string(),
        parsed: Some("from base64".to_string()),
        size: 264,
        entropy: 256,
        hex: Some(hex::encode(&id_bytes)),
        bits: Some(id_bytes.iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(format!("000000{}00", (0..256).map(|_| "2").collect::<String>())),
        high_confidence: true,
        ..Default::default()
    })
}
