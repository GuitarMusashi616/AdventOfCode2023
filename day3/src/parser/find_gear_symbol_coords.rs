use crate::{coord::Coord, engine::EngineSchematic};

use super::IFindSymbolCoords;

pub struct FindGearSymbolCoords;

impl FindGearSymbolCoords {
    pub fn new() -> Self {
        FindGearSymbolCoords{}
    }
}

impl IFindSymbolCoords for FindGearSymbolCoords {
    fn find(&self, schem: &EngineSchematic) -> Vec<Coord> {
        let mut res = Vec::new();
        for i in 0..schem.grid.len() {
            if let Some(row) = schem.grid.get(i) {
                for j in 0..row.len() {
                    if let Some(col) = row.get(j) {
                        if *col == '*' {
                            res.push((i, j).into());
                        }
                    }
                }
            }
        }
        res
    }
}