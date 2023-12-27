use crate::{coord::Coord, engine::EngineSchematic};

pub trait ICoordsToNumber {
    fn process(&self, schem: &EngineSchematic, coords: &Vec<Coord>) -> usize;
}