use h3o::{CellIndex, LatLng};
use std::fmt::Write;
use std::str::FromStr;

use crate::schema::{Args, IDInfo};
use crate::utils::repeat_char;

pub fn parse_h3(args: &Args) -> Option<IDInfo> {
    let cell = CellIndex::from_str(&args.id).ok()?;
    let center = LatLng::from(cell);
    let id_int: u64 = u64::from(cell);
    let shape = match cell.is_pentagon() {
        false => "hexagon",
        true => "pentagon",
    };

    Some(IDInfo {
        id_type: "H3 Grid System".to_string(),
        version: Some("H3 Cell (Mode 1)".to_string()),
        standard: cell.to_string(),
        integer: Some(id_int as u128),
        size: 64,
        node1: Some(format!("Resolution: {}, base cell: {} ({})", u8::from(cell.resolution()), u8::from(cell.base_cell()), shape)),
        node2: Some(format!("Center (lon, lat): {:.6}, {:.6}", center.lng(), center.lat())),
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some(repeat_char('0', 1) + &repeat_char('1', 4) + &repeat_char('0', 3) + &repeat_char('4', 11) + &repeat_char('5', 45)),
        high_confidence: true,
        ..Default::default()
    })
}
