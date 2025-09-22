use colored::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::formats::snowflake::{
    SnowflakeAnnotation, annotate_discord, annotate_flakeid, annotate_frostflake, annotate_instagram, annotate_linkedin, annotate_mastodon, annotate_sony, annotate_spaceflake, annotate_twitter,
};
use crate::id_format::ALL_PARSERS;
use crate::schema::{Args, TimestampComparable};
use crate::utils::nanoseconds_to_iso8601;

const NOW_DISPLAY: &str = "--- Now ---";
const SNOWFLAKE_ANNOTATE_FUNCTIONS: [fn(&Args) -> SnowflakeAnnotation; 9] = [
    annotate_twitter,
    annotate_discord,
    annotate_instagram,
    annotate_sony,
    annotate_spaceflake,
    annotate_linkedin,
    annotate_mastodon,
    annotate_frostflake,
    annotate_flakeid,
];

fn all_parsers_times(args: &Args) -> Vec<TimestampComparable> {
    let mut all_times: Vec<TimestampComparable> = vec![];
    for parser in ALL_PARSERS {
        if let Some(value) = parser(args)
            && value.datetime.is_some()
        {
            all_times.push(TimestampComparable {
                timestamp: value.timestamp.clone().unwrap_or_default().parse::<f64>().unwrap_or_default(),
                datetime: value.datetime.unwrap_or_default(),
                name: match value.version {
                    Some(version) => format!("{}: {}", value.id_type, version),
                    None => value.id_type,
                },
            });
        }
    }
    all_times
}

pub fn snowflake_times(args: &Args) -> Vec<TimestampComparable> {
    if args.id.trim().parse::<u64>().is_err() {
        return vec![];
    }
    let mut snowflake_times: Vec<TimestampComparable> = vec![];
    for annotate_fn in SNOWFLAKE_ANNOTATE_FUNCTIONS {
        let annotated = annotate_fn(args);
        snowflake_times.push(TimestampComparable {
            timestamp: annotated.timestamp.clone().unwrap_or_default().parse::<f64>().unwrap_or_default(),
            datetime: annotated.datetime.unwrap_or_default(),
            name: format!("Snowflake: {}", annotated.version.unwrap_or_default()),
        });
    }
    snowflake_times
}

pub fn compare_times(args: &Args) {
    let mut all_times: Vec<TimestampComparable> = vec![];

    all_times.extend(all_parsers_times(args));
    all_times.extend(snowflake_times(args));

    if !all_times.is_empty() {
        println!("Date/times of the valid IDs parsed as:");
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        all_times.push(TimestampComparable {
            timestamp: now as f64 / 1_000_000_000.0,
            datetime: nanoseconds_to_iso8601(now).1,
            name: NOW_DISPLAY.to_string(),
        });
    } else {
        println!("This ID is not valid in any time-aware format.");
    }

    all_times.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());
    for time in all_times {
        let line = format!("- {} {}", time.datetime, time.name);
        if time.name == *NOW_DISPLAY {
            println!("{}", line.yellow());
        } else {
            println!("{}", line);
        }
    }

    std::process::exit(0);
}
