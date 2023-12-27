use crate::coord::Coord;

pub trait IPartNumberFilter {
    fn filter(&self, part_numbers: &Vec<Vec<Coord>>, symbols: &Vec<Coord>) -> Vec<Vec<Coord>>;
}