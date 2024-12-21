use crate::cuid::{parse_cuid1, parse_cuid2};
use crate::flake::parse_flake;
use crate::hashid::parse_hashid;
use crate::ksuid::parse_ksuid;
use crate::nanoid::parse_nanoid;
use crate::objectid::parse_objectid;
use crate::schema::{Args, IDInfo, IdFormat};
use crate::scru::{parse_scru128, parse_scru64};
use crate::snowflake::parse_snowflake;
use crate::sqid::parse_sqid;
use crate::stripe::parse_stripe;
use crate::timeflake::{parse_timeflake_any, parse_timeflake_base62};
use crate::tsid::parse_tsid;
use crate::ulid::parse_ulid;
use crate::upid::parse_upid;
use crate::uuid::{parse_base64_uuid, parse_short_uuid, parse_uuid, parse_uuid25};
use crate::xid::parse_xid;
use crate::youtube::parse_youtube;

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
                None => match parse_timeflake_base62(args) {
                    Some(value) => Some(value),
                    None => parse_base64_uuid(args),
                },
            },
            21 => parse_nanoid(args),
            20 => parse_xid(args),
            18 => parse_flake(args),
            13 => parse_tsid(args),
            12 => parse_scru64(args),
            11 => parse_youtube(args),
            _ => None,
        };
        // Variable length:
        id_info = match id_info {
            Some(value) => Some(value),
            None => match parse_stripe(args) {
                Some(value) => Some(value),
                None => match parse_cuid2(args) {
                    Some(value) => Some(value),
                    None => match parse_sqid(args) {
                        Some(value) => Some(value),
                        None => match parse_nanoid(args) {
                            Some(value) => Some(value),
                            None => parse_hashid(args),
                        },
                    },
                },
            },
        }

        // }
    }
    id_info
}

pub fn force_format(args: &Args) -> Option<IDInfo> {
    match &args.force? {
        IdFormat::Uuid => parse_uuid(args),
        IdFormat::Shortuuid => parse_short_uuid(args),
        IdFormat::UuidB64 => parse_base64_uuid(args),
        IdFormat::Uuid25 => parse_uuid25(args),
        IdFormat::Ulid => parse_ulid(args),
        IdFormat::Upid => parse_upid(args),
        IdFormat::Timeflake => parse_timeflake_any(args),
        IdFormat::Flake => parse_flake(args),
        IdFormat::Scru128 => parse_scru128(args),
        IdFormat::Scru64 => parse_scru64(args),
        IdFormat::Mongodb => parse_objectid(args),
        IdFormat::Ksuid => parse_ksuid(args),
        IdFormat::Xid => parse_xid(args),
        IdFormat::Cuid1 => parse_cuid1(args),
        IdFormat::Cuid2 => parse_cuid2(args),
        IdFormat::Nanoid => parse_nanoid(args),
        IdFormat::SfTwitter => parse_snowflake(args),
        IdFormat::SfMastodon => parse_snowflake(args),
        IdFormat::SfDiscord => parse_snowflake(args),
        IdFormat::SfInstagram => parse_snowflake(args),
        IdFormat::SfLinkedin => parse_snowflake(args),
        IdFormat::SfSony => parse_snowflake(args),
        IdFormat::SfSpaceflake => parse_snowflake(args),
        IdFormat::SfFrostflake => parse_snowflake(args),
        IdFormat::Tsid => parse_tsid(args),
        IdFormat::Sqid => parse_sqid(args),
        IdFormat::Hashid => parse_hashid(args),
        IdFormat::Youtube => parse_youtube(args),
        IdFormat::Stripe => parse_stripe(args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Output;

    #[test]
    fn test_auto_detect() {
        fn _assert(id: &str, id_type: &str, version: &str) {
            let args = Args {
                id: id.to_string(),
                output: Output::Card,
                compare_snowflake: false,
                ..Default::default()
            };
            assert_eq!(auto_detect(&args).unwrap().id_type, id_type.to_string(), "{id} - {id_type} - {version}");
            assert_eq!(auto_detect(&args).unwrap().version.unwrap_or("-".to_string()), version.to_string(), "{id} - {id_type} - {version}");
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
        _assert("4e3b9f31-c9ae-a2ff-D191-1e180a66f92f", "Microsoft GUID", "-");
        // UUID wrappers:
        _assert("32CQvwbvpbnkmkhhguznVH", "ShortUUID of UUID (RFC-4122)", "4 (random)");
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
        _assert("XBCdxzsCR2FEFeSwhnjCo", "Nano ID", "Default alphabet, default length");
        _assert("0J4AEXRN106Z0", "TSID", "-");
        _assert("86Rf07xd4z", "Sqid", "Default alphabet");
        _assert("gocwRvLhDf8", "YouTube Video ID", "-");
        _assert("cus_lO1DEQWBbQAACfHO", "Stripe ID", "Customer ID");
    }

    #[test]
    fn test_force_format() {
        fn _assert(id: &str, force: IdFormat, id_type: &str, version: &str) {
            let args = Args {
                id: id.to_string(),
                output: Output::Card,
                force: Some(force),
                ..Default::default()
            };
            assert_eq!(force_format(&args).unwrap().id_type, id_type.to_string(), "{id} - {id_type} - {version}");
            assert_eq!(force_format(&args).unwrap().version.unwrap_or("-".to_string()), version.to_string(), "{id} - {id_type} - {version}");
        }

        // UUID:
        _assert("16689a10-a518-11ef-aa74-4ec6089be97a", IdFormat::Uuid, "UUID (RFC-4122)", "1 (timestamp and node)");
        _assert("215d3d9f-e980-2cf4-9191-7dd485ba4fee", IdFormat::Uuid, "UUID (RFC-4122)", "2 (DCE security)");
        _assert("6fc22fc2-8e36-3ab9-888f-d6fbd3af370a", IdFormat::Uuid, "UUID (RFC-4122)", "3 (MD5 hash)");
        _assert("8584c629-371f-4fc6-a1af-ae5201e8f210", IdFormat::Uuid, "UUID (RFC-4122)", "4 (random)");
        _assert("26ad69ad-4c50-5737-bb66-bd46328ecf8a", IdFormat::Uuid, "UUID (RFC-4122)", "5 (SHA-1 hash)");
        _assert("1efa519c-2b25-6fd0-8fa1-610b58ceebbe", IdFormat::Uuid, "UUID (RFC-9562)", "6 (sortable timestamp and node)");
        _assert("01933b8c-7875-7b8e-b5fa-bb500eb8bb38", IdFormat::Uuid, "UUID (RFC-9562)", "7 (sortable timestamp and random)");
        _assert("4a0b86fe-afe0-86b9-8b1e-3375b2be3580", IdFormat::Uuid, "UUID (RFC-9562)", "8 (custom)");
        _assert("00000000-0000-0000-0000-000000000000", IdFormat::Uuid, "Nil UUID (all zeros)", "-");
        _assert("ffffffff-ffff-ffff-ffff-ffffffffffff", IdFormat::Uuid, "Max UUID (all ones)", "-");
        _assert("906b4e7f-84a3-a0ed-1191-2dea8b497113", IdFormat::Uuid, "NCS UUID", "-");
        _assert("4e3b9f31-c9ae-a2ff-D191-1e180a66f92f", IdFormat::Uuid, "Microsoft GUID", "-");
        // Other formats wrapped in UUID:
        _assert("01933b98-7e9f-bde4-2a24-7c603489e802", IdFormat::Ulid, "ULID wrapped in UUID", "-");
        _assert("01933b9c-a723-e1ea-609d-d6372631d096", IdFormat::Upid, "UPID wrapped in UUID", "A (default)");
        _assert("00000134-d423-5a10-109a-dd5e0e8f0005", IdFormat::Flake, "Flake (Boundary) wrapped in UUID", "-");
        _assert("016fb420-9023-b444-fd07-590f81b7b0eb", IdFormat::Timeflake, "Timeflake wrapped in UUID", "-");
        _assert("01936600-0a18-12a4-811f-c295c399b412", IdFormat::Scru128, "SCRU128 wrapped in UUID", "-");
        // UUID wrappers:
        _assert("32CQvwbvpbnkmkhhguznVH", IdFormat::Shortuuid, "ShortUUID of UUID (RFC-4122)", "4 (random)");
        _assert("UHKjBazX_UG8dEAJaikK1g==", IdFormat::UuidB64, "Padded Base64 of UUID (RFC-4122)", "4 (random)");
        _assert("UHKjBazX_UG8dEAJaikK1g", IdFormat::UuidB64, "Unpadded Base64 of UUID (RFC-4122)", "4 (random)");
        _assert("dpoadk8izg9y4tte7vy1xt94o", IdFormat::Uuid25, "Uuid25 of UUID (RFC-4122)", "4 (random)");
        // Snowflakes:
        _assert("1777150623882019211", IdFormat::SfTwitter, "Snowflake", "Twitter");
        _assert("1304369705066434662", IdFormat::SfDiscord, "Snowflake", "Discord");
        _assert("1671390786412876801", IdFormat::SfInstagram, "Snowflake", "Instagram");
        _assert("540226260526170119", IdFormat::SfSony, "Snowflake", "Sony");
        _assert("1015189130756840860", IdFormat::SfSpaceflake, "Snowflake", "Spaceflake");
        _assert("112277929257317646", IdFormat::SfMastodon, "Snowflake", "Mastodon");
        _assert("7256902784527069184", IdFormat::SfLinkedin, "Snowflake", "LinkedIn");
        _assert("7423342004626526207", IdFormat::SfFrostflake, "Snowflake", "Frostflake");
        _assert("JERHwh5PXjL", IdFormat::SfFrostflake, "Snowflake", "Frostflake");
        // Other:
        _assert("01JCXSGZMZQQJ2M93WC0T8KT02", IdFormat::Ulid, "ULID", "-");
        _assert("abcd_2adnrb7b6jkyos6xusvmaa", IdFormat::Upid, "UPID", "A (default)");
        _assert("6592008029c8c3e4dc76256c", IdFormat::Mongodb, "MongoDB ObjectId", "-");
        _assert("1HCpXwx2EK9oYluWbacgeCnFcLf", IdFormat::Ksuid, "KSUID", "Base62-encoded");
        _assert("13c7f72eff938c3cba49cfb88fa840868effca9c", IdFormat::Ksuid, "KSUID", "Hex-encoded");
        _assert("cst4p962941gd9baqg70", IdFormat::Xid, "Xid", "-");
        _assert("cm3xemk9o00070cm7ghnl6toe", IdFormat::Cuid1, "CUID", "1");
        _assert("byab6ewccgwheoshq1wk9hds", IdFormat::Cuid2, "CUID", "2");
        _assert("03cwivkme1qhj3crprqujv4lu", IdFormat::Scru128, "SCRU128", "-");
        _assert("0v20wcjrb21p", IdFormat::Scru64, "SCRU64", "-");
        _assert("02i2XhN7hAuaFh3MwztcMd", IdFormat::Timeflake, "Timeflake", "-");
        _assert("8HFaR8qWtRlGDHnO57", IdFormat::Flake, "Flake (Boundary)", "-");
        _assert("XBCdxzsCR2FEFeSwhnjCo", IdFormat::Nanoid, "Nano ID", "Default alphabet, default length");
        _assert("h9jYw2bcOe", IdFormat::Nanoid, "Nano ID", "Default alphabet, custom length (10)");
        _assert("0J4AEXRN106Z0", IdFormat::Tsid, "TSID", "-");
        _assert("653390205760314336", IdFormat::Tsid, "TSID", "-");
        _assert("HamVxsto6jDM", IdFormat::Sqid, "Sqid", "Default alphabet");
        _assert("80JTEquWr", IdFormat::Hashid, "Hashid", "No salt");
        _assert("gocwRvLhDf8", IdFormat::Youtube, "YouTube Video ID", "-");
        _assert("cus_lO1DEQWBbQAACfHO", IdFormat::Stripe, "Stripe ID", "Customer ID");
    }
}
