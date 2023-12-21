use std::collections::HashSet;

use super::{ICoordAdj, Coord};

pub struct CoordAdj {
    abs_diff_adj: HashSet<Coord>,
}

impl CoordAdj {
    pub fn new() -> Self {
        let coord_set = [(0, 1), (1, 1), (1, 0)];
        let abs_diff_adj = coord_set.into_iter().map(|x| x.into()).collect();
        CoordAdj{abs_diff_adj}
    }

    fn abs_diff(&self, num1: usize, num2: usize) -> usize {
        // return the absolute difference
        if num1 >= num2 {
            return num1 - num2
        }
        num2 - num1
    }
}

impl ICoordAdj for CoordAdj {
    fn is_adj(&self, coord1: &Coord, coord2: &Coord) -> bool {
        let row_diff = self.abs_diff(coord1.row, coord2.row);
        let col_diff = self.abs_diff(coord1.col, coord2.col);
        let abs_diff = (row_diff, col_diff).into();
        self.abs_diff_adj.contains(&abs_diff)
    }
}

#[cfg(test)]
mod tests {
    use crate::coord::ICoordAdj;

    use super::CoordAdj;

    #[test]
    fn adj_coords() {
        let coord1 = (5, 4).into();
        let coord2 = (4, 4).into();
        let coord3 = (6, 4).into();
        let coord4 = (5, 5).into();
        let coord5 = (6, 5).into();
        let coord6 = (4, 3).into();
        let coord7 = (2, 3).into();

        let coord_adj: Box<dyn ICoordAdj> = Box::new(CoordAdj::new());

        let res1 = coord_adj.is_adj(&coord1, &coord2);
        assert!(res1);

        let res2 = coord_adj.is_adj(&coord1, &coord3);
        assert!(res2);

        let res3 = coord_adj.is_adj(&coord1, &coord4);
        assert!(res3);

        let res4 = coord_adj.is_adj(&coord1, &coord5);
        assert!(res4);

        let res5 = coord_adj.is_adj(&coord1, &coord6);
        assert!(res5);

        let res6 = coord_adj.is_adj(&coord1, &coord7);
        assert!(!res6);
    }
}