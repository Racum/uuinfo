use clap::Parser;
use std::io;

mod formats;
mod id_format;
mod schema;
mod utils;

use crate::id_format::{auto_detect, compare_snowflake, force_format, parse_all};
use crate::schema::Args;
mod display;

fn main() {
    let mut args = Args::parse();

    if args.compare_snowflake {
        compare_snowflake(&args)
    }

    if &args.id == "-" {
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_ok() {
            if let Some(value) = buffer.split('\n').next() {
                args.id = value.to_string();
            }
        }
    }

    if args.everything {
        let valid_ids = parse_all(&args);
        if !valid_ids.is_empty() {
            for value in valid_ids {
                value.print(&args);
            }
        } else {
            println!("Unknown ID type.");
        }
    } else {
        match &args.force {
            Some(_) => match force_format(&args) {
                Some(value) => value.print(&args),
                None => {
                    println!("Invalid ID for this format.");
                    std::process::exit(1);
                }
            },
            None => match auto_detect(&args) {
                Some(value) => value.print(&args),
                None => {
                    println!("Unknown ID type.");
                    std::process::exit(1);
                }
            },
        };
    }
}
