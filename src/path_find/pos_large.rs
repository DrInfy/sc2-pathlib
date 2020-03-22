use pathfinding::prelude::absdiff;

//static SQRT2: f32 = 1.4142135623730950488016887242097;
pub static SQRT2: usize = 14142;
pub static MULT: usize = 10000;
pub static MULTF64: f64 = 10000.0;

//constants
pub static DIAGONAL_MINUS_CARDINAL: usize = 4142;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PosLarge(pub usize, pub usize);

impl PosLarge {
    #[inline]
    pub fn manhattan_distance(&self, other: &PosLarge) -> usize {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) * MULT
    }

    #[inline]
    pub fn euclidean_distance(&self, other: &PosLarge) -> usize {
        let a = self.0 - other.0;
        let b = self.1 - other.1;
        let dist2 = a * a + b * b;
        ((dist2 as f64).sqrt() * MULTF64) as usize

        // ((((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)) as f64).sqrt() * MULTF64) as usize
    }

    #[inline]
    pub fn octile_distance(&self, other: &PosLarge) -> usize {
        let dx = absdiff(self.0, other.0);
        let dy = absdiff(self.1, other.1);

        if dx > dy {
            MULT * dx + DIAGONAL_MINUS_CARDINAL * dy
        } else {
            MULT * dy + DIAGONAL_MINUS_CARDINAL * dx
        }
    }

    pub fn successors(&self, grid: &[Vec<usize>]) -> Vec<(PosLarge, usize)> {
        let &PosLarge(x, y) = self;
        let mut arr = Vec::<(PosLarge, usize)>::with_capacity(8);
        //let arr = Vec<(PosLarge, f32)>();

        let mut val_left: bool = false;
        let mut val_down: bool = false;
        let mut val_right: bool = false;
        let mut val_up: bool = false;

        let mut val_left_up: bool = false;
        let mut val_left_down: bool = false;
        let mut val_right_up: bool = false;
        let mut val_right_down: bool = false;

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
            arr.push((PosLarge(x - 1, y), MULT));

            if val_down {
                val_left_down = grid[x - 1][y - 1] > 0;
            }

            if val_up {
                val_left_up = grid[x - 1][y + 1] > 0;
            }
        }

        if val_right {
            arr.push((PosLarge(x + 1, y), MULT));

            if val_down {
                val_right_down = grid[x + 1][y - 1] > 0;
            }

            if val_up {
                val_right_up = grid[x + 1][y + 1] > 0;
            }
        }

        if val_up && (val_left_up || val_right_up) {
            arr.push((PosLarge(x, y + 1), MULT));
        }

        if val_down && (val_left_down || val_right_down) {
            arr.push((PosLarge(x, y - 1), MULT));
        }

        if val_left && (val_left_up || val_left_down) {
            arr.push((PosLarge(x - 1, y), MULT));
        }

        if val_right && (val_right_up || val_right_down) {
            arr.push((PosLarge(x + 1, y), MULT));
        }

        if val_left_up {
            arr.push((PosLarge(x - 1, y + 1), SQRT2));
        }

        if val_left_down {
            arr.push((PosLarge(x - 1, y - 1), SQRT2));
        }

        if val_right_up {
            arr.push((PosLarge(x + 1, y + 1), SQRT2));
        }

        if val_right_down {
            arr.push((PosLarge(x + 1, y - 1), SQRT2));
        }

        arr
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InfluencedPosLarge(pub usize, pub usize);

impl InfluencedPosLarge {
    #[inline]
    pub fn manhattan_distance(&self, other: &InfluencedPosLarge, normal_influence: usize) -> usize {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) * MULT * normal_influence
    }

    #[inline]
    pub fn euclidean_distance(&self, other: &InfluencedPosLarge, normal_influence: usize) -> usize {
        let a = (self.0 - other.0) * normal_influence;
        let b = (self.1 - other.1) * normal_influence;
        let dist2 = a * a + b * b;
        ((dist2 as f64).sqrt() * MULTF64) as usize
    }

    #[inline]
    pub fn octile_distance(&self, other: &InfluencedPosLarge, normal_influence: usize) -> usize {
        let dx = absdiff(self.0, other.0);
        let dy = absdiff(self.1, other.1);

        if dx > dy {
            (MULT * dx + DIAGONAL_MINUS_CARDINAL * dy) * normal_influence
        } else {
            (MULT * dy + DIAGONAL_MINUS_CARDINAL * dx) * normal_influence
        }
    }

    pub fn successors(&self, grid: &[Vec<usize>]) -> Vec<(InfluencedPosLarge, usize)> {
        let &InfluencedPosLarge(x, y) = self;
        let mut arr = Vec::<(InfluencedPosLarge, usize)>::with_capacity(8);

        let mut val_left: usize = 0;
        let mut val_down: usize = 0;
        let mut val_right: usize = 0;
        let mut val_up: usize = 0;

        let mut val_left_up: usize = 0;
        let mut val_left_down: usize = 0;
        let mut val_right_up: usize = 0;
        let mut val_right_down: usize = 0;

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
            arr.push((InfluencedPosLarge(x, y + 1), val_up * MULT));
        }

        if val_down > 0 && (val_left_down > 0 || val_right_down > 0) {
            arr.push((InfluencedPosLarge(x, y - 1), val_down * MULT));
        }

        if val_left > 0 && (val_left_up > 0 || val_left_down > 0) {
            arr.push((InfluencedPosLarge(x - 1, y), val_left * MULT));
        }

        if val_right > 0 && (val_right_up > 0 || val_right_down > 0) {
            arr.push((InfluencedPosLarge(x + 1, y), val_right * MULT));
        }

        if val_left_up > 0 {
            arr.push((InfluencedPosLarge(x - 1, y + 1), val_left_up * SQRT2));
        }

        if val_left_down > 0 {
            arr.push((InfluencedPosLarge(x - 1, y - 1), val_left_down * SQRT2));
        }

        if val_right_up > 0 {
            arr.push((InfluencedPosLarge(x + 1, y + 1), val_right_up * SQRT2));
        }

        if val_right_down > 0 {
            arr.push((InfluencedPosLarge(x + 1, y - 1), val_right_down * SQRT2));
        }

        arr
    }
}
