use crate::{coord::Coord, engine::EngineSchematic};

pub trait IFindNumberCoords {
    fn find(&self, schem: &EngineSchematic) -> Vec<Coord>;
}