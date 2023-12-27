use crate::{engine::EngineSchematic, coord::Coord};

use super::IFindSymbolCoords;

pub struct FindSymbolCoords;

impl FindSymbolCoords {
    pub fn new() -> Self {
        FindSymbolCoords{}
    }
}

impl IFindSymbolCoords for FindSymbolCoords {
    fn find(&self, schem: &EngineSchematic) -> Vec<Coord> {
        let mut res = Vec::new();
        for i in 1..schem.grid.len() {
            if let Some(row) = schem.grid.get(i) {
                for j in 1..row.len() {
                    if let Some(col) = row.get(j) {
                        if !col.is_digit(10) && *col != '.' {
                            res.push((i, j).into());
                        }
                    }
                }
            }
        }
        res
    }
}