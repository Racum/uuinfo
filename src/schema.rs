use clap::Parser;
use clap::ValueEnum;
use serde::Serialize;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Output {
    /// Pretty-printed information card
    Card,
    /// One line with only ID type and version
    Short,
    /// Parsed information as JSON
    Json,
    /// Raw binary representation of the ID
    Binary,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum IdFormat {
    /// UUID
    Uuid,
    /// ShortUUID
    Shortuuid,
    /// UUID as Integer
    UuidInt,
    /// UUID as Base64
    UuidB64,
    /// Uuid25
    Uuid25,
    /// ULID
    Ulid,
    /// UPID
    Upid,
    /// Timeflake
    Timeflake,
    /// Flake (Boundary)
    Flake,
    /// SCRU128,
    Scru128,
    /// SCRU64,
    Scru64,
    /// MongoDB ObjectId
    Mongodb,
    /// KSUID
    Ksuid,
    /// Xid
    Xid,
    /// CUID 1
    Cuid1,
    /// CUID 2
    Cuid2,
    /// Nano ID
    Nanoid,
    /// TSID
    Tsid,
    /// Sqid
    Sqid,
    /// Hashid
    Hashid,
    /// YouTube Video ID
    Youtube,
    /// Stripe ID
    Stripe,
    /// Datadog Trace ID
    Datadog,
    /// NUID (NATS)
    Nuid,
    /// TypeID (Jetify)
    Typeid,
    /// Breeze ID
    Breezeid,
    /// PUID
    Puid,
    /// PushID (Firebase)
    Pushid,
    /// TID (AT Protocol, Bluesky)
    Tid,
    /// Thread ID (Meta Threads)
    Threads,
    /// Snowflake: Twitter
    SfTwitter,
    /// Snowflake: Mastodon
    SfMastodon,
    /// Snowflake: Discord
    SfDiscord,
    /// Snowflake: Instagram "Shard ID"
    SfInstagram,
    /// Snowflake: LinkedIn
    SfLinkedin,
    /// Snowflake: Sony "Sonyflake"
    SfSony,
    /// Snowflake: Spaceflake
    SfSpaceflake,
    /// Snowflake: Frostflake
    SfFrostflake,
    /// Snowflake Flake ID
    SfFlakeid,
    /// Unix timestamp: Auto-detect
    Unix,
    /// Unix timestamp: Seconds
    UnixS,
    /// Unix timestamp: Milliseconds
    UnixMs,
    /// Unix timestamp: Microseconds
    UnixUs,
    /// Unix timestamp: Nanoseconds
    UnixNs,
    /// Hex-encoded Hash
    Hash,
    /// IPFS Address (CID, IPNS)
    Ipfs,
    /// Network: IPv4
    Ipv4,
    /// Network: IPv6
    Ipv6,
    /// Network: MAC Address
    Mac,
    /// Network: IMEI
    Imei,
    /// ISBN
    Isbn,
    /// Geo: H3 Grid System
    H3,
}

/// Shows debug information about complex ID.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// UUID, ULID, Snowflake or other IDs; use "-" for STDIN
    #[arg(allow_hyphen_values = true)]
    pub id: String,

    /// Output format
    #[arg(short, long, default_value_t = Output::Card)]
    pub output: Output,

    /// Force format
    #[arg(short = 'f', long)]
    pub force: Option<IdFormat>,

    /// Try to parse all known formats
    #[arg(short = 'e', long)]
    pub everything: bool,

    /// Compare times of different Snowflake versions
    #[arg(short = 'c', long)]
    pub compare: bool,

    /// Use custom alphabet for Sqids and Nono ID
    #[arg(short = 'a', long)]
    pub alphabet: Option<String>,

    /// Custom salt for Hashids
    #[arg(long)]
    pub salt: Option<String>,
}

impl Default for Args {
    fn default() -> Args {
        Self {
            id: "".to_string(),
            output: Output::Card,
            force: None,
            everything: false,
            compare: false,
            alphabet: None,
            salt: None,
        }
    }
}

#[derive(Debug)]
pub struct TimestampComparable {
    pub timestamp: f64,   // For sorting.
    pub datetime: String, // To show.
    pub name: String,     // To show: ID Type and Version
}

#[allow(dead_code)]
#[derive(Default, Clone, Serialize, Debug)]
pub struct IDInfo {
    pub id_type: String,
    pub version: Option<String>,
    pub standard: String,
    pub integer: Option<u128>,
    pub short_uuid: Option<String>,
    pub base64: Option<String>,
    pub uuid_wrap: Option<String>,
    pub parsed: Option<String>,
    pub size: u16,
    pub entropy: u16,
    pub datetime: Option<String>,
    pub timestamp: Option<String>,
    pub sequence: Option<u128>,
    pub node1: Option<String>,
    pub node2: Option<String>,
    pub hex: Option<String>,
    #[serde(skip_serializing)]
    pub bits: Option<String>,
    #[serde(skip_serializing)]
    pub color_map: Option<String>,
}

/*
IDInfo.color_map codes:
 - 0: neutral
 - 1: yellow (id type)
 - 2: green (entropy)
 - 3: cyan (timestamp)
 - 4: purple (node 1)
 - 5: red (node 2)
 - 6: blue (sequence)
*/
