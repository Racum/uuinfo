use hash_ids::HashIds;
use std::panic;

use crate::schema::{Args, IDInfo};
use crate::utils::factor_size_hex_bits_color_from_text;

pub fn parse_hashid(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() > 43 {
        return None;
    }
    let mut version: Option<String> = Some("No salt".to_string());

    let mut builder = HashIds::builder();
    if let Some(salt) = &args.salt {
        builder = builder.with_salt(salt);
        version = Some("Custom salt".to_string());
    }
    let hashid_core = builder.finish();

    // Panic trap:
    match panic::catch_unwind(|| {
        panic::set_hook(Box::new(|_info| {}));
        hashid_core.decode(&args.id).ok() // Bug on hash-ids crate: this should not panic.
    }) {
        Ok(_) => (),
        Err(_) => return None,
    }

    let numbers = hashid_core.decode(&args.id).ok()?;
    if numbers.is_empty() {
        return None;
    }
    let (size, hex, bits, _) = factor_size_hex_bits_color_from_text(&args.id);

    Some(IDInfo {
        id_type: "Hashid".to_string(),
        version,
        standard: args.id.clone(),
        parsed: Some("as ASCII".to_string()),
        size,
        node1: Some(numbers.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")),
        hex,
        bits,
        color_map: Some((0..size).map(|_| "4").collect::<String>()),
        ..Default::default()
    })
}
