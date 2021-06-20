use arrayvec::ArrayVec;
use pathfinding::prelude::absdiff;
use crate::path_find::pos::PositionAPI;
use crate::path_find::pos::Pos;

//static SQRT2: f32 = 1.4142135623730950488016887242097;
pub static SQRT2: usize = 14142;
pub static MULT: usize = 10000;
pub static MULTF32: f32 = 10000.0;

//constants
pub static DIAGONAL_MINUS_CARDINAL: usize = 4142;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PosLargeAPI();

impl PositionAPI for PosLargeAPI {
    #[inline]
    fn manhattan_distance(&self, start: &Pos, end: &Pos) -> usize {
        (absdiff(start.0, end.0) + absdiff(start.1, end.1)) * MULT
    }

    #[inline]
    fn euclidean_distance(&self, start: &Pos, end: &Pos) -> usize {
        let a = start.0 - end.0;
        let b = start.1 - end.1;
        let dist2 = a * a + b * b;
        ((dist2 as f32).sqrt() * MULTF32) as usize

        // ((((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)) as f32).sqrt() * MULTF32) as usize
    }

    #[inline]
    fn octile_distance(&self, start: &Pos, end: &Pos) -> usize {
        let dx = absdiff(start.0, end.0);
        let dy = absdiff(start.1, end.1);

        if dx > dy {
            MULT * dx + DIAGONAL_MINUS_CARDINAL * dy
        } else {
            MULT * dy + DIAGONAL_MINUS_CARDINAL * dx
        }
    }

    #[inline]
    fn successors(&self, pos: &Pos, grid: &[Vec<usize>]) -> ArrayVec<(Pos, usize), 8> {
        self.successors_within(pos, grid, ((0, 0), (grid.len(), grid[0].len())))
    }

    #[inline]
    fn successors_within(&self, pos: &Pos, grid: &[Vec<usize>], window: ((usize, usize), (usize, usize))) -> ArrayVec<(Pos, usize), 8> {
        let &Pos(x, y) = pos;
        let mut arr = ArrayVec::<(Pos, usize), 8>::new();
        //let arr = Vec<(Pos, f32)>();

        let mut val_left: bool = false;
        let mut val_down: bool = false;
        let mut val_right: bool = false;
        let mut val_up: bool = false;

        let mut val_left_up: bool = false;
        let mut val_left_down: bool = false;
        let mut val_right_up: bool = false;
        let mut val_right_down: bool = false;

        let ((x0, y0), (x1, y1)) = window;

        if x > x0 {
            val_left = grid[x - 1][y] > 0;
        }

        if y > y0 {
            val_down = grid[x][y - 1] > 0;
        }

        if x + 1 < x1 {
            val_right = grid[x + 1][y] > 0;
        }

        if y + 1 < y1 {
            val_up = grid[x][y + 1] > 0;
        }

        if val_left {
            arr.push((Pos(x - 1, y), MULT));

            if val_down {
                val_left_down = grid[x - 1][y - 1] > 0;
            }

            if val_up {
                val_left_up = grid[x - 1][y + 1] > 0;
            }
        }

        if val_right {
            arr.push((Pos(x + 1, y), MULT));

            if val_down {
                val_right_down = grid[x + 1][y - 1] > 0;
            }

            if val_up {
                val_right_up = grid[x + 1][y + 1] > 0;
            }
        }

        if val_up && (val_left_up || val_right_up) {
            arr.push((Pos(x, y + 1), MULT));
        }

        if val_down && (val_left_down || val_right_down) {
            arr.push((Pos(x, y - 1), MULT));
        }

        if val_left && (val_left_up || val_left_down) {
            arr.push((Pos(x - 1, y), MULT));
        }

        if val_right && (val_right_up || val_right_down) {
            arr.push((Pos(x + 1, y), MULT));
        }

        if val_left_up {
            arr.push((Pos(x - 1, y + 1), SQRT2));
        }

        if val_left_down {
            arr.push((Pos(x - 1, y - 1), SQRT2));
        }

        if val_right_up {
            arr.push((Pos(x + 1, y + 1), SQRT2));
        }

        if val_right_down {
            arr.push((Pos(x + 1, y - 1), SQRT2));
        }

        arr
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InfluencedPosLargeAPI {
    pub normal_influence: usize
}

impl PositionAPI for InfluencedPosLargeAPI {
    #[inline]
    fn manhattan_distance(&self, start: &Pos, end: &Pos) -> usize {
        (absdiff(start.0, end.0) + absdiff(start.1, end.1)) * MULT * self.normal_influence
    }

    #[inline]
    fn euclidean_distance(&self, start: &Pos, end: &Pos) -> usize {
        let a = (start.0 - end.0) * self.normal_influence;
        let b = (start.1 - end.1) * self.normal_influence;
        let dist2 = a * a + b * b;
        ((dist2 as f32).sqrt() * MULTF32) as usize
    }

    #[inline]
    fn octile_distance(&self, start: &Pos, end: &Pos) -> usize {
        let dx = absdiff(start.0, end.0);
        let dy = absdiff(start.1, end.1);

        if dx > dy {
            (MULT * dx + DIAGONAL_MINUS_CARDINAL * dy) * self.normal_influence
        } else {
            (MULT * dy + DIAGONAL_MINUS_CARDINAL * dx) * self.normal_influence
        }
    }

    #[inline]
    fn successors(&self, pos: &Pos, grid: &[Vec<usize>]) -> ArrayVec<(Pos, usize), 8> {
        self.successors_within(pos, grid, ((0, 0), (grid.len(), grid[0].len())))
    }

    #[inline]
    fn successors_within(&self, pos: &Pos, grid: &[Vec<usize>], window: ((usize, usize), (usize, usize))) -> ArrayVec<(Pos, usize), 8> {
        let &Pos(x, y) = pos;
        let mut arr = ArrayVec::<(Pos, usize), 8>::new();

        let mut val_left: usize = 0;
        let mut val_down: usize = 0;
        let mut val_right: usize = 0;
        let mut val_up: usize = 0;

        let mut val_left_up: usize = 0;
        let mut val_left_down: usize = 0;
        let mut val_right_up: usize = 0;
        let mut val_right_down: usize = 0;

        let x0 = window.0.0;
        let y0 = window.0.1;
        let x1 = window.1.0;
        let y1 = window.1.1;

        if x > x0 {
            val_left = grid[x - 1][y];
        }

        if y > y0 {
            val_down = grid[x][y - 1];
        }

        if x + 1 < x1 {
            val_right = grid[x + 1][y];
        }

        if y + 1 < y1 {
            val_up = grid[x][y + 1];
        }

        if val_left > 0 {
            if val_down > 0 {
                val_left_down = grid[x - 1][y - 1];
            }

            if val_up > 0 {
                val_left_up = grid[x - 1][y + 1];
            }
        }

        if val_right > 0 {
            if val_down > 0 {
                val_right_down = grid[x + 1][y - 1];
            }

            if val_up > 0 {
                val_right_up = grid[x + 1][y + 1];
            }
        }

        if val_up > 0 && (val_left_up > 0 || val_right_up > 0) {
            arr.push((Pos(x, y + 1), val_up * MULT));
        }

        if val_down > 0 && (val_left_down > 0 || val_right_down > 0) {
            arr.push((Pos(x, y - 1), val_down * MULT));
        }

        if val_left > 0 && (val_left_up > 0 || val_left_down > 0) {
            arr.push((Pos(x - 1, y), val_left * MULT));
        }

        if val_right > 0 && (val_right_up > 0 || val_right_down > 0) {
            arr.push((Pos(x + 1, y), val_right * MULT));
        }

        if val_left_up > 0 {
            arr.push((Pos(x - 1, y + 1), val_left_up * SQRT2));
        }

        if val_left_down > 0 {
            arr.push((Pos(x - 1, y - 1), val_left_down * SQRT2));
        }

        if val_right_up > 0 {
            arr.push((Pos(x + 1, y + 1), val_right_up * SQRT2));
        }

        if val_right_down > 0 {
            arr.push((Pos(x + 1, y - 1), val_right_down * SQRT2));
        }

        arr
    }
}
