use crate::schema::{Args, IDInfo, IdFormat};
use crate::tsid::parse_tsid;
use crate::utils::{bits64, milliseconds_to_seconds_and_iso8601};
use std::fmt::Write;

#[derive(Debug)]
struct SnowflakeAnnotation {
    version: Option<String>,
    datetime: Option<String>,
    timestamp: Option<String>,
    node1: Option<String>,
    node2: Option<String>,
    sequence: Option<u128>,
    color_map: Option<String>,
}

impl Default for SnowflakeAnnotation {
    fn default() -> SnowflakeAnnotation {
        Self {
            version: Some("Unknown (use -f to specify version)".to_string()),
            datetime: None,
            timestamp: None,
            node1: None,
            node2: None,
            sequence: None,
            color_map: Some("0000000000000000000000000000000000000000000000000000000000000000".to_string()),
        }
    }
}

fn annotate_twitter(args: &Args) -> SnowflakeAnnotation {
    let id_int: u64 = args.id.trim().parse().unwrap();
    let timestamp_raw = bits64(id_int, 1, 41);
    let worker_id = bits64(id_int, 42, 10);
    let sequence = bits64(id_int, 52, 12);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, Some(1288834974657));
    SnowflakeAnnotation {
        version: Some("Twitter".to_string()),
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        node1: Some(format!("{} (Worker ID)", worker_id)),
        node2: None,
        sequence: Some(sequence as u128),
        color_map: Some("0333333333333333333333333333333333333333334444444444666666666666".to_string()),
    }
}

fn annotate_discord(args: &Args) -> SnowflakeAnnotation {
    let id_int: u64 = args.id.trim().parse().unwrap();
    let timestamp_raw = bits64(id_int, 0, 42);
    let worker_id = bits64(id_int, 42, 5);
    let process_id = bits64(id_int, 47, 5);
    let sequence = bits64(id_int, 52, 12);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, Some(1420070400000));
    SnowflakeAnnotation {
        version: Some("Discord".to_string()),
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        node1: Some(format!("{} (Worker ID)", worker_id)),
        node2: Some(format!("{} (Process ID)", process_id)),
        sequence: Some(sequence as u128),
        color_map: Some("3333333333333333333333333333333333333333334444455555666666666666".to_string()),
    }
}

fn annotate_instagram(args: &Args) -> SnowflakeAnnotation {
    let id_int: u64 = args.id.trim().parse().unwrap();
    let timestamp_raw = bits64(id_int, 0, 41);
    let shard_id = bits64(id_int, 41, 13);
    let sequence = bits64(id_int, 54, 10);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, Some(1314220021721));
    SnowflakeAnnotation {
        version: Some("Instagram".to_string()),
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        node1: Some(format!("{} (Shard ID)", shard_id)),
        node2: None,
        sequence: Some(sequence as u128),
        color_map: Some("3333333333333333333333333333333333333333344444444444446666666666".to_string()),
    }
}

fn annotate_sony(args: &Args) -> SnowflakeAnnotation {
    let id_int: u64 = args.id.trim().parse().unwrap();
    let timestamp_raw = bits64(id_int, 1, 39);
    let sequence = bits64(id_int, 40, 8);
    let machine_id = bits64(id_int, 48, 16);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw * 10, Some(1409529600000));
    SnowflakeAnnotation {
        version: Some("Sony".to_string()),
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        node1: Some(format!("{} (Machine ID)", machine_id)),
        node2: None,
        sequence: Some(sequence as u128),
        color_map: Some("0333333333333333333333333333333333333333666666664444444444444444".to_string()),
    }
}

fn annotate_spaceflake(args: &Args) -> SnowflakeAnnotation {
    let id_int: u64 = args.id.trim().parse().unwrap();
    let timestamp_raw = bits64(id_int, 1, 41);
    let node_id = bits64(id_int, 42, 5);
    let worker_id = bits64(id_int, 47, 5);
    let sequence = bits64(id_int, 52, 12);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, Some(1420070400000));
    SnowflakeAnnotation {
        version: Some("Spaceflake".to_string()),
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        node1: Some(format!("{} (Node ID)", node_id)),
        node2: Some(format!("{} (Worker ID)", worker_id)),
        sequence: Some(sequence as u128),
        color_map: Some("0333333333333333333333333333333333333333334444455555666666666666".to_string()),
    }
}

fn annotate_linkedin(args: &Args) -> SnowflakeAnnotation {
    let id_int: u64 = args.id.trim().parse().unwrap();
    let timestamp_raw = bits64(id_int, 1, 41);
    let worker_id = bits64(id_int, 42, 10);
    let sequence = bits64(id_int, 52, 12);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, None);
    SnowflakeAnnotation {
        version: Some("LinkedIn".to_string()),
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        node1: Some(format!("{} (Worker ID)", worker_id)),
        node2: None,
        sequence: Some(sequence as u128),
        color_map: Some("0333333333333333333333333333333333333333334444444444666666666666".to_string()),
    }
}

fn annotate_mastodon(args: &Args) -> SnowflakeAnnotation {
    let id_int: u64 = args.id.trim().parse().unwrap();
    let timestamp_raw = bits64(id_int, 0, 48);
    let sequence = bits64(id_int, 48, 16);
    let (timestamp, datetime) = milliseconds_to_seconds_and_iso8601(timestamp_raw, None);
    SnowflakeAnnotation {
        version: Some("Mastodon".to_string()),
        datetime: Some(datetime),
        timestamp: Some(timestamp),
        node1: None,
        node2: None,
        sequence: Some(sequence as u128),
        color_map: Some("3333333333333333333333333333333333333333333333336666666666666666".to_string()),
    }
}

fn annotate_snowflake_variant(args: &Args) -> SnowflakeAnnotation {
    match args.force {
        Some(value) => {
            let annotation: SnowflakeAnnotation = match value {
                IdFormat::SfTwitter => annotate_twitter(args),
                IdFormat::SfMastodon => annotate_mastodon(args),
                IdFormat::SfDiscord => annotate_discord(args),
                IdFormat::SfInstagram => annotate_instagram(args),
                IdFormat::SfSony => annotate_sony(args),
                IdFormat::SfSpaceflake => annotate_spaceflake(args),
                IdFormat::SfLinkedin => annotate_linkedin(args),
                _ => SnowflakeAnnotation::default(),
            };
            annotation
        }
        None => SnowflakeAnnotation::default(),
    }
}

pub fn parse_snowflake(args: &Args) -> Option<IDInfo> {
    let id_int: u64 = match args.id.trim().parse::<u64>() {
        Ok(value) => value,
        Err(_) => return None,
    };

    let annotation = annotate_snowflake_variant(args);

    Some(IDInfo {
        id_type: "Snowflake".to_string(),
        version: annotation.version,
        standard: id_int.to_string(),
        integer: Some(id_int as u128),
        short_uuid: None,
        base64: None,
        uuid_wrap: None,
        size: 64,
        entropy: 0,
        datetime: annotation.datetime,
        timestamp: annotation.timestamp,
        sequence: annotation.sequence,
        node1: annotation.node1,
        node2: annotation.node2,
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: annotation.color_map,
    })
}

pub fn compare_snowflake(args: &Args) {
    if args.id.trim().parse::<u64>().is_ok() {
        println!("Date/times of the Snowflake ID if parsed as:");
        let tsid_time: String = match parse_tsid(args) {
            Some(value) => value.datetime.unwrap(),
            None => "-".to_string(),
        };
        let mut snowflake_times = vec![
            format!("- {} Twitter", annotate_twitter(args).datetime.unwrap_or("".to_string())),
            format!("- {} Discord", annotate_discord(args).datetime.unwrap_or("".to_string())),
            format!("- {} Instagram", annotate_instagram(args).datetime.unwrap_or("".to_string())),
            format!("- {} Sony", annotate_sony(args).datetime.unwrap_or("".to_string())),
            format!("- {} Spaceflake", annotate_spaceflake(args).datetime.unwrap_or("".to_string())),
            format!("- {} LinkedIn", annotate_linkedin(args).datetime.unwrap_or("".to_string())),
            format!("- {} Mastodon", annotate_mastodon(args).datetime.unwrap_or("".to_string())),
            format!("- {} TSID", tsid_time), // Not Snowflake, but 64-bit compatible:
        ];
        snowflake_times.sort();
        for time in &snowflake_times {
            println!("{time}");
        }
    } else {
        println!("Not a valid Snowflake ID.");
    }
    std::process::exit(0);
}
