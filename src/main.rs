use clap::Parser;

mod utils;
mod uuid;
mod ulid;
mod upid;
mod ksuid;
mod timeflake;
mod xid;
mod objectid;
mod flake;
mod cuid;
mod scru;
mod nanoid;

mod schema;
use crate::schema::{Args, IDInfo};
mod snowflake;
use crate::snowflake::{compare_snowflake};
mod formats;
use crate::formats::{auto_detect, force_format};

fn main() {
    let args = Args::parse();

    if args.compare_snowflake {
        compare_snowflake(&args)
    }

    let id_info: Option<IDInfo> = match &args.force {
        Some(_) => force_format(&args),
        None => auto_detect(&args),
    };

    match id_info {
        Some(value) => value.print(),
        None => {
            println!("Something else");
        }
    }
}
