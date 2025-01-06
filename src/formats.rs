use crate::breezeid::parse_breezeid;
use crate::cuid::{parse_cuid1, parse_cuid2};
use crate::datadog::parse_datadog;
use crate::flake::parse_flake;
use crate::hash::parse_hash;
use crate::hashid::parse_hashid;
use crate::ipfs::parse_ipfs;
use crate::ksuid::parse_ksuid;
use crate::nanoid::parse_nanoid;
use crate::nuid::parse_nuid;
use crate::objectid::parse_objectid;
use crate::puid::{parse_puid, parse_puid_any, parse_shortpuid};
use crate::pushid::parse_pushid;
use crate::schema::{Args, IDInfo, IdFormat};
use crate::scru::{parse_scru128, parse_scru64};
use crate::snowflake::parse_snowflake;
use crate::sqid::parse_sqid;
use crate::stripe::parse_stripe;
use crate::timeflake::{parse_timeflake_any, parse_timeflake_base62};
use crate::tsid::parse_tsid;
use crate::typeid::parse_typeid;
use crate::ulid::parse_ulid;
use crate::unix::{parse_unix, parse_unix_ms, parse_unix_ns, parse_unix_recent, parse_unix_s, parse_unix_us};
use crate::upid::parse_upid;
use crate::uuid::{parse_base64_uuid, parse_short_uuid, parse_uuid, parse_uuid25, parse_uuid_integer};
use crate::xid::parse_xid;
use crate::youtube::parse_youtube;

type ParseFunction = fn(&Args) -> Option<IDInfo>;

#[rustfmt::skip]
#[allow(dead_code)]
const ALL_PARSERS: [ParseFunction; 33] = [
    parse_uuid,
    parse_base64_uuid,
    parse_uuid25,
    parse_short_uuid,
    parse_uuid_integer,
    parse_ulid,
    parse_upid,
    parse_objectid,
    parse_ksuid,
    parse_xid,
    parse_cuid1,
    parse_cuid2,
    parse_scru128,
    parse_scru64,
    parse_timeflake_any,
    parse_flake,
    parse_tsid,
    parse_nuid,
    parse_typeid,
    parse_pushid,
    parse_sqid,
    parse_hashid,
    parse_youtube,
    parse_stripe,
    parse_datadog,
    parse_snowflake,
    parse_unix,
    parse_hash,
    parse_ipfs,
    parse_breezeid,
    parse_puid,
    parse_shortpuid,
    parse_nanoid,
];

pub fn parse_all(args: &Args) -> Vec<IDInfo> {
    let mut valid_ids: Vec<IDInfo> = vec![];
    for parser in ALL_PARSERS {
        if let Some(value) = parser(args) {
            valid_ids.push(value);
        }
    }
    valid_ids
}

pub fn pick_first_valid(args: &Args, parsers: Vec<ParseFunction>) -> Option<IDInfo> {
    for parser in parsers {
        if let Some(value) = parser(args) {
            return Some(value);
        }
    }
    None
}

pub fn auto_detect(args: &Args) -> Option<IDInfo> {
    let mut id_info: Option<IDInfo>;
    if args.id.trim().parse::<u128>().is_ok() {
        // Numeric:
        id_info = pick_first_valid(args, vec![parse_unix_recent, parse_snowflake, parse_uuid_integer]);
    } else {
        // Fixed length:
        id_info = match args.id.chars().count() {
            56 | 64 | 96 | 128 => parse_hash(args),
            40 => parse_ksuid(args),
            36 => parse_uuid(args),
            32 => pick_first_valid(args, vec![parse_datadog, parse_uuid]),
            27 => pick_first_valid(args, vec![parse_upid, parse_ksuid]),
            26 => parse_ulid(args),
            25 => pick_first_valid(args, vec![parse_cuid1, parse_scru128]),
            24 => pick_first_valid(args, vec![parse_objectid, parse_puid, parse_base64_uuid]),
            22 => pick_first_valid(args, vec![parse_short_uuid, parse_timeflake_base62, parse_base64_uuid, parse_nuid]),
            21 => parse_nanoid(args),
            20 => pick_first_valid(args, vec![parse_xid, parse_stripe, parse_pushid]),
            18 => parse_flake(args),
            14 => parse_shortpuid(args),
            13 => parse_tsid(args),
            12 => pick_first_valid(args, vec![parse_scru64, parse_shortpuid]),
            11 => parse_youtube(args),
            _ => None,
        };
        // Variable length:
        id_info = match id_info {
            Some(value) => Some(value),
            #[rustfmt::skip]
            None => pick_first_valid(args, vec![
                parse_typeid,
                parse_ipfs,
                parse_stripe,
                parse_breezeid,
                parse_cuid2,
                parse_sqid,
                parse_nanoid,
                parse_hashid,
            ]),
        };
    }
    id_info
}

pub fn force_format(args: &Args) -> Option<IDInfo> {
    match &args.force? {
        IdFormat::Uuid => parse_uuid(args),
        IdFormat::Shortuuid => parse_short_uuid(args),
        IdFormat::UuidB64 => parse_base64_uuid(args),
        IdFormat::Uuid25 => parse_uuid25(args),
        IdFormat::UuidInt => parse_uuid_integer(args),
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
        IdFormat::SfFlakeid => parse_snowflake(args),
        IdFormat::Tsid => parse_tsid(args),
        IdFormat::Sqid => parse_sqid(args),
        IdFormat::Hashid => parse_hashid(args),
        IdFormat::Youtube => parse_youtube(args),
        IdFormat::Stripe => parse_stripe(args),
        IdFormat::Unix => parse_unix(args),
        IdFormat::UnixS => parse_unix_s(args),
        IdFormat::UnixMs => parse_unix_ms(args),
        IdFormat::UnixUs => parse_unix_us(args),
        IdFormat::UnixNs => parse_unix_ns(args),
        IdFormat::Datadog => parse_datadog(args),
        IdFormat::Hash => parse_hash(args),
        IdFormat::Ipfs => parse_ipfs(args),
        IdFormat::Nuid => parse_nuid(args),
        IdFormat::Typeid => parse_typeid(args),
        IdFormat::Breezeid => parse_breezeid(args),
        IdFormat::Puid => parse_puid_any(args),
        IdFormat::Pushid => parse_pushid(args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_detect() {
        fn _assert(id: &str, id_type: &str, version: &str) {
            let id_info = auto_detect(&Args {
                id: id.to_string(),
                ..Default::default()
            })
            .unwrap();
            assert_eq!(id_info.id_type, id_type.to_string(), "{id} - {id_type} - {version}");
            assert_eq!(id_info.version.unwrap_or("-".to_string()), version.to_string(), "{id} - {id_type} - {version}");
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
        _assert("2093703425379131962944436515747969848", "Integer of UUID (RFC-9562)", "7 (sortable timestamp and random)");
        // Snowflakes:
        _assert("1400000000000000000", "Snowflake", "Unknown (use -f to specify version)");
        // Unix timestamp
        _assert("1734971723", "Unix timestamp", "Assuming seconds");
        _assert("1734971723000", "Unix timestamp", "Assuming milliseconds");
        _assert("1734971723000000", "Unix timestamp", "Assuming microseconds");
        _assert("1734971723000000000", "Unix timestamp", "Assuming nanoseconds");
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
        _assert("6772800700000000d97a8af26532e259", "Datadog Trace ID", "-");
        _assert("EQyuCsA4ysv7ezXReOrk4i", "NUID", "-");
        _assert("prefix_01h2xcejqtf2nbrexx3vqjhp41", "TypeID", "-");
        _assert("9NU6-XQLZ-BDIH-6HKE", "Breeze ID", "Default alphabet");
        _assert("9nu6-xqlz-bdih-6hke", "Breeze ID", "Lowercase alphabet");
        _assert("he5fps6l2504cd1w3ag8ut8e", "Puid", "-");
        _assert("aeby6ob5sso4zd", "Puid", "Short puid with node ID");
        _assert("-OFrJ24CPTXLcIPPjvh3", "PushID (Firebase)", "-");
        // Hash-based:
        _assert("b265f33f6fe99bd366dae49c45d2c3d288fdd852024103e85c07002d", "Hex-encoded Hash", "Probably SHA-224");
        _assert("4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865", "Hex-encoded Hash", "Probably SHA-256");
        _assert(
            "d654902b550e334bb6898d5c4ab8ebe1aedc6c85368eafe28e0f89b62a74a23e1ed20abbc10c02ce321266384d444717",
            "Hex-encoded Hash",
            "Probably SHA-384",
        );
        _assert(
            "3abb6677af34ac57c0ca5828fd94f9d886c26ce59a8ce60ecf6778079423dccff1d6f19cb655805d56098e6d38a1a710dee59523eed7511e5a9e4b8ccb3a4686",
            "Hex-encoded Hash",
            "Probably SHA-512",
        );
        _assert("QmbWqxBEKC3P8tqsKc98xmWNzrzDtRLMiMPL8wBuTGsMnR", "IPFS", "CID v0");
        _assert("bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi", "IPFS", "CID v1");
        _assert("k51qzi5uqu5dlvj2baxnqndepeb86cbk3ng7n3i46uzyxzyqj2xjonzllnv0v8", "IPFS", "CID v1 (IPNS)");
    }

    #[test]
    fn test_force_format() {
        fn _assert(id: &str, force: IdFormat, id_type: &str, version: &str) {
            let id_info = force_format(&Args {
                id: id.to_string(),
                force: Some(force),
                ..Default::default()
            })
            .unwrap();
            assert_eq!(id_info.id_type, id_type.to_string(), "{id} - {id_type} - {version}");
            assert_eq!(id_info.version.unwrap_or("-".to_string()), version.to_string(), "{id} - {id_type} - {version}");
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
        _assert(
            "2093703425379131962944436515747969848",
            IdFormat::UuidInt,
            "Integer of UUID (RFC-9562)",
            "7 (sortable timestamp and random)",
        );
        // Snowflakes:
        _assert("1777150623882019211", IdFormat::SfTwitter, "Snowflake", "Twitter");
        _assert("1304369705066434662", IdFormat::SfDiscord, "Snowflake", "Discord");
        _assert("1671390786412876801", IdFormat::SfInstagram, "Snowflake", "Instagram");
        _assert("540226260526170119", IdFormat::SfSony, "Snowflake", "Sony");
        _assert("1015189130756840860", IdFormat::SfSpaceflake, "Snowflake", "Spaceflake");
        _assert("112277929257317646", IdFormat::SfMastodon, "Snowflake", "Mastodon");
        _assert("7256902784527069184", IdFormat::SfLinkedin, "Snowflake", "LinkedIn");
        _assert("5828128208445124608", IdFormat::SfFlakeid, "Snowflake", "Flake ID");
        _assert("7423342004626526207", IdFormat::SfFrostflake, "Snowflake", "Frostflake");
        _assert("JERHwh5PXjL", IdFormat::SfFrostflake, "Snowflake", "Frostflake");
        // Unix timestamp
        _assert("1734971723", IdFormat::Unix, "Unix timestamp", "Assuming seconds");
        _assert("1734971723000", IdFormat::Unix, "Unix timestamp", "Assuming milliseconds");
        _assert("1734971723000000", IdFormat::Unix, "Unix timestamp", "Assuming microseconds");
        _assert("1734971723000000000", IdFormat::Unix, "Unix timestamp", "Assuming nanoseconds");
        _assert("1734971723", IdFormat::UnixS, "Unix timestamp", "As seconds");
        _assert("1734971723000", IdFormat::UnixMs, "Unix timestamp", "As milliseconds");
        _assert("1734971723000000", IdFormat::UnixUs, "Unix timestamp", "As microseconds");
        _assert("1734971723000000000", IdFormat::UnixNs, "Unix timestamp", "As nanoseconds");
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
        _assert("6772800700000000d97a8af26532e259", IdFormat::Datadog, "Datadog Trace ID", "-");
        _assert("EQyuCsA4ysv7ezXReOrk4i", IdFormat::Nuid, "NUID", "-");
        _assert("prefix_01h2xcejqtf2nbrexx3vqjhp41", IdFormat::Typeid, "TypeID", "-");
        _assert("9NU6-XQLZ-BDIH-6HKE", IdFormat::Breezeid, "Breeze ID", "Default alphabet");
        _assert("9nu6-xqlz-bdih-6hke", IdFormat::Breezeid, "Breeze ID", "Lowercase alphabet");
        _assert("he5fps6l2504cd1w3ag8ut8e", IdFormat::Puid, "Puid", "-");
        _assert("aeby6ob5sso4zd", IdFormat::Puid, "Puid", "Short puid with node ID");
        _assert("aeby6ob5sso4", IdFormat::Puid, "Puid", "Short puid without node ID");
        _assert("-OFrJ24CPTXLcIPPjvh3", IdFormat::Pushid, "PushID (Firebase)", "-");
        // Hash-based:
        _assert("b026324c6904b2a9cb4b88d6d61c81d1", IdFormat::Hash, "Hex-encoded Hash", "Probably MD5");
        _assert("e5fa44f2b31c1fb553b6021e7360d07d5d91ff5e", IdFormat::Hash, "Hex-encoded Hash", "Probably SHA-1");
        _assert("b265f33f6fe99bd366dae49c45d2c3d288fdd852024103e85c07002d", IdFormat::Hash, "Hex-encoded Hash", "Probably SHA-224");
        _assert(
            "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865",
            IdFormat::Hash,
            "Hex-encoded Hash",
            "Probably SHA-256",
        );
        _assert(
            "d654902b550e334bb6898d5c4ab8ebe1aedc6c85368eafe28e0f89b62a74a23e1ed20abbc10c02ce321266384d444717",
            IdFormat::Hash,
            "Hex-encoded Hash",
            "Probably SHA-384",
        );
        _assert(
            "3abb6677af34ac57c0ca5828fd94f9d886c26ce59a8ce60ecf6778079423dccff1d6f19cb655805d56098e6d38a1a710dee59523eed7511e5a9e4b8ccb3a4686",
            IdFormat::Hash,
            "Hex-encoded Hash",
            "Probably SHA-512",
        );
        _assert("QmbWqxBEKC3P8tqsKc98xmWNzrzDtRLMiMPL8wBuTGsMnR", IdFormat::Ipfs, "IPFS", "CID v0");
        _assert("bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi", IdFormat::Ipfs, "IPFS", "CID v1");
        _assert("k51qzi5uqu5dlvj2baxnqndepeb86cbk3ng7n3i46uzyxzyqj2xjonzllnv0v8", IdFormat::Ipfs, "IPFS", "CID v1 (IPNS)");
    }
}
