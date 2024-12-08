use crate::schema::{Args, IDInfo};
use sqids::Sqids;
use std::panic;

pub fn parse_sqid(args: &Args) -> Option<IDInfo> {
    let mut version: Option<String> = Some("Default alphabet".to_string());
    let sqids = match &args.alphabet {
        Some(alphabet) => match Sqids::builder().alphabet(alphabet.chars().collect()).build() {
            Ok(value) => {
                version = Some("Custom alphabet".to_string());
                value
            }
            Err(_) => {
                println!("Invalid alphabet for Sqids.");
                std::process::exit(1);
            }
        },
        None => Sqids::default(),
    };

    let _ = panic::catch_unwind(|| {
        panic::set_hook(Box::new(|_info| {
            println!("Invalid ID for this format.");
            std::process::exit(1);
        }));
        sqids.decode(&args.id); // Bug on sqids crate: this should not panic.
    });

    let numbers = sqids.decode(&args.id);
    if numbers.is_empty() {
        return None;
    }

    Some(IDInfo {
        id_type: "Sqid".to_string(),
        version,
        standard: args.id.clone(),
        integer: None,
        short_uuid: None,
        base64: None,
        uuid_wrap: None,
        size: 0,
        entropy: 0,
        datetime: None,
        timestamp: None,
        sequence: None,
        node1: Some(numbers.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")),
        node2: None,
        hex: None,
        bits: None,
        color_map: None,
    })
}
