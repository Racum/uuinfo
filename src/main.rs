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
use crate::schema::{Args, IDInfo};
mod snowflake;
use crate::snowflake::compare_snowflake;
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
