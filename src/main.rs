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
mod objectid;
use crate::objectid::parse_objectid;
mod flake;
use crate::flake::parse_flake;
mod cuid;
use crate::cuid::{parse_cuid1, parse_cuid2};
mod scru;
use crate::scru::{parse_scru128, parse_scru64};

fn main() {
    let args = Args::parse();

    if args.compare_snowflake {
        compare_snowflake(&args)
    }

    let mut id_info: Option<IDInfo>;
    if args.id.trim().parse::<u64>().is_ok() {
        // Numeric:
        id_info = parse_snowflake(&args)
    } else {
        // Fixed length:
        id_info = match &args.id.chars().count() {
            40 => parse_ksuid(&args),
            36 | 32 => parse_uuid(&args),
            27 => match parse_upid(&args) {
                Some(value) => Some(value),
                None => parse_ksuid(&args),
            },
            26 => parse_ulid(&args),
            25 => match parse_cuid1(&args) {
                Some(value) => Some(value),
                None => parse_scru128(&args),
            },

            24 => match parse_objectid(&args) {
                Some(value) => Some(value),
                None => parse_base64_uuid(&args),
            },
            22 => match parse_short_uuid(&args) {
                Some(value) => Some(value),
                None => parse_timeflake_base62(&args),
            },
            20 => parse_xid(&args),
            18 => parse_flake(&args),
            12 => parse_scru64(&args),
            _ => None,
        };
        // Variable length:
        id_info = match id_info {
            Some(value) => Some(value),
            None => match parse_base64_uuid(&args) {
                Some(value) => Some(value),
                None => parse_cuid2(&args),
            },
        };
    }

    match id_info {
        Some(value) => value.print(),
        None => {
            println!("Something else");
        }
    }
}
