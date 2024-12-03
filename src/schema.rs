use clap::Parser;
use clap::ValueEnum;
use colored::*;
use serde::Serialize;
use std::io::stdout;
use std::io::Write;

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

impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value().expect("-").get_name().fmt(f)
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum IdFormat {
    /// UUID
    Uuid,
    /// ShortUUID
    Shortuuid,
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
    /// Snowflake: Twitter
    SfTwitter,
    /// Snowflake: Mastodon
    SfMastodon,
    /// Snowflake: Discord
    SfDiscord,
    /// Snowflake: Instagram "Shard ID"
    SfInstagram,
    /// Snowflake: LinkedIn (also OnlineAppZone)
    SfLinkedin,
    /// Snowflake: Sony "Sonyflake"
    SfSony,
    /// Snowflake: Spaceflake
    SfSpaceflake,
}

/// Shows debug information about complex ID.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// UUID, ULID, Snowflake or other IDs; use "-" for STDIN
    #[arg(allow_hyphen_values = true)]
    pub id: String,

    /// Output format.
    #[arg(short, long, default_value_t = Output::Card)]
    pub output: Output,

    /// Force format
    #[arg(short = 'f', long)]
    pub force: Option<IdFormat>,

    /// Compare times of different Snowflake versions.
    #[arg(short = 'c', long)]
    pub compare_snowflake: bool,

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
#[derive(Default, Clone, Serialize, Debug)]
pub struct IDInfo {
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

impl IDInfo {
    pub fn print_card(&self) {
        let l_space = 9;
        let r_space = 43;

        let size = match self.size {
            0 => "-",
            _ => &format!("{} bits", self.size),
        };

        let entropy = match self.size {
            0 => "-",
            _ => &format!("{} bits", self.entropy),
        };

        println!("┏━{:━<l_space$}━┯{:━<r_space$}━━┓", "", "");
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "ID Tyoe", self.id_type);
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Version".yellow(), self.version.as_deref().unwrap_or("-"));
        println!("┠─{:─<l_space$}─┼─{:─<r_space$}─┨", "", "");
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "String", self.standard);

        if let Some(value) = self.integer {
            println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Integer", value);
        }
        if let Some(value) = self.uuid_wrap.as_deref() {
            println!("┃ {:<l_space$} │ {:<r_space$} ┃", "UUID wrap", value);
        }
        if let Some(value) = self.short_uuid.as_deref() {
            println!("┃ {:<l_space$} │ {:<r_space$} ┃", "ShortUUID", value);
        }
        if let Some(value) = self.base64.as_deref() {
            println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Base64", value);
        }
        let timestamp = match self.timestamp.as_deref() {
            Some(value) => format!("{} ({})", value, self.datetime.as_deref().unwrap_or("-")),
            None => "-".to_string(),
        };
        let sequence = match self.sequence {
            Some(value) => value.to_string(),
            None => "-".to_string(),
        };
        println!("┠─{:─<l_space$}─┼─{:─<r_space$}─┨", "", "");
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Size", size);
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Entropy".green(), entropy);
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Timestamp".cyan(), timestamp);
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Node 1".purple(), self.node1.as_deref().unwrap_or("-"));
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Node 2".red(), self.node2.as_deref().unwrap_or("-"));
        println!("┃ {:<l_space$} │ {:<r_space$} ┃", "Sequence".blue(), sequence);
        println!("┠─{:─<l_space$}─┼─{:─<r_space$}─┨", "", "");

        let (hex_lines, bin_lines) = self.get_hex_bin_lines();
        let fix_space = r_space - 43; // The colored rendering messes with the count.
        for (i, hex_line) in hex_lines.into_iter().enumerate() {
            println!("┃ {:<l_space$} │ {}{:<fix_space$} ┃", hex_line, bin_lines[i], "");
        }
        println!("┗━{:━<l_space$}━┷{:━<r_space$}━━┛", "", "");
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
                stdout().write_all(&number.to_be_bytes()[slice_offset..16]).unwrap();
            }
            None => println!("{}", self.standard),
        }
    }

    pub fn print(&self, args: &Args) {
        match args.output {
            Output::Short => self.print_short(),
            Output::Json => self.print_json(),
            Output::Binary => self.print_binary(),
            _ => self.print_card(),
        }
    }
}
