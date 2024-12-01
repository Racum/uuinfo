use clap::Parser;

mod cuid;
mod flake;
mod ksuid;
mod nanoid;
mod objectid;
mod scru;
mod timeflake;
mod ulid;
mod upid;
mod utils;
mod uuid;
mod xid;

mod schema;
use crate::schema::Args;
mod snowflake;
use crate::snowflake::compare_snowflake;
mod formats;
use crate::formats::{auto_detect, force_format};

fn main() {
    let args = Args::parse();

    if args.compare_snowflake {
        compare_snowflake(&args)
    }

    match &args.force {
        Some(_) => match force_format(&args) {
            Some(value) => value.print(&args),
            None => {
                println!("Invalid ID for this format");
                std::process::exit(1);
            }
        },
        None => match auto_detect(&args) {
            Some(value) => value.print(&args),
            None => {
                println!("Unknown ID type");
                std::process::exit(1);
            }
        },
    };
}
