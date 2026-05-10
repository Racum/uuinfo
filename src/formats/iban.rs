use crate::schema::{Args, IDInfo};
use crate::utils::{factor_size_hex_bits_color_from_text, repeat_char};

#[rustfmt::skip]
fn iban_length(country: &str) -> Option<u8> {
    match country {
        "NO" => Some(15),
        "BE" => Some(16),
        "DK" | "FI" | "FK" | "FO" | "GL" | "NL" | "SD" => Some(18),
        "MK" | "SI" => Some(19),
        "AT" | "BA" | "EE" | "KZ" | "LT" | "LU" | "MN" | "XK" => Some(20),
        "CH" | "HR" | "LI" | "LV" => Some(21),
        "BG" | "BH" | "CR" | "DE" | "GB" | "GE" | "IE" | "ME" | "RS" | "VA" => Some(22),
        "AE" | "GI" | "IL" | "IQ" | "OM" | "SO" | "TL" => Some(23),
        "AD" | "CZ" | "DZ" | "ES" | "MD" | "PK" | "RO" | "SA" | "SE" | "SK" | "TN" | "VG" => Some(24),
        "AO" | "CV" | "GW" | "LY" | "MZ" | "PT" | "ST" => Some(25),
        "IR" | "IS" | "TR" => Some(26),
        "BI" | "CF" | "CG" | "CM" | "DJ" | "FR" | "GA" | "GQ" | "GR" | "IT" | "KM" | "MC" | "MG" | "MR" | "SM" | "TD" => Some(27),
        "AL" | "AZ" | "BF" | "BJ" | "BY" | "CI" | "CY" | "DO" | "GT" | "HN" | "HU" | "LB" | "MA" | "ML" | "NE" | "NI" | "PL" | "SN" | "SV" | "TG" => Some(28),
        "BR" | "EG" | "PS" | "QA" | "UA" => Some(29),
        "JO" | "KW" | "MU" | "YE" => Some(30),
        "MT" | "SC" => Some(31),
        "LC" => Some(32),
        "RU" => Some(33),
        _ => None,
    }
}

fn validate_checksum(iban: &str) -> bool {
    let rearranged = format!("{}{}", &iban[4..], &iban[..4]);
    let numeric: String = rearranged
        .chars()
        .map(|c| if c.is_ascii_digit() { c.to_string() } else { ((c as u32) - ('A' as u32) + 10).to_string() })
        .collect();
    mod97(&numeric) == 1
}

fn mod97(digits: &str) -> u32 {
    let mut remainder: u32 = 0;
    for c in digits.chars() {
        let digit = c.to_digit(10).unwrap_or(0);
        remainder = (remainder * 10 + digit) % 97;
    }
    remainder
}

fn country_name(code: &str) -> &str {
    match code {
        "AL" => "Albania",
        "AD" => "Andorra",
        "AE" => "United Arab Emirates",
        "AO" => "Angola",
        "AT" => "Austria",
        "AZ" => "Azerbaijan",
        "BA" => "Bosnia and Herzegovina",
        "BE" => "Belgium",
        "BF" => "Burkina Faso",
        "BG" => "Bulgaria",
        "BH" => "Bahrain",
        "BI" => "Burundi",
        "BJ" => "Benin",
        "BR" => "Brazil",
        "BY" => "Belarus",
        "CF" => "Central African Republic",
        "CG" => "Congo",
        "CH" => "Switzerland",
        "CI" => "Ivory Coast",
        "CM" => "Cameroon",
        "CR" => "Costa Rica",
        "CV" => "Cape Verde",
        "CY" => "Cyprus",
        "CZ" => "Czech Republic",
        "DE" => "Germany",
        "DJ" => "Djibouti",
        "DK" => "Denmark",
        "DO" => "Dominican Republic",
        "DZ" => "Algeria",
        "EE" => "Estonia",
        "EG" => "Egypt",
        "ES" => "Spain",
        "FI" => "Finland",
        "FK" => "Falkland Islands",
        "FO" => "Faroe Islands",
        "FR" => "France",
        "GA" => "Gabon",
        "GB" => "United Kingdom",
        "GE" => "Georgia",
        "GI" => "Gibraltar",
        "GL" => "Greenland",
        "GQ" => "Equatorial Guinea",
        "GR" => "Greece",
        "GT" => "Guatemala",
        "GW" => "Guinea-Bissau",
        "HN" => "Honduras",
        "HR" => "Croatia",
        "HU" => "Hungary",
        "IE" => "Ireland",
        "IL" => "Israel",
        "IQ" => "Iraq",
        "IR" => "Iran",
        "IS" => "Iceland",
        "IT" => "Italy",
        "JO" => "Jordan",
        "KM" => "Comoros",
        "KW" => "Kuwait",
        "KZ" => "Kazakhstan",
        "LB" => "Lebanon",
        "LC" => "Saint Lucia",
        "LI" => "Liechtenstein",
        "LT" => "Lithuania",
        "LU" => "Luxembourg",
        "LV" => "Latvia",
        "LY" => "Libya",
        "MA" => "Morocco",
        "MC" => "Monaco",
        "MD" => "Moldova",
        "ME" => "Montenegro",
        "MG" => "Madagascar",
        "MK" => "North Macedonia",
        "ML" => "Mali",
        "MN" => "Mongolia",
        "MR" => "Mauritania",
        "MT" => "Malta",
        "MU" => "Mauritius",
        "MZ" => "Mozambique",
        "NE" => "Niger",
        "NI" => "Nicaragua",
        "NL" => "Netherlands",
        "NO" => "Norway",
        "OM" => "Oman",
        "PK" => "Pakistan",
        "PL" => "Poland",
        "PS" => "Palestine",
        "PT" => "Portugal",
        "QA" => "Qatar",
        "RO" => "Romania",
        "RS" => "Serbia",
        "RU" => "Russia",
        "SA" => "Saudi Arabia",
        "SC" => "Seychelles",
        "SD" => "Sudan",
        "SE" => "Sweden",
        "SI" => "Slovenia",
        "SK" => "Slovakia",
        "SM" => "San Marino",
        "SN" => "Senegal",
        "SO" => "Somalia",
        "ST" => "São Tomé and Príncipe",
        "SV" => "El Salvador",
        "TD" => "Chad",
        "TG" => "Togo",
        "TL" => "Timor-Leste",
        "TN" => "Tunisia",
        "TR" => "Turkey",
        "UA" => "Ukraine",
        "VA" => "Vatican City",
        "VG" => "British Virgin Islands",
        "XK" => "Kosovo",
        "YE" => "Yemen",
        _ => "Unknown",
    }
}

pub fn parse_iban(args: &Args) -> Option<IDInfo> {
    let normalized = args.id.replace([' ', '-'], "").to_uppercase();
    let id_len = normalized.len();
    if !(15..=34).contains(&id_len) {
        return None;
    }
    if !normalized[..2].chars().all(|c| c.is_ascii_uppercase()) {
        return None;
    }
    if !normalized[2..4].chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if !normalized[4..].chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }
    let country_code = &normalized[..2];
    let expected_len = iban_length(country_code)?;
    if id_len != expected_len as usize {
        return None;
    }
    let valida_checksum = validate_checksum(&normalized);
    let check_digits = &normalized[2..4];
    let bban = &normalized[4..];
    let (size, hex, bits, _) = factor_size_hex_bits_color_from_text(&normalized);
    let country_bits = 16;
    let check_bits = 16;
    let bban_bits = (id_len - 4) * 8;

    Some(IDInfo {
        id_type: "IBAN".to_string(),
        version: Some(format!("{} ({})", country_code, country_name(country_code))),
        standard: format_iban(&normalized),
        parsed: Some("as ASCII".to_string()),
        size,
        entropy: 0,
        node1: if valida_checksum {
            format!("{} (Valid Checksum)", check_digits).into()
        } else {
            format!("{} (Invalid Checksum)", check_digits).into()
        },
        node2: Some(format!("{} (BBAN)", bban)),
        hex,
        bits,
        color_map: Some(repeat_char('1', country_bits) + &repeat_char('4', check_bits) + &repeat_char('5', bban_bits)),
        high_confidence: valida_checksum,
        ..Default::default()
    })
}

fn format_iban(iban: &str) -> String {
    iban.chars().collect::<Vec<_>>().chunks(4).map(|chunk| chunk.iter().collect::<String>()).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Args;

    fn args(id: &str) -> Args {
        Args {
            id: id.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_norway_shortest() {
        let result = parse_iban(&args("NO9386011117947")).unwrap();
        assert_eq!(result.id_type, "IBAN");
        assert_eq!(result.version.unwrap(), "NO (Norway)");
        assert_eq!(result.standard, "NO93 8601 1117 947");
        assert!(result.high_confidence);
    }

    #[test]
    fn test_russia_longest() {
        let result = parse_iban(&args("RU0204452560040702810412345678901")).unwrap();
        assert_eq!(result.id_type, "IBAN");
        assert_eq!(result.version.unwrap(), "RU (Russia)");
        assert_eq!(result.standard, "RU02 0445 2560 0407 0281 0412 3456 7890 1");
        assert!(result.high_confidence);
    }

    #[test]
    fn test_gb_alpha_bban() {
        let result = parse_iban(&args("GB29NWBK60161331926819")).unwrap();
        assert_eq!(result.id_type, "IBAN");
        assert_eq!(result.version.unwrap(), "GB (United Kingdom)");
        assert_eq!(result.node2.unwrap(), "NWBK60161331926819 (BBAN)");
        assert!(result.high_confidence);
    }

    #[test]
    fn test_invalid_checksum() {
        let result = parse_iban(&args("GB00NWBK60161331926819")).unwrap();
        assert_eq!(result.id_type, "IBAN");
        assert!(!result.high_confidence);
        assert!(result.node1.unwrap().contains("Invalid Checksum"));
    }

    #[test]
    fn test_wrong_length_rejected() {
        assert!(parse_iban(&args("GB29NWBK6016133192681")).is_none());
    }

    #[test]
    fn test_unknown_country_rejected() {
        assert!(parse_iban(&args("ZZ99123456789012345")).is_none());
    }

    #[test]
    fn test_spaces_normalized() {
        let result = parse_iban(&args("GB29 NWBK 6016 1331 9268 19")).unwrap();
        assert_eq!(result.version.unwrap(), "GB (United Kingdom)");
        assert!(result.high_confidence);
    }
}
