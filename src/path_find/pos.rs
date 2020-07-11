use pathfinding::prelude::absdiff;

//static SQRT2: f32 = 1.4142135623730950488016887242097;
pub static SQRT2: usize = 14142;
pub static MULT: usize = 10000;
pub static MULTF64: f64 = 10000.0;

//constants
pub static DIAGONAL_MINUS_CARDINAL: usize = 4142;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub usize, pub usize);

impl Pos {
    #[inline]
    pub fn manhattan_distance(&self, other: &Pos) -> usize {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) * MULT
    }

    #[inline]
    pub fn euclidean_distance(&self, other: &Pos) -> usize {
        let a = self.0 as f64 - other.0 as f64;
        let b = self.1 as f64 - other.1 as f64;
        let dist2 = a * a + b * b;
        (dist2.sqrt() * MULTF64) as usize
    }

    #[inline]
    pub fn octile_distance(&self, other: &Pos) -> usize {
        let dx = absdiff(self.0, other.0);
        let dy = absdiff(self.1, other.1);

        if dx > dy {
            MULT * dx + DIAGONAL_MINUS_CARDINAL * dy
        } else {
            MULT * dy + DIAGONAL_MINUS_CARDINAL * dx
        }
    }

    pub fn successors(&self, grid: &[Vec<usize>]) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut arr = Vec::<(Pos, usize)>::with_capacity(8);
        //let arr = Vec<(Pos, f32)>();

        let mut val_left: bool = false;
        let mut val_down: bool = false;
        let mut val_right: bool = false;
        let mut val_up: bool = false;

        if x > 0 {
            val_left = grid[x - 1][y] > 0;
        }

        if y > 0 {
            val_down = grid[x][y - 1] > 0;
        }

        if x + 1 < grid.len() {
            val_right = grid[x + 1][y] > 0;
        }

        if y + 1 < grid[0].len() {
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
pub struct InfluencedPos(pub usize, pub usize);

impl InfluencedPos {
    #[inline]
    pub fn manhattan_distance(&self, other: &InfluencedPos, normal_influence: usize) -> usize {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) * MULT * normal_influence
    }

    #[inline]
    pub fn euclidean_distance(&self, other: &InfluencedPos, normal_influence: usize) -> usize {
        let a = (self.0 - other.0) * normal_influence;
        let b = (self.1 - other.1) * normal_influence;
        let dist2 = a * a + b * b;
        ((dist2 as f64).sqrt() * MULTF64) as usize
    }

    #[inline]
    pub fn octile_distance(&self, other: &InfluencedPos, normal_influence: usize) -> usize {
        let dx = absdiff(self.0, other.0);
        let dy = absdiff(self.1, other.1);

        if dx > dy {
            (MULT * dx + DIAGONAL_MINUS_CARDINAL * dy) * normal_influence
        } else {
            (MULT * dy + DIAGONAL_MINUS_CARDINAL * dx) * normal_influence
        }
    }

    pub fn successors(&self, grid: &[Vec<usize>]) -> Vec<(InfluencedPos, usize)> {
        let &InfluencedPos(x, y) = self;
        let mut arr = Vec::<(InfluencedPos, usize)>::with_capacity(8);
        //let arr = Vec<(Pos, f32)>();

        let mut val_left: usize = 0;
        let mut val_down: usize = 0;
        let mut val_right: usize = 0;
        let mut val_up: usize = 0;

        if x > 0 {
            val_left = grid[x - 1][y];
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
            arr.push((InfluencedPos(x - 1, y), val_left * MULT));

            if val_down > 0 {
                let diag_val = grid[x - 1][y - 1];

                if diag_val > 0 {
                    arr.push((InfluencedPos(x - 1, y - 1), diag_val * SQRT2));
                }
            }

            if val_up > 0 {
                let diag_val = grid[x - 1][y + 1];

                if diag_val > 0 {
                    arr.push((InfluencedPos(x - 1, y + 1), diag_val * SQRT2));
                }
            }
        }

        if val_right > 0 {
            arr.push((InfluencedPos(x + 1, y), val_right * MULT));

            if val_down > 0 {
                let diag_val = grid[x + 1][y - 1];

                if diag_val > 0 {
                    arr.push((InfluencedPos(x + 1, y - 1), diag_val * SQRT2));
                }
            }

            if val_up > 0 {
                let diag_val = grid[x + 1][y + 1];

                if diag_val > 0 {
                    arr.push((InfluencedPos(x + 1, y + 1), diag_val * SQRT2));
                }
            }
        }

        if val_up > 0 {
            arr.push((InfluencedPos(x, y + 1), val_up * MULT));
        }

        if val_down > 0 {
            arr.push((InfluencedPos(x, y - 1), val_down * MULT));
        }

        arr
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InvertPos(pub usize, pub usize);

impl InvertPos {
    #[inline]
    pub fn manhattan_distance(&self, other: &InvertPos) -> usize {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) * MULT
    }

    #[inline]
    pub fn euclidean_distance(&self, other: &InvertPos) -> usize {
        let a = self.0 - other.0;
        let b = self.1 - other.1;
        let dist2 = a * a + b * b;
        ((dist2 as f64).sqrt() * MULTF64) as usize
    }

    #[inline]
    pub fn octile_distance(&self, other: &InvertPos) -> usize {
        let dx = absdiff(self.0, other.0);
        let dy = absdiff(self.1, other.1);

        if dx > dy {
            MULT * dx + DIAGONAL_MINUS_CARDINAL * dy
        } else {
            MULT * dy + DIAGONAL_MINUS_CARDINAL * dx
        }
    }

    pub fn successors(&self, grid: &[Vec<usize>]) -> Vec<(InvertPos, usize)> {
        let &InvertPos(x, y) = self;
        let mut arr = Vec::<(InvertPos, usize)>::with_capacity(8);
        //let arr = Vec<(Pos, f32)>();

        let mut val_left: bool = false;
        let mut val_down: bool = false;
        let mut val_right: bool = false;
        let mut val_up: bool = false;

        if x > 0 {
            val_left = grid[x - 1][y] == 0;
        }

        if y > 0 {
            val_down = grid[x][y - 1] == 0;
        }

        if x + 1 < grid.len() {
            val_right = grid[x + 1][y] == 0;
        }

        if y + 1 < grid[0].len() {
            val_up = grid[x][y + 1] == 0;
        }

        if val_left {
            arr.push((InvertPos(x - 1, y), MULT));

            if val_down {
                let diag_val = grid[x - 1][y - 1] == 0;

                if diag_val {
                    arr.push((InvertPos(x - 1, y - 1), SQRT2));
                }
            }

            if val_up {
                let diag_val = grid[x - 1][y + 1] == 0;

                if diag_val {
                    arr.push((InvertPos(x - 1, y + 1), SQRT2));
                }
            }
        }

        if val_right {
            arr.push((InvertPos(x + 1, y), MULT));

            if val_down {
                let diag_val = grid[x + 1][y - 1] == 0;

                if diag_val {
                    arr.push((InvertPos(x + 1, y - 1), SQRT2));
                }
            }

            if val_up {
                let diag_val = grid[x + 1][y + 1];

                if diag_val == 0 {
                    arr.push((InvertPos(x + 1, y + 1), SQRT2));
                }
            }
        }

        if val_up {
            arr.push((InvertPos(x, y + 1), MULT));
        }

        if val_down {
            arr.push((InvertPos(x, y - 1), MULT));
        }

        arr
    }
}
