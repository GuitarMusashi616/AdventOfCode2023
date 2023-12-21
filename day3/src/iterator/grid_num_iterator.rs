use crate::{engine::EngineSchematic, coord::Coord};

pub struct GridNumIterator<'a> {
    curr_coord: Coord,
    schem: &'a EngineSchematic,
}

impl<'a> GridNumIterator<'a> {
    pub fn new(schem: &'a EngineSchematic) -> Self {
        GridNumIterator{curr_coord: (0, 0).into(), schem}
    }
}

impl<'a> Iterator for GridNumIterator<'a> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_coord.row >= self.schem.grid.len() {
            return None;
        }
        
        let pre_increment = self.curr_coord;

        if self.curr_coord.col < self.schem.grid[self.curr_coord.row].len() {
            self.curr_coord.col += 1;
        } else {
            self.curr_coord.row += 1;
            self.curr_coord.col = 0;
        }

        let curr_char = self.schem.grid[self.curr_coord.row][self.curr_coord.col];
        if curr_char.is_numeric() {
            Some(pre_increment)
        } else {
            None
        }
    }
}
