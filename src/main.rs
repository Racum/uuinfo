use clap::Parser;

mod utils;

mod schema;
use crate::schema::{Args, ForceFormat, IDInfo};
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
use crate::timeflake::{parse_timeflake_any, parse_timeflake_base62};
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
mod nanoid;
use crate::nanoid::parse_nanoid;

pub fn auto_detect(args: &Args) -> Option<IDInfo> {
    let mut id_info: Option<IDInfo>;
    if args.id.trim().parse::<u64>().is_ok() {
        // Numeric:
        id_info = parse_snowflake(args)
    } else {
        // Fixed length:
        id_info = match args.id.chars().count() {
            40 => parse_ksuid(args),
            36 | 32 => parse_uuid(args),
            27 => match parse_upid(args) {
                Some(value) => Some(value),
                None => parse_ksuid(args),
            },
            26 => parse_ulid(args),
            25 => match parse_cuid1(args) {
                Some(value) => Some(value),
                None => parse_scru128(args),
            },
            24 => match parse_objectid(args) {
                Some(value) => Some(value),
                None => parse_base64_uuid(args),
            },
            22 => match parse_short_uuid(args) {
                Some(value) => Some(value),
                None => parse_timeflake_base62(args),
            },
            20 => parse_xid(args),
            18 => parse_flake(args),
            12 => parse_scru64(args),
            _ => None,
        };
        // Variable length:
        id_info = match id_info {
            Some(value) => Some(value),
            None => match parse_base64_uuid(args) {
                Some(value) => Some(value),
                None => match parse_cuid2(args) {
                    Some(value) => Some(value),
                    None => parse_nanoid(args),
                },
            },
        }
    }
    id_info
}

pub fn force_format(args: &Args) -> Option<IDInfo> {
    match &args.force? {
        ForceFormat::Uuid => parse_uuid(args),
        ForceFormat::Shortuuid => parse_short_uuid(args),
        ForceFormat::UuidB64 => parse_base64_uuid(args),
        ForceFormat::Uuid25 => None,
        ForceFormat::Ulid => parse_ulid(args),
        ForceFormat::Upid => parse_upid(args),
        ForceFormat::Timeflake => parse_timeflake_any(args),
        ForceFormat::Flake => parse_flake(args),
        ForceFormat::Scru128 => parse_scru128(args),
        ForceFormat::Scru64 => parse_scru64(args),
        ForceFormat::Mongodb => parse_objectid(args),
        ForceFormat::Ksuid => parse_ksuid(args),
        ForceFormat::Xid => parse_xid(args),
        ForceFormat::Cuid1 => parse_cuid1(args),
        ForceFormat::Cuid2 => parse_cuid2(args),
        ForceFormat::Nanoid => parse_nanoid(args),
        ForceFormat::SfTwitter => parse_snowflake(args),
        ForceFormat::SfMastodon => parse_snowflake(args),
        ForceFormat::SfDiscord => parse_snowflake(args),
        ForceFormat::SfInstagram => parse_snowflake(args),
        ForceFormat::SfLinkedin => parse_snowflake(args),
        ForceFormat::SfSony => parse_snowflake(args),
        ForceFormat::SfSpaceflake => parse_snowflake(args),
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_detect() {
        fn _assert(id: &str, id_type: &str, version: &str) {
            let args = Args {
                id: id.to_string(),
                output: None,
                force: None,
                compare_snowflake: false,
                b64_nopad: false,
                b64_bigendian: false,
                alphabet: None,
                hashid_salt: None,
            };
            assert_eq!(auto_detect(&args).unwrap().id_type, id_type.to_string(), "{id}");
            assert_eq!(auto_detect(&args).unwrap().version.unwrap_or("-".to_string()), version.to_string(), "{id}");
        }

        // UUID:
        _assert("16689a10-a518-11ef-aa74-4ec6089be97a", "UUID (RFC-4122)", "1 (timestamp and node)");
        _assert("215d3d9f-e980-2cf4-9191-7dd485ba4fee", "UUID (RFC-4122)", "2 (DCE security)");
        _assert("6fc22fc2-8e36-3ab9-888f-d6fbd3af370a", "UUID (RFC-4122)", "3 (MD5 hash)");
        _assert("8584c629-371f-4fc6-a1af-ae5201e8f210", "UUID (RFC-4122)", "4 (random)");
        _assert("26ad69ad-4c50-5737-bb66-bd46328ecf8a", "UUID (RFC-4122)", "5 (SHA-1 hash)");
        _assert("1efa519c-2b25-6fd0-8fa1-610b58ceebbe", "UUID (RFC-9562)", "6 (sortable timestamp and node)");
        _assert("01933b8c-7875-7b8e-b5fa-bb500eb8bb38", "UUID (RFC-9562)", "7 (sortable timestamp and random)");
        _assert("4a0b86fe-afe0-86b9-8b1e-3375b2be3580", "UUID (RFC-9562)", "8 (custom)");
        _assert("00000000-0000-0000-0000-000000000000", "Nil UUID (all zeros)", "-");
        _assert("ffffffff-ffff-ffff-ffff-ffffffffffff", "Max UUID (all ones)", "-");
        _assert("906b4e7f-84a3-a0ed-1191-2dea8b497113", "NCS UUID", "-");
        _assert("4e3b9f31-c9ae-a2ff-D191-1e180a66f92f", "Microsoft UUID", "-");
        // UUID wrappers:
        _assert("32CQvwbvpbnkmkhhguznVH", "Short-UUID of UUID (RFC-4122)", "4 (random)");
        _assert("UHKjBazX_UG8dEAJaikK1g==", "Padded Base64 of UUID (RFC-4122)", "4 (random)");
        _assert("UHKjBazX_UG8dEAJaikK1g", "Unpadded Base64 of UUID (RFC-4122)", "4 (random)");
        // Snowflakes:
        _assert("1400000000000000000", "Snowflake", "Unknown (use -f to specify version)");
        // Other:
        _assert("01JCXSGZMZQQJ2M93WC0T8KT02", "ULID", "-");
        _assert("abcd_2adnrb7b6jkyos6xusvmaa", "UPID", "A (default)");
        _assert("6592008029c8c3e4dc76256c", "MongoDB ObjectId", "-");
        _assert("1HCpXwx2EK9oYluWbacgeCnFcLf", "KSUID", "Base62-encoded");
        _assert("13c7f72eff938c3cba49cfb88fa840868effca9c", "KSUID", "Hex-encoded");
        _assert("cst4p962941gd9baqg70", "Xid", "-");
        _assert("cm3xemk9o00070cm7ghnl6toe", "CUID", "1");
        _assert("byab6ewccgwheoshq1wk9hds", "CUID", "2");
        _assert("03cwivkme1qhj3crprqujv4lu", "SCRU128", "-");
        _assert("0v20wcjrb21p", "SCRU64", "-");
        _assert("02i2XhN7hAuaFh3MwztcMd", "Timeflake", "-");
        _assert("8HFaR8qWtRlGDHnO57", "Flake (Boundary)", "-");
        _assert("XBCdxzsCR2FEFeSwhnjCo", "Nano ID", "Default alphabet and length");
        _assert("XBCdxzsCR2FEFeSwhnjCo000", "Nano ID", "Default alphabet, custom length (24)");
    }

    #[test]
    fn test_force_format() {
        fn _assert(id: &str, force: ForceFormat, id_type: &str, version: &str) {
            let args = Args {
                id: id.to_string(),
                output: None,
                force: Some(force),
                compare_snowflake: false,
                b64_nopad: false,
                b64_bigendian: false,
                alphabet: None,
                hashid_salt: None,
            };
            assert_eq!(force_format(&args).unwrap().id_type, id_type.to_string(), "{id}");
            assert_eq!(force_format(&args).unwrap().version.unwrap_or("-".to_string()), version.to_string(), "{id}");
        }

        // UUID:
        _assert("16689a10-a518-11ef-aa74-4ec6089be97a", ForceFormat::Uuid, "UUID (RFC-4122)", "1 (timestamp and node)");
        _assert("215d3d9f-e980-2cf4-9191-7dd485ba4fee", ForceFormat::Uuid, "UUID (RFC-4122)", "2 (DCE security)");
        _assert("6fc22fc2-8e36-3ab9-888f-d6fbd3af370a", ForceFormat::Uuid, "UUID (RFC-4122)", "3 (MD5 hash)");
        _assert("8584c629-371f-4fc6-a1af-ae5201e8f210", ForceFormat::Uuid, "UUID (RFC-4122)", "4 (random)");
        _assert("26ad69ad-4c50-5737-bb66-bd46328ecf8a", ForceFormat::Uuid, "UUID (RFC-4122)", "5 (SHA-1 hash)");
        _assert("1efa519c-2b25-6fd0-8fa1-610b58ceebbe", ForceFormat::Uuid, "UUID (RFC-9562)", "6 (sortable timestamp and node)");
        _assert("01933b8c-7875-7b8e-b5fa-bb500eb8bb38", ForceFormat::Uuid, "UUID (RFC-9562)", "7 (sortable timestamp and random)");
        _assert("4a0b86fe-afe0-86b9-8b1e-3375b2be3580", ForceFormat::Uuid, "UUID (RFC-9562)", "8 (custom)");
        _assert("00000000-0000-0000-0000-000000000000", ForceFormat::Uuid, "Nil UUID (all zeros)", "-");
        _assert("ffffffff-ffff-ffff-ffff-ffffffffffff", ForceFormat::Uuid, "Max UUID (all ones)", "-");
        _assert("906b4e7f-84a3-a0ed-1191-2dea8b497113", ForceFormat::Uuid, "NCS UUID", "-");
        _assert("4e3b9f31-c9ae-a2ff-D191-1e180a66f92f", ForceFormat::Uuid, "Microsoft UUID", "-");
        // Other formats wrapped in UUID:
        _assert("01933b98-7e9f-bde4-2a24-7c603489e802", ForceFormat::Ulid, "ULID wrapped in UUID", "-");
        _assert("01933b9c-a723-e1ea-609d-d6372631d096", ForceFormat::Upid, "UPID wrapped in UUID", "A (default)");
        _assert("00000134-d423-5a10-109a-dd5e0e8f0005", ForceFormat::Flake, "Flake (Boundary) wrapped in UUID", "-");
        _assert("016fb420-9023-b444-fd07-590f81b7b0eb", ForceFormat::Timeflake, "Timeflake wrapped in UUID", "-");
        _assert("01936600-0a18-12a4-811f-c295c399b412", ForceFormat::Scru128, "SCRU128 wrapped in UUID", "-");
        // UUID wrappers:
        _assert("32CQvwbvpbnkmkhhguznVH", ForceFormat::Shortuuid, "Short-UUID of UUID (RFC-4122)", "4 (random)");
        _assert("UHKjBazX_UG8dEAJaikK1g==", ForceFormat::UuidB64, "Padded Base64 of UUID (RFC-4122)", "4 (random)");
        _assert("UHKjBazX_UG8dEAJaikK1g", ForceFormat::UuidB64, "Unpadded Base64 of UUID (RFC-4122)", "4 (random)");
        // Snowflakes:
        _assert("1777150623882019211", ForceFormat::SfTwitter, "Snowflake", "Twitter");
        _assert("1304369705066434662", ForceFormat::SfDiscord, "Snowflake", "Discord");
        _assert("1671390786412876801", ForceFormat::SfInstagram, "Snowflake", "Instagram");
        _assert("540226260526170119", ForceFormat::SfSony, "Snowflake", "Sony");
        _assert("1015189130756840860", ForceFormat::SfSpaceflake, "Snowflake", "Spaceflake");
        _assert("112277929257317646", ForceFormat::SfMastodon, "Snowflake", "Mastodon");
        _assert("7256902784527069184", ForceFormat::SfLinkedin, "Snowflake", "LinkedIn");
        _assert("XBCdxzsCR2FEFeSwhnjCo", ForceFormat::Nanoid, "Nano ID", "Default alphabet and length");
        _assert("XBCdxzsCR2FEFeSwhnjCo000", ForceFormat::Nanoid, "Nano ID", "Default alphabet, custom length (24)");
        // Other:
        _assert("01JCXSGZMZQQJ2M93WC0T8KT02", ForceFormat::Ulid, "ULID", "-");
        _assert("abcd_2adnrb7b6jkyos6xusvmaa", ForceFormat::Upid, "UPID", "A (default)");
        _assert("6592008029c8c3e4dc76256c", ForceFormat::Mongodb, "MongoDB ObjectId", "-");
        _assert("1HCpXwx2EK9oYluWbacgeCnFcLf", ForceFormat::Ksuid, "KSUID", "Base62-encoded");
        _assert("13c7f72eff938c3cba49cfb88fa840868effca9c", ForceFormat::Ksuid, "KSUID", "Hex-encoded");
        _assert("cst4p962941gd9baqg70", ForceFormat::Xid, "Xid", "-");
        _assert("cm3xemk9o00070cm7ghnl6toe", ForceFormat::Cuid1, "CUID", "1");
        _assert("byab6ewccgwheoshq1wk9hds", ForceFormat::Cuid2, "CUID", "2");
        _assert("03cwivkme1qhj3crprqujv4lu", ForceFormat::Scru128, "SCRU128", "-");
        _assert("0v20wcjrb21p", ForceFormat::Scru64, "SCRU64", "-");
        _assert("02i2XhN7hAuaFh3MwztcMd", ForceFormat::Timeflake, "Timeflake", "-");
        _assert("8HFaR8qWtRlGDHnO57", ForceFormat::Flake, "Flake (Boundary)", "-");
    }
}
