use super::Coord;

pub trait ICoordAdj {
    fn is_adj(&self, coord1: &Coord, coord2: &Coord) -> bool;
}