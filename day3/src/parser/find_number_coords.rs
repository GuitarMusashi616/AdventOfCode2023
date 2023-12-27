use crate::{engine::EngineSchematic, coord::Coord};

use super::IFindNumberCoords;

pub struct FindNumberCoords;

impl FindNumberCoords {
    pub fn new() -> Self {
        FindNumberCoords {}
    }
}

impl IFindNumberCoords for FindNumberCoords {
    fn find(&self, schem: &EngineSchematic) -> Vec<Coord> {
        let mut res = Vec::new();
        for i in 1..schem.grid.len() {
            if let Some(row) = schem.grid.get(i) {
                for j in 1..row.len() {
                    if let Some(col) = row.get(j) {
                        if col.is_digit(10) {
                            res.push((i, j).into());
                        }
                    }
                }
            }
        }
        res
    }
}