use clap::Parser;
use std::io;

mod cuid;
mod flake;
mod hashid;
mod ksuid;
mod nanoid;
mod objectid;
mod scru;
mod sqid;
mod stripe;
mod timeflake;
mod tsid;
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
use crate::formats::{auto_detect, force_format};

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
