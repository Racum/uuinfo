use crate::formats::isbn::parse_isbn13;
use crate::id_format::pick_first_valid;
use crate::schema::{Args, IDInfo};
use crate::utils::{factor_size_hex_bits_color_from_text, repeat_char};

fn gtin_check_digit(digits: &[u8]) -> u8 {
    let len = digits.len();
    let sum: u32 = digits.iter().enumerate().map(|(i, &d)| {
        let weight = if (len - i) % 2 == 0 { 1 } else { 3 };
        d as u32 * weight
    }).sum();
    ((10 - (sum % 10)) % 10) as u8
}

fn strip_dashes(s: &str) -> String {
    s.chars().filter(|&c| c != '-').collect()
}

fn parse_digits(s: &str) -> Option<Vec<u8>> {
    s.bytes().map(|b| {
        if b.is_ascii_digit() { Some(b - b'0') } else { None }
    }).collect()
}

fn validate_check_digit(digits: &[u8]) -> bool {
    let len = digits.len();
    if len < 2 {
        return false;
    }
    let expected = gtin_check_digit(&digits[..len - 1]);
    digits[len - 1] == expected
}

pub fn parse_ean8(args: &Args) -> Option<IDInfo> {
    let clean = strip_dashes(&args.id);
    if clean.len() != 8 {
        return None;
    }
    let digits = parse_digits(&clean)?;
    if !validate_check_digit(&digits) {
        return None;
    }
    let (_, hex, bits, _) = factor_size_hex_bits_color_from_text(&clean);
    let size = (clean.len() * 8) as u16;

    Some(IDInfo {
        id_type: "Commerce Barcode".to_string(),
        version: Some("EAN-8 (GTIN-8)".to_string()),
        standard: format!("{}-{}", &clean[..4], &clean[4..]),
        integer: clean.parse::<u128>().ok(),
        parsed: Some("as ASCII".to_string()),
        size,
        entropy: 0,
        hex,
        bits,
        color_map: Some(repeat_char('2', 56) + &repeat_char('0', 8)),
        high_confidence: true,
        ..Default::default()
    })
}

pub fn parse_upca(args: &Args) -> Option<IDInfo> {
    let clean = strip_dashes(&args.id);
    if clean.len() != 12 {
        return None;
    }
    let digits = parse_digits(&clean)?;
    if !validate_check_digit(&digits) {
        return None;
    }
    let number_system = match digits[0] {
        0 | 1 | 6..=8 => "Regular UPC",
        2 => "Variable weight",
        3 => "Drug/pharmaceutical",
        4 => "In-store use",
        5 | 9 => "Coupon",
        _ => unreachable!(),
    };
    let (_, hex, bits, _) = factor_size_hex_bits_color_from_text(&clean);
    let size = (clean.len() * 8) as u16;

    Some(IDInfo {
        id_type: "Commerce Barcode".to_string(),
        version: Some("UPC-A (GTIN-12)".to_string()),
        standard: format!("{}-{}-{}", &clean[..1], &clean[1..6], &clean[6..]),
        integer: clean.parse::<u128>().ok(),
        parsed: Some("as ASCII".to_string()),
        size,
        entropy: 0,
        node1: Some(format!("{} (Number system: {})", digits[0], number_system)),
        hex,
        bits,
        color_map: Some(repeat_char('4', 8) + &repeat_char('2', 80) + &repeat_char('0', 8)),
        high_confidence: true,
        ..Default::default()
    })
}

#[rustfmt::skip]
fn gs1_prefix_country(p: u16) -> &'static str {
    match p {
        0..=19 | 60..=99 => "US & Canada",
        20..=29 | 40..=49 | 200..=299 => "Variable weight (store)",
        30..=39 => "US drugs",
        50..=59 => "Coupons",
        100..=139 => "US",
        300..=379 => "France & Monaco",
        380 => "Bulgaria",
        383 => "Slovenia",
        385 => "Croatia",
        387 => "Bosnia and Herzegovina",
        389 => "Montenegro",
        390 => "Kosovo",
        400..=440 => "Germany",
        450..=459 | 490..=499 => "Japan",
        460..=469 => "Russia",
        470 => "Kyrgyzstan",
        471 => "Taiwan",
        474 => "Estonia",
        475 => "Latvia",
        476 => "Azerbaijan",
        477 => "Lithuania",
        478 => "Uzbekistan",
        479 => "Sri Lanka",
        480 => "Philippines",
        481 => "Belarus",
        482 => "Ukraine",
        484 => "Moldova",
        485 => "Armenia",
        486 => "Georgia",
        487 => "Kazakhstan",
        488 => "Tajikistan",
        489 => "Hong Kong",
        500..=509 => "United Kingdom",
        520..=521 => "Greece",
        528 => "Lebanon",
        529 => "Cyprus",
        530 => "Albania",
        531 => "North Macedonia",
        535 => "Malta",
        539 => "Ireland",
        540..=549 => "Belgium & Luxembourg",
        560 => "Portugal",
        569 => "Iceland",
        570..=579 => "Denmark, Faroe Islands, Greenland",
        590 => "Poland",
        594 => "Romania",
        599 => "Hungary",
        600..=601 => "South Africa",
        603 => "Ghana",
        604 => "Senegal",
        608 => "Bahrain",
        609 => "Mauritius",
        611 => "Morocco",
        613 => "Algeria",
        615 => "Nigeria",
        616 => "Kenya",
        618 => "Ivory Coast",
        619 => "Tunisia",
        620 => "Tanzania",
        621 => "Syria",
        622 => "Egypt",
        624 => "Libya",
        625 => "Jordan",
        626 => "Iran",
        627 => "Kuwait",
        628 => "Saudi Arabia",
        629 => "United Arab Emirates",
        640..=649 => "Finland",
        690..=699 => "China",
        700..=709 => "Norway",
        729 => "Israel",
        730..=739 => "Sweden",
        740 => "Guatemala",
        741 => "El Salvador",
        742 => "Honduras",
        743 => "Nicaragua",
        744 => "Costa Rica",
        745 => "Panama",
        746 => "Dominican Republic",
        750 => "Mexico",
        754..=755 => "Canada",
        759 => "Venezuela",
        760..=769 => "Switzerland & Liechtenstein",
        770..=771 => "Colombia",
        773 => "Uruguay",
        775 => "Peru",
        777 => "Bolivia",
        778..=779 => "Argentina",
        780 => "Chile",
        784 => "Paraguay",
        786 => "Ecuador",
        789..=790 => "Brazil",
        800..=839 => "Italy, San Marino, Vatican",
        840..=849 => "Spain & Andorra",
        850 => "Cuba",
        858 => "Slovakia",
        859 => "Czech Republic",
        860 => "Serbia",
        865 => "Mongolia",
        867 => "North Korea",
        868..=869 => "Turkey",
        870..=879 => "Netherlands",
        880 => "South Korea",
        883 => "Myanmar",
        884 => "Cambodia",
        885 => "Thailand",
        888 => "Singapore",
        890 => "India",
        893 => "Vietnam",
        896 => "Pakistan",
        899 => "Indonesia",
        900..=919 => "Austria",
        930..=939 => "Australia",
        940..=949 => "New Zealand",
        950 => "GS1 Global Office",
        955 => "Malaysia",
        958 => "Macau",
        977 => "Serial publications - ISSN",
        978..=979 => "Books and publications - ISBN",
        _ => "Unknown",
    }
}

pub fn parse_ean13(args: &Args) -> Option<IDInfo> {
    let clean = strip_dashes(&args.id);
    if clean.len() != 13 {
        return None;
    }
    let digits = parse_digits(&clean)?;
    if !validate_check_digit(&digits) {
        return None;
    }
    let prefix_str = &clean[..3];
    let prefix_num: u16 = prefix_str.parse().ok()?;
    if matches!(prefix_num, 978..=979) {
        let isbn_args = Args { id: clean, ..args.clone() };
        return parse_isbn13(&isbn_args);
    }
    let country = gs1_prefix_country(prefix_num);
    let (_, hex, bits, _) = factor_size_hex_bits_color_from_text(&clean);
    let size = (clean.len() * 8) as u16;

    Some(IDInfo {
        id_type: "Commerce Barcode".to_string(),
        version: Some("EAN-13 (GTIN-13)".to_string()),
        standard: format!("{}-{}-{}", &clean[..1], &clean[1..7], &clean[7..]),
        integer: clean.parse::<u128>().ok(),
        parsed: Some("as ASCII".to_string()),
        size,
        entropy: 0,
        node1: Some(format!("{} ({})", prefix_str, country)),
        hex,
        bits,
        color_map: Some(repeat_char('4', 24) + &repeat_char('2', 72) + &repeat_char('0', 8)),
        high_confidence: true,
        ..Default::default()
    })
}

pub fn parse_gtin14(args: &Args) -> Option<IDInfo> {
    let clean = strip_dashes(&args.id);
    if clean.len() != 14 {
        return None;
    }
    let digits = parse_digits(&clean)?;
    if !validate_check_digit(&digits) {
        return None;
    }
    let packaging = match digits[0] {
        0 => "consumer unit",
        1..=8 => "grouping/packaging level",
        9 => "variable measure",
        _ => return None,
    };
    let prefix_str = &clean[1..4];
    let prefix_num: u16 = prefix_str.parse().ok()?;
    let country = gs1_prefix_country(prefix_num);
    let (_, hex, bits, _) = factor_size_hex_bits_color_from_text(&clean);
    let size = (clean.len() * 8) as u16;

    Some(IDInfo {
        id_type: "Commerce Barcode".to_string(),
        version: Some(format!("GTIN-14, {}", packaging)),
        standard: format!("{}-{}-{}-{}", &clean[..1], &clean[1..4], &clean[4..8], &clean[8..]),
        integer: clean.parse::<u128>().ok(),
        parsed: Some("as ASCII".to_string()),
        size,
        entropy: 0,
        node1: Some(format!("{} ({})", prefix_str, country)),
        hex,
        bits,
        color_map: Some(repeat_char('1', 8) + &repeat_char('4', 24) + &repeat_char('2', 72) + &repeat_char('0', 8)),
        high_confidence: true,
        ..Default::default()
    })
}

pub fn parse_commerce(args: &Args) -> Option<IDInfo> {
    pick_first_valid(args, &[parse_ean13, parse_upca, parse_ean8, parse_gtin14])
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
    fn test_parse_ean13() {
        let args = make_args("5901234123457");
        let result = parse_ean13(&args).unwrap();
        assert_eq!(result.id_type, "Commerce Barcode");
        assert_eq!(result.version.unwrap(), "EAN-13 (GTIN-13)");
        assert!(result.node1.unwrap().contains("Poland"));
    }

    #[test]
    fn test_parse_ean13_isbn() {
        let args = make_args("9780553382570");
        let result = parse_ean13(&args).unwrap();
        assert_eq!(result.id_type, "ISBN-13");
    }

    #[test]
    fn test_parse_ean13_with_dashes() {
        let args = make_args("590-1234-123457");
        let result = parse_ean13(&args).unwrap();
        assert_eq!(result.version.unwrap(), "EAN-13 (GTIN-13)");
    }

    #[test]
    fn test_parse_upca() {
        let args = make_args("042100005264");
        let result = parse_upca(&args).unwrap();
        assert_eq!(result.id_type, "Commerce Barcode");
        assert_eq!(result.version.unwrap(), "UPC-A (GTIN-12)");
        assert!(result.node1.unwrap().contains("Regular UPC"));
    }

    #[test]
    fn test_parse_upca_with_dashes() {
        let args = make_args("0-42100-00526-4");
        let result = parse_upca(&args).unwrap();
        assert_eq!(result.version.unwrap(), "UPC-A (GTIN-12)");
    }

    #[test]
    fn test_parse_ean8() {
        let args = make_args("96385074");
        let result = parse_ean8(&args).unwrap();
        assert_eq!(result.id_type, "Commerce Barcode");
        assert_eq!(result.version.unwrap(), "EAN-8 (GTIN-8)");
    }

    #[test]
    fn test_parse_gtin14() {
        let args = make_args("10614141000415");
        let result = parse_gtin14(&args).unwrap();
        assert_eq!(result.id_type, "Commerce Barcode");
        assert_eq!(result.version.unwrap(), "GTIN-14, grouping/packaging level");
        assert!(result.node1.unwrap().contains("US & Canada"));
    }

    #[test]
    fn test_invalid_check_digit() {
        let args = make_args("5901234123450");
        assert!(parse_ean13(&args).is_none());
    }

    #[test]
    fn test_non_numeric() {
        let args = make_args("590123412345A");
        assert!(parse_ean13(&args).is_none());
    }

    #[test]
    fn test_commerce_dispatcher() {
        let args = make_args("5901234123457");
        let result = parse_commerce(&args).unwrap();
        assert_eq!(result.version.unwrap(), "EAN-13 (GTIN-13)");
    }
}
