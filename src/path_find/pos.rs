use arrayvec::ArrayVec;
use pathfinding::prelude::absdiff;
//static SQRT2: f32 = 1.4142135623730950488016887242097;
pub static SQRT2: usize = 14142;
pub static MULT: usize = 10000;
pub static MULTF32: f32 = 10000.0;

//constants
pub static DIAGONAL_MINUS_CARDINAL: usize = 4142;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub usize, pub usize);

pub trait PositionAPI {
    fn manhattan_distance(&self, start: &Pos, end: &Pos) -> usize;
    fn euclidean_distance(&self, start: &Pos, end: &Pos) -> usize;
    fn octile_distance(&self, start: &Pos, end: &Pos) -> usize;
    fn successors(&self, pos: &Pos, grid: &[Vec<usize>]) -> ArrayVec<(Pos, usize), 8>;
    fn successors_within(&self,
                         pos: &Pos,
                         grid: &[Vec<usize>],
                         window: ((usize, usize), (usize, usize)))
                         -> ArrayVec<(Pos, usize), 8>;
}

pub struct NormalPosAPI();

impl PositionAPI for NormalPosAPI {
    #[inline]
    fn manhattan_distance(&self, start: &Pos, end: &Pos) -> usize {
        (absdiff(start.0, end.0) + absdiff(start.1, end.1)) * MULT
    }

    #[inline]
    fn euclidean_distance(&self, start: &Pos, end: &Pos) -> usize {
        let a = start.0 as f32 - end.0 as f32;
        let b = start.1 as f32 - end.1 as f32;
        let dist2 = a * a + b * b;
        (dist2.sqrt() * MULTF32) as usize
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
    fn successors_within(&self,
                         pos: &Pos,
                         grid: &[Vec<usize>],
                         window: ((usize, usize), (usize, usize)))
                         -> ArrayVec<(Pos, usize), 8> {
        let &Pos(x, y) = pos;
        let mut arr = ArrayVec::<(Pos, usize), 8>::new();
        //let arr = Vec<(Pos, f32)>();

        let mut val_left: bool = false;
        let mut val_down: bool = false;
        let mut val_right: bool = false;
        let mut val_up: bool = false;

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
                let diag_val = grid[x - 1][y - 1] > 0;

                if diag_val {
                    arr.push((Pos(x - 1, y - 1), SQRT2));
                }
            }

            if val_up {
                let diag_val = grid[x - 1][y + 1] > 0;

                if diag_val {
                    arr.push((Pos(x - 1, y + 1), SQRT2));
                }
            }
        }

        if val_right {
            arr.push((Pos(x + 1, y), MULT));

            if val_down {
                let diag_val = grid[x + 1][y - 1] > 0;

                if diag_val {
                    arr.push((Pos(x + 1, y - 1), SQRT2));
                }
            }

            if val_up {
                let diag_val = grid[x + 1][y + 1];

                if diag_val > 0 {
                    arr.push((Pos(x + 1, y + 1), SQRT2));
                }
            }
        }

        if val_up {
            arr.push((Pos(x, y + 1), MULT));
        }

        if val_down {
            arr.push((Pos(x, y - 1), MULT));
        }

        arr
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InfluencedPosAPI {
    pub normal_influence: usize,
}

impl PositionAPI for InfluencedPosAPI {
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
    fn successors_within(&self,
                         pos: &Pos,
                         grid: &[Vec<usize>],
                         window: ((usize, usize), (usize, usize)))
                         -> ArrayVec<(Pos, usize), 8> {
        let &Pos(x, y) = pos;
        let mut arr = ArrayVec::<(Pos, usize), 8>::new();
        //let arr = Vec<(Pos, f32)>();

        let mut val_left: usize = 0;
        let mut val_down: usize = 0;
        let mut val_right: usize = 0;
        let mut val_up: usize = 0;

        let ((x0, y0), (x1, y1)) = window;

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
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InvertPosAPI();

impl PositionAPI for InvertPosAPI {
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

    fn successors(&self, pos: &Pos, grid: &[Vec<usize>]) -> ArrayVec<(Pos, usize), 8> {
        self.successors_within(pos, grid, ((0, 0), (grid.len(), grid[0].len())))
    }

    fn successors_within(&self,
                         pos: &Pos,
                         grid: &[Vec<usize>],
                         window: ((usize, usize), (usize, usize)))
                         -> ArrayVec<(Pos, usize), 8> {
        let &Pos(x, y) = pos;
        let mut arr = ArrayVec::<(Pos, usize), 8>::new();
        //let arr = Vec<(Pos, f32)>();

        let mut val_left: bool = false;
        let mut val_down: bool = false;
        let mut val_right: bool = false;
        let mut val_up: bool = false;

        let ((x0, y0), (x1, y1)) = window;

        if x > x0 {
            val_left = grid[x - 1][y] == 0;
        }

        if y > y0 {
            val_down = grid[x][y - 1] == 0;
        }

        if x + 1 < x1 {
            val_right = grid[x + 1][y] == 0;
        }

        if y + 1 < y1 {
            val_up = grid[x][y + 1] == 0;
        }

        if val_left {
            arr.push((Pos(x - 1, y), MULT));

            if val_down {
                let diag_val = grid[x - 1][y - 1] == 0;

                if diag_val {
                    arr.push((Pos(x - 1, y - 1), SQRT2));
                }
            }

            if val_up {
                let diag_val = grid[x - 1][y + 1] == 0;

                if diag_val {
                    arr.push((Pos(x - 1, y + 1), SQRT2));
                }
            }
        }

        if val_right {
            arr.push((Pos(x + 1, y), MULT));

            if val_down {
                let diag_val = grid[x + 1][y - 1] == 0;

                if diag_val {
                    arr.push((Pos(x + 1, y - 1), SQRT2));
                }
            }

            if val_up {
                let diag_val = grid[x + 1][y + 1];

                if diag_val == 0 {
                    arr.push((Pos(x + 1, y + 1), SQRT2));
                }
            }
        }

        if val_up {
            arr.push((Pos(x, y + 1), MULT));
        }

        if val_down {
            arr.push((Pos(x, y - 1), MULT));
        }

        arr
    }
}
