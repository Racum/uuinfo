use cid::{Cid, Version};
use std::fmt::Write;
use std::str::FromStr;

use crate::schema::{Args, IDInfo};

pub fn parse_ipfs(args: &Args) -> Option<IDInfo> {
    let cid = Cid::from_str(&args.id).ok()?;
    let (version, parsed) = match (cid.version(), cid.codec()) {
        (Version::V0, _) => ("CID v0", "from base58btc"),
        (Version::V1, 114) => ("CID v1 (IPNS)", "from base32"),
        (Version::V1, _) => ("CID v1", "from base32"),
    };
    let version_map = (cid.encoded_len() - cid.hash().size() as usize) * 8;
    let entropy_map = cid.hash().size() as usize * 8;

    Some(IDInfo {
        id_type: "IPFS".to_string(),
        version: Some(version.to_string()),
        standard: args.id.to_string(),
        parsed: Some(parsed.to_string()),
        size: (cid.encoded_len() * 8) as u16,
        entropy: (cid.hash().size() as u16 * 8),
        hex: Some(hex::encode(cid.to_bytes())),
        bits: Some(cid.to_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(format!(
            "{}{}",
            (0..(version_map)).map(|_| "1").collect::<String>(),
            (0..(entropy_map)).map(|_| "2").collect::<String>()
        )),
        high_confidence: true,
        ..Default::default()
    })
}
