use crate::coord::Coord;

pub trait IFindNumberCoords {
    fn find(&self) -> Box<dyn Iterator<Item = Coord>>;
}