use clap::Parser;
use clap::ValueEnum;
use colored::*;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Format {
    Basic,
    Ascii,
    Json,
    Binary,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SnowflakeVerion {
    /// Twitter
    Twitter,
    /// Mastodon
    Mastodon,
    /// Discord
    Discord,
    /// Instagram "Shard ID"
    Instagram,
    /// LinkedIn (also OnlineAppZone)
    Linkedin,
    /// Sony "Sonyflake"
    Sony,
    /// Spaceflake
    Spaceflake,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum UUIDAlternative {
    /// ULID
    Ulid,
    /// UPID
    Upid,
    /// Timeflake
    Timeflake,
    /// Flake
    Flake,
    /// TimeUUID
    Timeuuid,
    /// SCRU128
    SCRU128,
}

/// Shows debug information about complex ID.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// UUID, ULID, Snowflake or other IDs.
    pub id: String,

    /// Output format.
    #[arg(short, long)]
    pub format: Option<Format>,

    /// Compare times of different Snowflake versions.
    #[arg(short = 'c', long)]
    pub compare_snowflake: bool,

    /// Parse Snowflake as version.
    #[arg(short = 's', long)]
    pub snowflake: Option<SnowflakeVerion>,

    /// Force UUID wrapping alternative.
    #[arg(short = 'u', long)]
    pub uuid_alt: Option<UUIDAlternative>,

    /// Remove Base64 padding.
    #[arg(long)]
    pub b64_nopad: bool,

    /// Encode Base64 from UUID big-endian bytes.
    #[arg(long)]
    pub b64_bigendian: bool,

    #[arg(long)]
    pub alphabet: Option<String>,

    #[arg(long)]
    pub hashid_salt: Option<String>,
}

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct IDInfo {
    pub known: bool,
    pub id_type: String,
    pub version: Option<String>,
    pub standard: String,
    pub integer: Option<u128>,
    pub short_uuid: Option<String>,
    pub base64: Option<String>,
    pub uuid_wrap: Option<String>,
    pub size: u8,
    pub entropy: u8,
    pub datetime: Option<String>,
    pub timestamp: Option<String>,
    pub sequence: Option<u128>,
    pub node1: Option<String>,
    pub node2: Option<String>,
    pub hex: Option<String>,
    pub bits: Option<String>,
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

impl IDInfo {
    pub fn print(&self) {
        let lc = 9;
        let rc = 43;

        let size = match self.size {
            0 => "-",
            _ => &format!("{} bits", self.size),
        };

        let entropy = match self.size {
            0 => "-",
            _ => &format!("{} bits", self.entropy),
        };

        println!("┏━━━━━━━━━━━┯━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",);
        println!("┃ {:<lc$} │ {:<rc$} ┃", "ID Tyoe", self.id_type);
        println!(
            "┃ {:<lc$} │ {:<rc$} ┃",
            "Version".yellow(),
            self.version.as_deref().unwrap_or("-")
        );
        println!("┠───────────┼─────────────────────────────────────────────┨",);
        println!("┃ {:<lc$} │ {:<rc$} ┃", "String", self.standard);

        if let Some(value) = self.integer {
            println!("┃ {:<lc$} │ {:<rc$} ┃", "Integer", value);
        }
        if let Some(value) = self.uuid_wrap.as_deref() {
            println!("┃ {:<lc$} │ {:<rc$} ┃", "UUID wrap", value);
        }
        if let Some(value) = self.short_uuid.as_deref() {
            println!("┃ {:<lc$} │ {:<rc$} ┃", "ShortUUID", value);
        }
        if let Some(value) = self.base64.as_deref() {
            println!("┃ {:<lc$} │ {:<rc$} ┃", "Base64", value);
        }
        let timestamp = match self.timestamp.as_deref() {
            Some(value) => format!("{} ({})", value, self.datetime.as_deref().unwrap_or("-")),
            None => "-".to_string(),
        };
        let sequence = match self.sequence {
            Some(value) => value.to_string(),
            None => "-".to_string(),
        };
        println!("┠───────────┼─────────────────────────────────────────────┨",);
        println!("┃ {:<lc$} │ {:<rc$} ┃", "Size", size);
        println!("┃ {:<lc$} │ {:<rc$} ┃", "Entropy".green(), entropy);
        println!("┃ {:<lc$} │ {:<rc$} ┃", "Timestamp".cyan(), timestamp);
        println!(
            "┃ {:<lc$} │ {:<rc$} ┃",
            "Node 1".purple(),
            self.node1.as_deref().unwrap_or("-")
        );
        println!(
            "┃ {:<lc$} │ {:<rc$} ┃",
            "Node 2".red(),
            self.node2.as_deref().unwrap_or("-")
        );
        println!("┃ {:<lc$} │ {:<rc$} ┃", "Sequence".blue(), sequence);
        println!("┠───────────┼─────────────────────────────────────────────┨",);

        let (hex_lines, bin_lines) = self.get_hex_bin_lines();
        for (i, hex_line) in hex_lines.into_iter().enumerate() {
            println!("┃ {:<lc$} │ {:<rc$} ┃", hex_line, bin_lines[i]);
        }

        println!("┗━━━━━━━━━━━┷━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛",);
    }

    fn get_hex_bin_lines(&self) -> (Vec<String>, Vec<String>) {
        let mut bin_lines: Vec<String> = vec![];
        let mut hex_lines: Vec<String> = vec![];

        match self.bits.clone() {
            Some(bits) => {
                let mut bin_line: String = "".to_string();
                let mut hex_line: String = "".to_string();
                for (i, c) in bits.chars().enumerate() {
                    let colored_bit = match self.color_map.clone() {
                        Some(color_map) => match color_map.chars().nth(i).unwrap() {
                            '1' => format!("{}", c.to_string().yellow()),
                            '2' => format!("{}", c.to_string().green()),
                            '3' => format!("{}", c.to_string().cyan()),
                            '4' => format!("{}", c.to_string().purple()),
                            '5' => format!("{}", c.to_string().red()),
                            '6' => format!("{}", c.to_string().blue()),
                            _ => format!("{}", c.to_string().normal()),
                        },
                        None => c.to_string(),
                    };
                    bin_line.push_str(&colored_bit);
                    if ((i + 1) % 4) == 0 {
                        bin_line.push(' ');
                        hex_line.push(self.hex.clone().unwrap().chars().nth(i / 4).unwrap());
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
                        bin_line = "".to_string();
                        hex_line = "".to_string();
                    }
                }
            }
            None => {
                bin_lines.push("No bits".to_string());
                hex_lines.push("No hex".to_string());
            }
        }
        (hex_lines, bin_lines)
    }
}
