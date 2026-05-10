use std::cmp;
use std::io::{Write, stdout};

use chrono::Utc;
use clap::ValueEnum;
use colored::*;
use timediff::TimeDiff;

use crate::schema::{Args, IDInfo, Output};

fn truncate_to_millis(ts: &str) -> &str {
    match ts.find('.') {
        Some(dot) => {
            let end = (dot + 4).min(ts.len());
            &ts[..end]
        }
        None => ts,
    }
}

fn truncate_datetime_to_millis(dt: &str) -> String {
    if let Some(dot) = dt.find('.') {
        let after_dot = &dt[dot + 1..];
        let non_digit_pos = after_dot.find(|c: char| !c.is_ascii_digit()).unwrap_or(after_dot.len());
        let digits = &after_dot[..non_digit_pos];
        let suffix = &after_dot[non_digit_pos..];
        let millis: &str = if digits.len() >= 3 { &digits[..3] } else { digits };
        format!("{}.{}{}", &dt[..dot], millis, suffix)
    } else {
        dt.to_string()
    }
}

impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value().expect("-").get_name().fmt(f)
    }
}

impl IDInfo {
    #[allow(clippy::indexing_slicing)]
    pub fn print_card(&self, args: &Args) {
        let timestamp = match self.timestamp.as_deref() {
            Some(value) => {
                let ts_display = truncate_to_millis(value);
                let dt_display = truncate_datetime_to_millis(self.datetime.as_deref().unwrap_or("-"));
                format!("{} ({})", ts_display, dt_display)
            }
            None => "-".to_string(),
        };

        const MIN_R_SPACE: usize = 43;
        let l_space = 9;
        let r_space = cmp::max(MIN_R_SPACE, timestamp.chars().count());

        fn limit_r(text: String) -> String {
            match text.char_indices().nth(MIN_R_SPACE) {
                None => text,
                Some((idx, _)) => format!("{}...", &text[..idx - 3]),
            }
        }

        let size = match self.size {
            0 => "-",
            _ => match &self.parsed {
                Some(parsed) => &format!("{} bits ({})", self.size, parsed),
                None => &format!("{} bits", self.size),
            },
        };

        let entropy = match self.size {
            0 => "-",
            _ => &format!("{} bits", self.entropy),
        };

        println!("┏━{:━<l_space$}━┯{:━<r_space$}━━┓", "", "");
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "ID Type", self.id_type);
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Version".yellow(), self.version.as_deref().unwrap_or("-"));
        println!("┠─{:─<l_space$}─┼─{:─<r_space$}─┨", "", "");
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "String", limit_r(self.standard.clone()));

        if let Some(value) = self.integer {
            println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Integer", value);
        }
        if let Some(value) = self.uuid_wrap.as_deref() {
            println!("┃ {:<l_space$} │ {:<r_space$} ┃", "UUID wrap", value);
        }
        let sequence = match self.sequence {
            Some(value) => value.to_string(),
            None => "-".to_string(),
        };
        println!("┠─{:─<l_space$}─┼─{:─<r_space$}─┨", "", "");
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Size", size);
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Entropy".green(), entropy);
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Timestamp".cyan(), timestamp);

        if let Some(value) = self.relative_time.clone()
            && args.relative
        {
            println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Relative".cyan(), value);
        }

        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Node 1".purple(), limit_r(self.node1.clone().unwrap_or("-".to_string())));
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Node 2".red(), limit_r(self.node2.clone().unwrap_or("-".to_string())));
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Sequence".blue(), sequence);
        println!("┠─{:─<l_space$}─┼─{:─<r_space$}─┨", "", "");

        let (hex_lines, bin_lines) = self.get_hex_bin_lines();
        let fix_space = r_space - MIN_R_SPACE; // The colored rendering messes with the count.
        for (i, hex_line) in hex_lines.into_iter().enumerate() {
            if self.bits.is_some() {
                println!("┃ {:<l_space$} │ {}{:<fix_space$} ┃", hex_line, bin_lines[i], "");
            } else {
                println!("┃ {:<l_space$} │ {:<r_space$} ┃", hex_line, bin_lines[i]);
            }
        }
        println!("┗━{:━<l_space$}━┷{:━<r_space$}━━┛", "", "");
    }

    fn get_hex_bin_lines(&self) -> (Vec<String>, Vec<String>) {
        let mut bin_lines: Vec<String> = vec![];
        let mut hex_lines: Vec<String> = vec![];

        match (&self.bits, &self.color_map, &self.hex) {
            (Some(bits), Some(color_map), Some(hex)) => {
                let remaining_bits = (32 - (bits.chars().count() % 32)) % 32;
                let padded_bits: String = format!("{}{}", bits, ".".repeat(remaining_bits));
                let padded_color_map: String = format!("{}{}", color_map, "0".repeat(remaining_bits));
                let padded_hex: String = format!("{}{}", hex, ".".repeat(remaining_bits / 4));
                let hex_chars: Vec<char> = padded_hex.chars().collect();
                let color_chars: Vec<char> = padded_color_map.chars().collect();

                let mut bin_line = String::new();
                let mut hex_line = String::new();
                for (i, c) in padded_bits.chars().enumerate() {
                    let cs = c.to_string();
                    let colored_bit = match color_chars.get(i) {
                        Some('1') => format!("{}", cs.yellow()),
                        Some('2') => format!("{}", cs.green()),
                        Some('3') => format!("{}", cs.cyan()),
                        Some('4') => format!("{}", cs.purple()),
                        Some('5') => format!("{}", cs.red()),
                        Some('6') => format!("{}", cs.blue()),
                        _ => format!("{}", cs.normal()),
                    };
                    bin_line.push_str(&colored_bit);
                    if ((i + 1) % 4) == 0 {
                        bin_line.push(' ');
                        if let Some(&hc) = hex_chars.get(i / 4) {
                            hex_line.push(hc);
                        }
                    }
                    if ((i + 1) % 8) == 0 {
                        bin_line.push(' ');
                    }
                    if ((i + 1) % 16) == 0 {
                        bin_line.push(' ');
                        hex_line.push(' ');
                    }
                    if ((i + 1) % 32) == 0 {
                        bin_lines.push(bin_line.trim().to_string());
                        hex_lines.push(hex_line.trim().to_string());
                        bin_line = String::new();
                        hex_line = String::new();
                    }
                }
            }
            _ => {
                bin_lines.push("No bits (non-numeric ID)".to_string());
                hex_lines.push("No hex".to_string());
            }
        }
        (hex_lines, bin_lines)
    }

    pub fn print_short(&self) {
        match &self.version {
            Some(version) => println!("ID Type: {}, version: {}.", self.id_type, version),
            None => println!("ID Type: {}.", self.id_type),
        }
    }

    pub fn print_json(&self) {
        match serde_json::to_string(self) {
            Ok(json) => println!("{}", json),
            Err(_) => {
                println!("Error rendering JSON");
                std::process::exit(3);
            }
        };
    }

    pub fn print_binary(&self) {
        match &self.integer {
            Some(number) => {
                let slice_offset: usize = ((128 - &self.size) / 8).into();
                stdout().write_all(number.to_be_bytes().get(slice_offset..16).unwrap()).unwrap();
            }
            None => match &self.hex {
                Some(value) => {
                    let bytes = hex::decode(value).unwrap();
                    stdout().write_all(&bytes).unwrap();
                }
                None => println!("{}", self.standard),
            },
        }
    }

    pub fn print(&mut self, args: &Args) {
        if self.timestamp.is_some() {
            let timestamp_sec = self.timestamp.clone().unwrap().parse::<f64>().unwrap_or_default() as i64;
            let diff = timestamp_sec - Utc::now().timestamp();
            self.relative_time = Some(TimeDiff::to_diff((diff).to_string() + "s").parse().unwrap_or('-'.to_string()));
        }
        if args.everything {
            self.print_card(args);
        } else {
            match args.output {
                Output::Short => self.print_short(),
                Output::Json => self.print_json(),
                Output::Binary => self.print_binary(),
                _ => self.print_card(args),
            }
        }
    }
}
