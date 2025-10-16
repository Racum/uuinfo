use h3ron::{H3Cell, Index, ToCoordinate};
use std::fmt::Write;
use std::str::FromStr;

use crate::schema::{Args, IDInfo};

pub fn parse_h3(args: &Args) -> Option<IDInfo> {
    let cell = H3Cell::from_str(&args.id).ok()?;
    if !cell.is_valid() {
        return None;
    }
    let center = cell.to_coordinate().ok()?;
    let id_int: u64 = cell.h3index();
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
        node1: Some(format!("Resolution: {}, base cell: {} ({})", cell.resolution(), cell.get_base_cell_number(), shape)),
        node2: Some(format!("Center (lon, lat): {:.6}, {:.6}", center.x, center.y)),
        hex: Some(hex::encode(id_int.to_be_bytes())),
        bits: Some(id_int.to_be_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("0111100044444444444555555555555555555555555555555555555555555555".to_string()),
        high_confidence: true,
        ..Default::default()
    })
}
