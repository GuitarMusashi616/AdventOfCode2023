use crate::{coord::Coord, engine::EngineSchematic};

pub trait IGearCounter {
    fn count(&self, schem: &EngineSchematic, part_numbers: &Vec<Vec<Coord>>, gears: &Vec<Coord>) -> usize;
}