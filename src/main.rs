use clap::Parser;
use std::io;

mod cuid;
mod datadog;
mod flake;
mod hash;
mod hashid;
mod ipfs;
mod ksuid;
mod nanoid;
mod nuid;
mod objectid;
mod scru;
mod sqid;
mod stripe;
mod timeflake;
mod tsid;
mod typeid;
mod ulid;
mod unix;
mod upid;
mod utils;
mod uuid;
mod xid;
mod youtube;

mod schema;
use crate::schema::Args;
mod snowflake;
use crate::snowflake::compare_snowflake;
mod formats;
use crate::formats::{auto_detect, force_format, parse_all};

fn main() {
    let mut args = Args::parse();

    if args.compare_snowflake {
        compare_snowflake(&args)
    }

    if &args.id == "-" {
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_ok() {
            if let Some(value) = buffer.split('\n').next() {
                args.id = value.to_string();
            }
        }
    }

    if args.everything {
        let valid_ids = parse_all(&args);
        if !valid_ids.is_empty() {
            for value in parse_all(&args) {
                value.print(&args);
            }
        } else {
            println!("Unknown ID type.");
        }
    } else {
        match &args.force {
            Some(_) => match force_format(&args) {
                Some(value) => value.print(&args),
                None => {
                    println!("Invalid ID for this format.");
                    std::process::exit(1);
                }
            },
            None => match auto_detect(&args) {
                Some(value) => value.print(&args),
                None => {
                    println!("Unknown ID type.");
                    std::process::exit(1);
                }
            },
        };
    }
}
