use crate::coord::Coord;

use super::ICoordsToNumber;

pub struct CoordsToNumber;

impl CoordsToNumber {
    pub fn new() -> Self {
        CoordsToNumber {}
    }
}

impl ICoordsToNumber for CoordsToNumber {
    fn process(&self, schem: &crate::engine::EngineSchematic, coords: &Vec<Coord>) -> usize {
        let mut digits  = coords.len() as u32;
        let mut output = 0;
        for coord in coords.iter() {
            if let Some(row) = schem.grid.get(coord.row) {
                if let Some(col) = row.get(coord.col) {
                    if let Some(num) = col.to_digit(10) {
                        let base: u32 = 10;
                        output += num * base.pow(digits - 1);
                        digits -= 1;
                    }
                }
            }
        }
        output as usize
    }
}