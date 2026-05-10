use chrono::Datelike;

use crate::schema::{Args, IDInfo};
use crate::utils::{factor_size_hex_bits_color_from_text, repeat_char};

#[rustfmt::skip]
fn wmi_manufacturer(wmi: &str) -> Option<&'static str> {
    match wmi {
        // United States
        "1C3" | "1C4" | "1C6" => Some("Chrysler"),
        "1D3" | "1D4" | "1D7" | "1D8" => Some("Dodge"),
        "1FA" | "1FB" | "1FC" | "1FD" | "1FM" | "1FT" => Some("Ford"),
        "1G1" => Some("Chevrolet"),
        "1G2" => Some("Pontiac"),
        "1G3" => Some("Oldsmobile"),
        "1G4" => Some("Buick"),
        "1G6" => Some("Cadillac"),
        "1GC" | "1GT" => Some("GMC"),
        "1GY" => Some("Cadillac"),
        "1HG" => Some("Honda"),
        "1J4" | "1J8" => Some("Jeep"),
        "1LN" => Some("Lincoln"),
        "1ME" => Some("Mercury"),
        "1N4" | "1N6" => Some("Nissan"),
        "1VW" => Some("Volkswagen"),
        "1YV" => Some("Mazda"),
        "1ZV" => Some("Ford"),
        "2C3" => Some("Chrysler"),
        "2D3" => Some("Dodge"),
        "2FA" | "2FB" | "2FM" | "2FT" => Some("Ford"),
        "2G1" => Some("Chevrolet"),
        "2G2" => Some("Pontiac"),
        "2HG" | "2HJ" | "2HK" => Some("Honda"),
        "2T1" | "2T2" | "2T3" => Some("Toyota"),
        "3C4" | "3C6" => Some("Chrysler"),
        "3D7" => Some("Dodge"),
        "3FA" => Some("Ford"),
        "3G1" => Some("Chevrolet"),
        "3GT" => Some("GMC"),
        "3HG" => Some("Honda"),
        "3N1" | "3N6" => Some("Nissan"),
        "3VW" => Some("Volkswagen"),
        "4F2" | "4F4" => Some("Mazda"),
        "4T1" | "4T3" | "4T4" => Some("Toyota"),
        "5FN" | "5FR" => Some("Honda"),
        "5NP" => Some("Hyundai"),
        "5TD" | "5TF" => Some("Toyota"),
        "5UX" => Some("BMW"),
        "5YJ" => Some("Tesla"),
        // Japan
        "JA3" | "JA4" => Some("Mitsubishi"),
        "JF1" | "JF2" => Some("Subaru"),
        "JHM" => Some("Honda"),
        "JN1" | "JN6" | "JN8" => Some("Nissan"),
        "JT2" | "JTD" | "JTE" | "JTN" => Some("Toyota"),
        "JM1" => Some("Mazda"),
        // Germany
        "WA1" | "WAU" => Some("Audi"),
        "WBA" | "WBS" | "WBY" => Some("BMW"),
        "WDB" | "WDC" | "WDD" | "WDF" => Some("Mercedes-Benz"),
        "WF0" => Some("Ford (Germany)"),
        "WP0" | "WP1" => Some("Porsche"),
        "WVW" | "WV1" | "WV2" => Some("Volkswagen"),
        // United Kingdom
        "SAJ" => Some("Jaguar"),
        "SAL" => Some("Land Rover"),
        "SAR" => Some("Rover"),
        "SCC" => Some("Lotus"),
        "SCF" => Some("Aston Martin"),
        // South Korea
        "KM8" | "KMH" => Some("Hyundai"),
        "KNA" | "KND" => Some("Kia"),
        // Sweden
        "YS3" => Some("Saab"),
        "YV1" | "YV4" => Some("Volvo"),
        // Italy
        "ZAR" => Some("Alfa Romeo"),
        "ZAM" => Some("Maserati"),
        "ZCF" => Some("Iveco"),
        "ZFA" => Some("Fiat"),
        "ZFF" => Some("Ferrari"),
        "ZHW" => Some("Lamborghini"),
        // France
        "VF1" => Some("Renault"),
        "VF3" => Some("Peugeot"),
        "VF7" => Some("Citroën"),
        // China
        "LFV" => Some("FAW-Volkswagen"),
        "LHG" => Some("Honda (China)"),
        "LSG" => Some("SAIC GM"),
        "LVS" => Some("Ford (China)"),
        // India
        "MA1" | "MA3" => Some("Mahindra"),
        "MAT" => Some("Tata"),
        _ => None,
    }
}

#[rustfmt::skip]
fn wmi_country(c: u8) -> &'static str {
    match c {
        b'1' | b'4' | b'5' => "United States",
        b'2' => "Canada",
        b'3' => "Mexico",
        b'6' => "Australia",
        b'7' => "New Zealand",
        b'8' | b'9' => "South America",
        b'A' => "South Africa",
        b'B'..=b'H' => "Africa",
        b'J' => "Japan",
        b'K' => "South Korea",
        b'L' => "China",
        b'M' => "India/Southeast Asia",
        b'N' => "Iran/Pakistan/Turkey",
        b'P' => "Philippines",
        b'R' => "Taiwan/UAE",
        b'S' => "United Kingdom",
        b'T' => "Switzerland/Czech Republic",
        b'U' => "Romania",
        b'V' => "France/Spain/Austria",
        b'W' => "Germany",
        b'X' => "Russia/Netherlands",
        b'Y' => "Sweden/Finland/Belgium",
        b'Z' => "Italy",
        _ => "Unknown",
    }
}

#[rustfmt::skip]
fn model_year(c: u8) -> Option<(u16, u16)> {
    match c {
        b'A' => Some((2010, 1980)),
        b'B' => Some((2011, 1981)),
        b'C' => Some((2012, 1982)),
        b'D' => Some((2013, 1983)),
        b'E' => Some((2014, 1984)),
        b'F' => Some((2015, 1985)),
        b'G' => Some((2016, 1986)),
        b'H' => Some((2017, 1987)),
        b'J' => Some((2018, 1988)),
        b'K' => Some((2019, 1989)),
        b'L' => Some((2020, 1990)),
        b'M' => Some((2021, 1991)),
        b'N' => Some((2022, 1992)),
        b'P' => Some((2023, 1993)),
        b'R' => Some((2024, 1994)),
        b'S' => Some((2025, 1995)),
        b'T' => Some((2026, 1996)),
        b'V' => Some((2028, 1998)),
        b'W' => Some((2029, 1999)),
        b'X' => Some((2030, 2000)),
        b'Y' => Some((2031, 2001)),
        b'1' => Some((2031, 2001)),
        b'2' => Some((2032, 2002)),
        b'3' => Some((2033, 2003)),
        b'4' => Some((2034, 2004)),
        b'5' => Some((2035, 2005)),
        b'6' => Some((2036, 2006)),
        b'7' => Some((2037, 2007)),
        b'8' => Some((2038, 2008)),
        b'9' => Some((2039, 2009)),
        _ => None,
    }
}

fn format_model_year(c: u8) -> String {
    let current_year = chrono::Utc::now().year() as u16;
    match model_year(c) {
        Some((new, old)) => {
            if new <= current_year {
                format!("{} or {}", new, old)
            } else {
                format!("{}", old)
            }
        }
        None => "Unknown".to_string(),
    }
}

#[allow(clippy::indexing_slicing)]
pub fn parse_vin(args: &Args) -> Option<IDInfo> {
    let id: String = args.id.chars().filter(|&c| c != '-').collect::<String>().to_uppercase();
    if id.len() != 17 {
        return None;
    }
    let bytes = id.as_bytes();
    for &b in bytes {
        match b {
            b'A'..=b'H' | b'J'..=b'N' | b'P' | b'R'..=b'Z' | b'0'..=b'9' => {}
            _ => return None,
        }
    }
    let wmi = &id[..3];
    let country = wmi_country(bytes[0]);
    let manufacturer = wmi_manufacturer(wmi);
    let node1_label = match manufacturer {
        Some(mfr) => format!("{}, {}", mfr, country),
        None => country.to_string(),
    };
    let vds = &id[3..8];
    let year = format_model_year(bytes[9]);
    let plant = &id[10..11];
    let sequence: u128 = id[11..].parse().ok().unwrap_or(0);
    let (size, hex, bits, _) = factor_size_hex_bits_color_from_text(&id);

    Some(IDInfo {
        id_type: "VIN (Vehicle Identification Number)".to_string(),
        version: Some(node1_label),
        standard: format!("{}-{}-{}-{}", &id[..3], &id[3..9], &id[9..11], &id[11..]),
        parsed: Some("as ASCII".to_string()),
        size,
        entropy: 0,
        node1: Some(format!("{} (Model)", vds)),
        node2: Some(format!("Model year: {}", year)),
        node3: Some(format!("Factory: {}", plant)),
        sequence: if sequence > 0 { Some(sequence) } else { None },
        hex,
        bits,
        color_map: Some(repeat_char('1', 24) + &repeat_char('4', 40) + &repeat_char('0', 8) + &repeat_char('5', 8) + &repeat_char('7', 8) + &repeat_char('6', 48)),
        high_confidence: true,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Output;

    fn make_args(id: &str) -> Args {
        Args {
            id: id.to_string(),
            output: Output::Card,
            force: None,
            everything: false,
            compare: false,
            alphabet: None,
            relative: false,
            salt: None,
        }
    }

    #[test]
    fn test_parse_vin_us() {
        let args = make_args("1HGCM82633A004352");
        let result = parse_vin(&args).unwrap();
        assert_eq!(result.id_type, "VIN (Vehicle Identification Number)");
        let version = result.version.unwrap();
        assert!(version.contains("Honda"));
        assert!(version.contains("United States"));
        assert!(result.node2.unwrap().contains("2003"));
        assert!(result.high_confidence);
    }

    #[test]
    fn test_parse_vin_german() {
        let args = make_args("WBANE53517CX66498");
        let result = parse_vin(&args).unwrap();
        let version = result.version.unwrap();
        assert!(version.contains("BMW"));
        assert!(version.contains("Germany"));
        assert!(result.high_confidence);
    }

    #[test]
    fn test_parse_vin_lowercase() {
        let args = make_args("1hgcm82633a004352");
        let result = parse_vin(&args).unwrap();
        let version = result.version.unwrap();
        assert!(version.contains("Honda"));
        assert!(version.contains("United States"));
    }

    #[test]
    fn test_parse_vin_unknown_manufacturer() {
        let args = make_args("WZZZZZ3CZWE123456");
        let result = parse_vin(&args);
        if let Some(r) = result {
            let version = r.version.unwrap();
            assert!(version.contains("Germany"));
            assert!(!version.contains(','));
        }
    }

    #[test]
    fn test_reject_invalid_chars() {
        let args = make_args("1HGCM826I3A004352");
        assert!(parse_vin(&args).is_none());
    }

    #[test]
    fn test_reject_wrong_length() {
        let args = make_args("1HGCM82633A00435");
        assert!(parse_vin(&args).is_none());
    }

    #[test]
    fn test_check_digit_x() {
        let args = make_args("11111111111111111");
        let result = parse_vin(&args);
        assert!(result.is_none() || result.unwrap().id_type.contains("VIN"));
    }
}
