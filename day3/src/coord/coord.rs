#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord {
    pub row: usize,
    pub col: usize
}

impl From<(usize, usize)> for Coord {
    fn from(tuple: (usize, usize)) -> Self {
        Coord {
            row: tuple.0,
            col: tuple.1,
        }
    }
}