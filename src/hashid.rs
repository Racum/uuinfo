use crate::schema::{Args, IDInfo};
use hash_ids::HashIds;
use std::panic;

pub fn parse_hashid(args: &Args) -> Option<IDInfo> {
    let mut version: Option<String> = Some("No salt".to_string());

    let mut builder = HashIds::builder();
    if let Some(salt) = &args.salt {
        builder = builder.with_salt(salt);
        version = Some("Custom salt".to_string());
    }
    let hashid_core = builder.finish();

    // Panic trap:
    let _ = panic::catch_unwind(|| {
        panic::set_hook(Box::new(|_info| {
            println!("Invalid ID for this format.");
            std::process::exit(1);
        }));
        hashid_core.decode(&args.id).ok() // Bug on hash-ids crate: this should not panic.
    });

    let numbers = match hashid_core.decode(&args.id) {
        Ok(value) => value,
        Err(_) => return None,
    };
    if numbers.is_empty() {
        return None;
    }

    Some(IDInfo {
        id_type: "Hashid".to_string(),
        version,
        standard: args.id.clone(),
        node1: Some(numbers.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")),
        ..Default::default()
    })
}
