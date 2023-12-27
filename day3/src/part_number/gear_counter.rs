use crate::{coord::{Coord, ICoordAdj}, engine::EngineSchematic};

use super::{IGearCounter, ICoordsToNumber};

pub struct GearCounter {
    coord_adj: Box<dyn ICoordAdj>,
    coords_to_num: Box<dyn ICoordsToNumber>,
}

impl GearCounter {
    pub fn new(coord_adj: Box<dyn ICoordAdj>, coords_to_num: Box<dyn ICoordsToNumber>) -> Self {
        GearCounter{coord_adj, coords_to_num}
    }

    fn is_part_number_coords_adj(&self, part_number_coords: &Vec<Coord>, symbol_coord: &Coord) -> bool {
        for part_number_coord in part_number_coords.iter() {
            if self.coord_adj.is_adj(part_number_coord, symbol_coord) {
                return true;
            }
        }
        return false;
    }

    // fn has_adj_coords(&self, part_number_coords: &Vec<Coord>, symbols: &Vec<Coord>) -> bool {
    //     for symbol_coord in symbols.iter() {
    //         let mut adj_count = 0;
    //         if self.is_part_number_coords_adj(part_number_coords, symbol_coord) {
    //             adj_count += 1;
    //         }
    //         if adj_count == 2 {
    //             return true;
    //         }
    //     }
    //     false
    // }

    fn get_adj_coords(&self, part_numbers: &Vec<Vec<Coord>>, gears: &Coord) -> Vec<Vec<Coord>> {
        let mut res = Vec::new();
        for part_number in part_numbers.iter() {
            if self.is_part_number_coords_adj(part_number, gears) {
                res.push(part_number.clone());
            }
        }
        res
    }

}

impl IGearCounter for GearCounter {
    fn count(&self, schem: &EngineSchematic, part_numbers: &Vec<Vec<Coord>>, gears: &Vec<Coord>) -> usize {
        let mut total = 0;
        for gear in gears.iter() {
            let adj_coords = self.get_adj_coords(part_numbers, gear);
            if adj_coords.len() == 2 {
                let product: usize = adj_coords.iter().map(|x| self.coords_to_num.process(schem, x)).product();
                total += product;
            }
        }
        total
    }
}