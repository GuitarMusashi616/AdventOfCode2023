use crate::{coord::Coord, engine::EngineSchematic};

pub trait IFindSymbolCoords {
    fn find(&self, schem: &EngineSchematic) -> Vec<Coord>;
}