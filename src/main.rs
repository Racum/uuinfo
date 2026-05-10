use clap::Parser;
use std::io;

mod compare;
mod formats;
mod id_format;
mod schema;
mod utils;

use crate::compare::compare_times;
use crate::id_format::{auto_detect, force_format, parse_all};
use crate::schema::Args;
mod display;

fn main() {
    let mut args = Args::parse();

    if args.compare {
        compare_times(&args)
    }

    if args.id == "-" {
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_ok()
            && let Some(value) = buffer.split('\n').next()
        {
            args.id = value.to_string();
        }
    }

    if args.everything {
        let valid_ids = parse_all(&args);
        if !valid_ids.is_empty() {
            for mut value in valid_ids {
                value.print(&args);
            }
        } else {
            println!("Unknown ID type.");
        }
    } else {
        let result = if args.force.is_some() { force_format(&args) } else { auto_detect(&args) };
        match result {
            Some(mut value) => value.print(&args),
            None => {
                let msg = if args.force.is_some() { "Invalid ID for this format." } else { "Unknown ID type." };
                println!("{}", msg);
                std::process::exit(1);
            }
        }
    }
}
