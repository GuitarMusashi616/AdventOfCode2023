use crate::{coord::Coord, iterator::GridNumIterator, engine::EngineSchematic};

use super::IFindNumberCoords;

pub struct FindNumberCoords<'a> {
    schem: &'a EngineSchematic,
}

impl<'a> FindNumberCoords<'a> {
    pub fn new(schem: &'a EngineSchematic) -> Self {
        FindNumberCoords {schem}
    }
}

impl<'a> IFindNumberCoords for FindNumberCoords<'a> {
    fn find(&self) -> Box<dyn Iterator<Item = Coord>> {
        return Box::new(GridNumIterator::new(self.schem))
    }
}