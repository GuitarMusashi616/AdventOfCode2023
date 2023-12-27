use crate::{parser::IFindPartNumberCoords, coord::{ICoordAdj, Coord}};

use super::ipart_number_filter::IPartNumberFilter;

pub struct PartNumberFilter {
    coord_adj: Box<dyn ICoordAdj>
}

impl PartNumberFilter {
    pub fn new(coord_adj: Box<dyn ICoordAdj>) -> Self {
        PartNumberFilter{coord_adj}
    }

    fn has_adj_coords(&self, part_number_coords: &Vec<Coord>, symbols: &Vec<Coord>) -> bool {
        for part_number_coord in part_number_coords.iter() {
            for symbol_coord in symbols.iter() {
                if self.coord_adj.is_adj(part_number_coord, symbol_coord) {
                    return true;
                }
            }
        }
        false
    }
}

impl IPartNumberFilter for PartNumberFilter {
    fn filter(&self, part_numbers: &Vec<Vec<Coord>>, symbols: &Vec<Coord>) -> Vec<Vec<Coord>> {
        let mut res = Vec::new();
        for part_number_coords in part_numbers.into_iter() {
            if self.has_adj_coords(&part_number_coords, &symbols) {
                res.push(part_number_coords.clone());
            }
        }
        res
    }
}