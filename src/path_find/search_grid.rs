#[derive(Clone, Debug)]
pub struct FreeFinder {
    closest_grid: Vec<(i64, i64)>,
}

impl FreeFinder {
    pub fn new() -> FreeFinder {
        let closest_grid = create_search_grid();
        FreeFinder { closest_grid }
    }

    pub fn find_free(&self, lookup: (usize, usize), map: &[Vec<usize>], width: usize, height: usize) -> (usize, usize) {
        let mut result = (lookup.0, lookup.1);

        for offset in &self.closest_grid {
            let adjusted = ((lookup.0 as i64 + offset.0), (lookup.1 as i64 + offset.1));

            if adjusted.0 >= 0 && adjusted.1 >= 0 {
                let adjusted_usize = (adjusted.0 as usize, adjusted.1 as usize);

                if adjusted_usize.0 < width && adjusted_usize.1 < height && map[adjusted_usize.0][adjusted_usize.1] > 0
                {
                    result = adjusted_usize;
                    break;
                }
            }
        }

        result
    }
}

/// ## Search order as follows:
/// ___A9A___
/// __87678__
/// _8543458_
/// A7421247A
/// 963101369
/// A7421247A
/// _854345__
/// __87678__
/// ___A9A___
fn create_search_grid() -> Vec<(i64, i64)> {
    let mut search_grid: Vec<(i64, i64)> = Vec::<(i64, i64)>::new();
    // 0, Not checked
    //search_grid.push((0, 0));

    // 1
    search_grid.push((-1, 0));
    search_grid.push((1, 0));
    search_grid.push((0, -1));
    search_grid.push((0, 1));

    // 2
    search_grid.push((1, 1));
    search_grid.push((-1, -1));
    search_grid.push((1, -1));
    search_grid.push((-1, 1));

    // 3
    search_grid.push((-2, 0));
    search_grid.push((2, 0));
    search_grid.push((0, -2));
    search_grid.push((0, 2));

    // 4
    search_grid.push((-2, 1));
    search_grid.push((2, -1));
    search_grid.push((-1, -2));
    search_grid.push((1, 2));
    search_grid.push((-2, -1));
    search_grid.push((2, 1));
    search_grid.push((1, -2));
    search_grid.push((-1, 2));

    // 5
    search_grid.push((2, 2));
    search_grid.push((-2, -2));
    search_grid.push((2, -2));
    search_grid.push((-2, 2));

    // 6
    search_grid.push((-3, 0));
    search_grid.push((3, 0));
    search_grid.push((0, -3));
    search_grid.push((0, 3));

    // 7
    search_grid.push((-3, 1));
    search_grid.push((3, -1));
    search_grid.push((-1, -3));
    search_grid.push((1, 3));
    search_grid.push((-3, -1));
    search_grid.push((3, 1));
    search_grid.push((1, -3));
    search_grid.push((-1, 3));

    // 8
    search_grid.push((-3, 2));
    search_grid.push((3, -2));
    search_grid.push((-2, -3));
    search_grid.push((2, 3));
    search_grid.push((-3, -2));
    search_grid.push((3, 2));
    search_grid.push((2, -3));
    search_grid.push((-2, 3));

    // 9
    search_grid.push((-4, 0));
    search_grid.push((4, 0));
    search_grid.push((0, -4));
    search_grid.push((0, 4));

    // A
    search_grid.push((-4, 1));
    search_grid.push((4, -1));
    search_grid.push((-1, -4));
    search_grid.push((1, 4));
    search_grid.push((-4, -1));
    search_grid.push((4, 1));
    search_grid.push((1, -4));
    search_grid.push((-1, 4));

    search_grid
}
