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
    let search_grid: Vec<(i64, i64)> = vec![// 0, Not checked
                                            //(0, 0),

                                            // 1
                                            (-1, 0),
                                            (1, 0),
                                            (0, -1),
                                            (0, 1),
                                            // 2
                                            (1, 1),
                                            (-1, -1),
                                            (1, -1),
                                            (-1, 1),
                                            // 3
                                            (-2, 0),
                                            (2, 0),
                                            (0, -2),
                                            (0, 2),
                                            // 4
                                            (-2, 1),
                                            (2, -1),
                                            (-1, -2),
                                            (1, 2),
                                            (-2, -1),
                                            (2, 1),
                                            (1, -2),
                                            (-1, 2),
                                            // 5
                                            (2, 2),
                                            (-2, -2),
                                            (2, -2),
                                            (-2, 2),
                                            // 6
                                            (-3, 0),
                                            (3, 0),
                                            (0, -3),
                                            (0, 3),
                                            // 7
                                            (-3, 1),
                                            (3, -1),
                                            (-1, -3),
                                            (1, 3),
                                            (-3, -1),
                                            (3, 1),
                                            (1, -3),
                                            (-1, 3),
                                            // 8
                                            (-3, 2),
                                            (3, -2),
                                            (-2, -3),
                                            (2, 3),
                                            (-3, -2),
                                            (3, 2),
                                            (2, -3),
                                            (-2, 3),
                                            // 9
                                            (-4, 0),
                                            (4, 0),
                                            (0, -4),
                                            (0, 4),
                                            // A
                                            (-4, 1),
                                            (4, -1),
                                            (-1, -4),
                                            (1, 4),
                                            (-4, -1),
                                            (4, 1),
                                            (1, -4),
                                            (-1, 4),];

    search_grid
}
