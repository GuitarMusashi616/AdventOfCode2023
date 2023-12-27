use crate::{engine::EngineSchematic, coord::Coord};

pub trait IFindPartNumberCoords {
    fn find(&self, schem: &EngineSchematic) -> Vec<Vec<Coord>>;
}