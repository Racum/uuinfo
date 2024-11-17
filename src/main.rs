use clap::Parser;

mod utils;

mod schema;
use crate::schema::{Args, IDInfo};
mod uuid;
use crate::uuid::{parse_base64_uuid, parse_short_uuid, parse_uuid};
mod ulid;
use crate::ulid::parse_ulid;
mod upid;
use crate::upid::parse_upid;
mod ksuid;
use crate::ksuid::parse_ksuid;
mod snowflake;
use crate::snowflake::{compare_snowflake, parse_snowflake};
mod timeflake;
use crate::timeflake::parse_timeflake_base62;
mod xid;
use crate::xid::parse_xid;

fn main() {
    let args = Args::parse();

    if args.compare_snowflake {
        compare_snowflake(&args)
    }

    let id_info: IDInfo;
    match &args.id.chars().count() {
        40 => {
            id_info = parse_ksuid(&args).unwrap_or_default();
            id_info.print()
        }
        36 | 32 => {
            id_info = parse_uuid(&args).unwrap_or_default();
            id_info.print()
        }
        27 => {
            match parse_upid(&args) {
                Some(value) => {
                    id_info = value;
                }
                None => {
                    id_info = parse_ksuid(&args).unwrap_or_default();
                }
            }
            id_info.print()
        }
        26 => {
            id_info = parse_ulid(&args).unwrap_or_default();
            id_info.print()
        }
        24 => {
            // println!("MongoDB");
            id_info = parse_base64_uuid(&args).unwrap_or_default();
            id_info.print();
        }
        21..24 => {
            id_info = parse_short_uuid(&args).unwrap_or(
                parse_base64_uuid(&args)
                    .unwrap_or(parse_timeflake_base62(&args).unwrap_or_default()),
            );
            id_info.print()
        }
        1..21 => {
            // println!("MAC")

            id_info = parse_xid(&args).unwrap_or(parse_snowflake(&args).unwrap_or_default());
            id_info.print()
        }
        _ => {
            println!("Something else")
        }
    }
}
