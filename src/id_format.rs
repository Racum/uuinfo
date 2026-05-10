use crate::schema::{Args, IDInfo, IdFormat};

use crate::formats::asin::parse_asin;
use crate::formats::bitcoin::parse_bitcoin;
use crate::formats::breezeid::parse_breezeid;
use crate::formats::commerce::parse_commerce;
use crate::formats::cuid::{parse_cuid1, parse_cuid2};
use crate::formats::datadog::parse_datadog;
use crate::formats::duns::parse_duns;
use crate::formats::ethereum::parse_ethereum;
use crate::formats::flake::parse_flake;
use crate::formats::gdocs::parse_gdocs;
use crate::formats::geo::parse_h3;
use crate::formats::hash::parse_hash;
use crate::formats::hashid::parse_hashid;
use crate::formats::iban::parse_iban;
use crate::formats::ipfs::parse_ipfs;
use crate::formats::isbn::{parse_isbn, parse_isbn10};
use crate::formats::ksuid::parse_ksuid;
use crate::formats::nano64::parse_nano64;
use crate::formats::nanoid::parse_nanoid;
use crate::formats::network::{parse_imei, parse_ipv4, parse_ipv6, parse_mac};
use crate::formats::nuid::parse_nuid;
use crate::formats::objectid::parse_objectid;
use crate::formats::orderlyid::parse_orderlyid;
use crate::formats::puid::{parse_puid, parse_puid_any, parse_shortpuid};
use crate::formats::pushid::parse_pushid;
use crate::formats::scru::{parse_scru64, parse_scru128};
use crate::formats::slack::parse_slack;
use crate::formats::snowflake::parse_snowflake;
use crate::formats::snowid::parse_snowid;
use crate::formats::spotify::parse_spotify;
use crate::formats::sqid::parse_sqid;
use crate::formats::stripe::parse_stripe;
use crate::formats::swhid::parse_swhid;
use crate::formats::threads::parse_threads;
use crate::formats::tid::parse_tid;
use crate::formats::timeflake::{parse_timeflake_any, parse_timeflake_base62};
use crate::formats::tsid::parse_tsid;
use crate::formats::typeid::parse_typeid;
use crate::formats::ulid::{parse_julid, parse_ulid, parse_ulid_any};
use crate::formats::unix::{parse_unix, parse_unix_ms, parse_unix_ns, parse_unix_recent, parse_unix_s, parse_unix_us};
use crate::formats::upid::parse_upid;
use crate::formats::uuid::{parse_base64_uuid, parse_short_uuid, parse_uuid, parse_uuid_integer, parse_uuid25};
use crate::formats::vin::parse_vin;
use crate::formats::xid::parse_xid;
use crate::formats::youtube::parse_youtube;

type ParseFunction = fn(&Args) -> Option<IDInfo>;

#[rustfmt::skip]
pub static ALL_PARSERS: &[ParseFunction] = &[
    parse_uuid,
    parse_base64_uuid,
    parse_uuid25,
    parse_short_uuid,
    parse_uuid_integer,
    parse_ulid,
    parse_julid,
    parse_upid,
    parse_objectid,
    parse_ksuid,
    parse_xid,
    parse_scru128,
    parse_scru64,
    parse_timeflake_any,
    parse_flake,
    parse_tsid,
    parse_nuid,
    parse_typeid,
    parse_pushid,
    parse_orderlyid,
    parse_threads,
    parse_snowid,
    parse_nano64,
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
    parse_ipv4,
    parse_ipv6,
    parse_mac,
    parse_isbn10,
    parse_tid,
    parse_duns,
    parse_asin,
    parse_nanoid,
    parse_cuid1,
    parse_cuid2,
    parse_h3,
    parse_imei,
    parse_gdocs,
    parse_slack,
    parse_spotify,
    parse_swhid,
    parse_iban,
    parse_bitcoin,
    parse_ethereum,
    parse_commerce,
    parse_vin,
];

pub fn parse_all(args: &Args) -> Vec<IDInfo> {
    let mut valid_ids: Vec<IDInfo> = vec![];
    for parser in ALL_PARSERS {
        if let Some(value) = parser(args)
            && value.high_confidence
        {
            valid_ids.push(value);
        }
    }
    valid_ids
}

pub fn pick_first_valid(args: &Args, parsers: &[ParseFunction]) -> Option<IDInfo> {
    for parser in parsers {
        if let Some(value) = parser(args) {
            return Some(value);
        }
    }
    None
}

pub fn auto_detect(args: &Args) -> Option<IDInfo> {
    let mut id_info: Option<IDInfo>;
    if let Some(result) = parse_iban(args) {
        return Some(result);
    }
    if args.id.trim().parse::<u128>().is_ok() {
        // Numeric:
        id_info = pick_first_valid(args, &[parse_isbn, parse_imei, parse_unix_recent, parse_snowflake, parse_uuid_integer]);
    } else {
        // Fixed length:
        id_info = match args.id.chars().count() {
            62 => parse_bitcoin(args),
            56 | 64 | 96 | 128 => parse_hash(args),
            44 => parse_gdocs(args),
            42 => pick_first_valid(args, &[parse_ethereum, parse_bitcoin]),
            40 => parse_ksuid(args),
            34 => parse_bitcoin(args),
            32 | 36 => pick_first_valid(args, &[parse_datadog, parse_uuid]),
            27 => pick_first_valid(args, &[parse_upid, parse_ksuid]),
            26 => parse_ulid_any(args),
            25 => pick_first_valid(args, &[parse_cuid1, parse_scru128]),
            24 => pick_first_valid(args, &[parse_objectid, parse_puid, parse_base64_uuid]),
            22 => pick_first_valid(args, &[parse_short_uuid, parse_timeflake_base62, parse_base64_uuid, parse_nuid, parse_spotify]),
            21 => parse_nanoid(args),
            20 => pick_first_valid(args, &[parse_xid, parse_stripe, parse_pushid]),
            18 => parse_flake(args),
            17 => pick_first_valid(args, &[parse_vin, parse_nano64]),
            16 => parse_nano64(args),
            15 => parse_h3(args),
            14 => parse_shortpuid(args),
            13 => pick_first_valid(args, &[parse_tid, parse_tsid]),
            12 => pick_first_valid(args, &[parse_scru64, parse_shortpuid]),
            11 => pick_first_valid(args, &[parse_slack, parse_youtube, parse_snowid]),
            10 => pick_first_valid(args, &[parse_asin, parse_snowid]),
            _ => None,
        };
        // Variable length:
        id_info = match id_info {
            Some(value) => Some(value),
            #[rustfmt::skip]
            None => pick_first_valid(args, &[
                parse_vin,
                parse_orderlyid,
                parse_isbn,
                parse_commerce,
                parse_typeid,
                parse_ipfs,
                parse_stripe,
                parse_breezeid,
                parse_ipv4,
                parse_ipv6,
                parse_mac,
                parse_cuid2,
                parse_sqid,
                parse_snowid,
                parse_duns,
                parse_threads,
                parse_imei,
                parse_hashid,
                parse_nanoid,
                parse_swhid,
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
        IdFormat::Julid => parse_julid(args),
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
        IdFormat::Ipv4 => parse_ipv4(args),
        IdFormat::Ipv6 => parse_ipv6(args),
        IdFormat::Mac => parse_mac(args),
        IdFormat::Imei => parse_imei(args),
        IdFormat::Isbn => parse_isbn(args),
        IdFormat::Tid => parse_tid(args),
        IdFormat::Threads => parse_threads(args),
        IdFormat::Duns => parse_duns(args),
        IdFormat::Asin => parse_asin(args),
        IdFormat::H3 => parse_h3(args),
        IdFormat::Snowid => parse_snowid(args),
        IdFormat::Gdocs => parse_gdocs(args),
        IdFormat::Slack => parse_slack(args),
        IdFormat::Spotify => parse_spotify(args),
        IdFormat::Nano64 => parse_nano64(args),
        IdFormat::Orderlyid => parse_orderlyid(args),
        IdFormat::Swhid => parse_swhid(args),
        IdFormat::Iban => parse_iban(args),
        IdFormat::Bitcoin => parse_bitcoin(args),
        IdFormat::Ethereum => parse_ethereum(args),
        IdFormat::Commerce => parse_commerce(args),
        IdFormat::Vin => parse_vin(args),
    }
}

#[cfg(test)]
#[path = "id_format_tests.rs"]
mod tests;
