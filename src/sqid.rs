use sqids::Sqids;

use crate::schema::{Args, IDInfo};

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

    let numbers = sqids.decode(&args.id);
    if numbers.is_empty() {
        return None;
    }

    Some(IDInfo {
        id_type: "Sqid".to_string(),
        version,
        standard: args.id.clone(),
        node1: Some(numbers.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")),
        ..Default::default()
    })
}
