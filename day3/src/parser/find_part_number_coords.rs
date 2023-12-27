use crate::{engine::EngineSchematic, coord::Coord};

use super::IFindPartNumberCoords;

pub struct FindPartNumberCoords;

impl FindPartNumberCoords {
    pub fn new() -> Self {
        FindPartNumberCoords {}
    }
}

impl IFindPartNumberCoords for FindPartNumberCoords {
    fn find(&self, schem: &EngineSchematic) -> Vec<Vec<Coord>> {
        
        let mut res = Vec::new();
        let mut row_progress = Vec::new();
        for i in 0..schem.grid.len() {
            if let Some(row) = schem.grid.get(i) {
                for j in 0..row.len() {
                    if let Some(col) = row.get(j) {
                        if col == &'.' {
                            res.push(row_progress.clone());
                            row_progress.clear();
                        }
                        else if col.is_digit(10) {
                            row_progress.push((i, j).into());
                        }
                    }
                }
            }
        }
        return res;
    }
}

