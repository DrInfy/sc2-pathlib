use pathfinding::prelude::absdiff;
use std::cmp::min;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub usize, pub usize);
//static SQRT2: f32 = 1.4142135623730950488016887242097;
pub static SQRT2: usize = 14142;
pub static MULT: usize = 10000;
pub static MULTF64: f64 = 10000.0;

impl Pos {
    pub fn manhattan_distance(&self, other: &Pos) -> usize {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) * MULT
    }

    pub fn euclidean_distance(&self, other: &Pos) -> usize {
        ((((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)) as f64).sqrt() * MULTF64) as usize
    }

    pub fn quick_distance(&self, other: &Pos) -> usize {
        let xd = absdiff(self.0, other.0);
        let yd = absdiff(self.1, other.1);
        let diag = min(xd, yd);
        (xd + yd - diag - diag) * MULT + diag * SQRT2
    }

    pub fn successors(&self, grid: &Vec<Vec<usize>>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut arr = Vec::<(Pos, usize)>::with_capacity(8);
        //let arr = Vec<(Pos, f32)>();

        let mut val_left: usize = 0;
        let mut val_down: usize = 0;
        let mut val_right: usize = 0;
        let mut val_up: usize = 0;

        if x > 0 {
            val_left =  grid[x - 1][y];
        }

        if y > 0 {
            val_down = grid[x][y - 1];
        }

        if x + 1 < grid.len() {
            val_right = grid[x + 1][y];
        }

        if y + 1 < grid[0].len() {
            val_up = grid[x][y + 1];
        }

        if val_left > 0 {
            arr.push((Pos(x - 1, y), val_left * MULT));

            if val_down > 0 {
                let diag_val = grid[x - 1][y - 1];

                if diag_val > 0 {
                    arr.push((Pos(x - 1, y - 1), diag_val * SQRT2));
                }
            }

            if val_up > 0 {
                let diag_val = grid[x - 1][y + 1];

                if diag_val > 0 {
                    arr.push((Pos(x - 1, y + 1), diag_val * SQRT2));
                }
            }
        }

        if val_right > 0 {
            arr.push((Pos(x + 1, y), val_right * MULT));

            if val_down > 0 {
                let diag_val = grid[x + 1][y - 1];

                if diag_val > 0 {
                    arr.push((Pos(x + 1, y - 1), diag_val * SQRT2));
                }
            }

            if val_up > 0 {
                let diag_val = grid[x + 1][y + 1];

                if diag_val > 0 {
                    arr.push((Pos(x + 1, y + 1), diag_val * SQRT2));
                }
            }
        }

        if val_up > 0 {
            arr.push((Pos(x, y + 1), val_up * MULT));
        }

        if val_down > 0 {
            arr.push((Pos(x, y - 1), val_down * MULT));
        }

        arr
    }

    pub fn successors_no_influence(&self, grid: &Vec<Vec<usize>>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut arr = Vec::<(Pos, usize)>::with_capacity(8);
        //let arr = Vec<(Pos, f32)>();

        let mut val_left: usize = 0;
        let mut val_down: usize = 0;
        let mut val_right: usize = 0;
        let mut val_up: usize = 0;

        if x > 0 {
            val_left =  grid[x - 1][y];
        }

        if y > 0 {
            val_down = grid[x][y - 1];
        }

        if x + 1 < grid.len() {
            val_right = grid[x + 1][y];
        }

        if y + 1 < grid[0].len() {
            val_up = grid[x][y + 1];
        }

        if val_left > 0 {
            arr.push((Pos(x - 1, y), MULT));

            if val_down > 0 {
                let diag_val = grid[x - 1][y - 1];

                if diag_val > 0 {
                    arr.push((Pos(x - 1, y - 1), SQRT2));
                }
            }

            if val_up > 0 {
                let diag_val = grid[x - 1][y + 1];

                if diag_val > 0 {
                    arr.push((Pos(x - 1, y + 1), SQRT2));
                }
            }
        }

        if val_right > 0 {
            arr.push((Pos(x + 1, y), val_right * MULT));

            if val_down > 0 {
                let diag_val = grid[x + 1][y - 1];

                if diag_val > 0 {
                    arr.push((Pos(x + 1, y - 1), SQRT2));
                }
            }

            if val_up > 0 {
                let diag_val = grid[x + 1][y + 1];

                if diag_val > 0 {
                    arr.push((Pos(x + 1, y + 1), SQRT2));
                }
            }
        }

        if val_up > 0 {
            arr.push((Pos(x, y + 1), MULT));
        }

        if val_down > 0 {
            arr.push((Pos(x, y - 1), MULT));
        }

        arr
    }
}