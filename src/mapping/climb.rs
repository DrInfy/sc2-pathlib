use crate::mapping::map_point;
use crate::mapping::map_point::Cliff;
const DIFFERENCE: usize = 16;

pub fn modify_climb(points: &mut Vec<Vec<map_point::MapPoint>>, x: i32, y: i32, x_dir: i32, y_dir: i32) {
    let x0 = x as usize;
    let y0 = y as usize;
    let x1 = (x + x_dir) as usize;
    let y1 = (y + y_dir) as usize;
    let x2 = (x + x_dir * 2) as usize;
    let y2 = (y + y_dir * 2) as usize;
    // let mut check_point = points[x0][y0];
    // let mut next_point = points[(x + x_dir) as usize][(y + y_dir) as usize];
    // let mut last_point = points[(x + x_dir * 2) as usize][(y + y_dir * 2) as usize];

    if points[x1][y1].walkable || !points[x2][y2].walkable {
        return; // Not climbable
    }

    // There are 12 possible reaper walls:
    // 01 10 11 00 01 10 11 11 10 01 00 00
    // 01 10 00 11 11 11 01 10 00 00 10 01
    // Let's numerize the corners
    // 01
    // 23

    let h0 = points[x1][y1 + 1].height;
    let h1 = points[x1 + 1][y1 + 1].height;
    let h2 = points[x1][y1].height;
    let h3 = points[x1 + 1][y1].height;

    // Difference between levels is 15.9375 in standard map height maps
    // Difference between levels is 2 in standard sc2 measurement units.
    // Because of rounding the height difference needs to be exactly 16

    let set_low = |x: Cliff| {
        if x == Cliff::None || x == Cliff::Low {
            Cliff::Low
        } else {
            Cliff::Both
        }
    };

    let set_high = |x: Cliff| {
        if x == Cliff::None || x == Cliff::High {
            Cliff::High
        } else {
            Cliff::Both
        }
    };
    if x_dir != 0 && y_dir != 0 {
        if x_dir == y_dir {
            // Need to check following scenarios:
            // 10 11 00 01
            // 11 01 10 00
            if (h0 == h1 || h0 == h2) && h2 == h1 + DIFFERENCE && h0 == h3 {
                // 10 00
                // 11 10
                points[x1][y1].climbable = true;
                if x_dir > 0 {
                    points[x0][y0].cliff_type = set_high(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_low(points[x2][y2].cliff_type);
                } else {
                    points[x0][y0].cliff_type = set_low(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_high(points[x2][y2].cliff_type);
                }
            } else if (h0 == h1 && h0 == h3 && h0 == h2 + DIFFERENCE) || (h0 == h2 && h0 == h3 && h1 == h2 + DIFFERENCE)
            {
                // 11 01
                // 01 00
                points[x1][y1].climbable = true;
                if x_dir > 0 {
                    points[x0][y0].cliff_type = set_low(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_high(points[x2][y2].cliff_type);
                } else {
                    points[x0][y0].cliff_type = set_high(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_low(points[x2][y2].cliff_type);
                }
            }
        } else {
            // Need to check following scenarios:
            // 01 11 10 00
            // 11 10 00 01
            if (h1 == h2 && h1 == h3 && h1 == h0 + DIFFERENCE) || (h0 == h1 && h0 == h2 && h3 == h0 + DIFFERENCE) {
                // 01 00
                // 11 01
                points[x1][y1].climbable = true;
                if x_dir > 0 {
                    points[x0][y0].cliff_type = set_low(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_high(points[x2][y2].cliff_type);
                } else {
                    points[x0][y0].cliff_type = set_high(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_low(points[x2][y2].cliff_type);
                }
            } else if (h0 == h1 && h0 == h2 && h0 == h3 + DIFFERENCE) || (h1 == h2 && h1 == h3 && h0 == h3 + DIFFERENCE)
            {
                // 11 10
                // 10 00
                points[x1][y1].climbable = true;
                if x_dir > 0 {
                    points[x0][y0].cliff_type = set_high(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_low(points[x2][y2].cliff_type);
                } else {
                    points[x0][y0].cliff_type = set_low(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_high(points[x2][y2].cliff_type);
                }
            }
        }
    } else {
        if x_dir != 0 {
            // Need to check following scenarios:
            // 01 10
            // 01 10
            if h0 == h2 && h1 == h3 && h0 + DIFFERENCE == h1 {
                // 01
                // 01
                points[x1][y1].climbable = true;
                if x_dir > 0 {
                    points[x0][y0].cliff_type = set_low(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_high(points[x2][y2].cliff_type);
                } else {
                    points[x0][y0].cliff_type = set_high(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_low(points[x2][y2].cliff_type);
                }
            } else if h0 == h2 && h1 == h3 && h0 == h1 + DIFFERENCE {
                // 10
                // 10
                points[x1][y1].climbable = true;
                if x_dir > 0 {
                    points[x0][y0].cliff_type = set_high(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_low(points[x2][y2].cliff_type);
                } else {
                    points[x0][y0].cliff_type = set_low(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_high(points[x2][y2].cliff_type);
                }
            }
        } else if y_dir != 0 {
            // Need to check following scenarios:
            // 00 11
            // 11 00
            if h0 == h1 && h2 == h3 && h0 + DIFFERENCE == h2 {
                // 00
                // 11
                points[x1][y1].climbable = true;
                if y_dir > 0 {
                    points[x0][y0].cliff_type = set_high(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_low(points[x2][y2].cliff_type);
                } else {
                    points[x0][y0].cliff_type = set_low(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_high(points[x2][y2].cliff_type);
                }
            } else if h0 == h1 && h2 == h3 && h0 == h2 + DIFFERENCE {
                // 11
                // 00
                points[x1][y1].climbable = true;
                if y_dir > 0 {
                    points[x0][y0].cliff_type = set_low(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_high(points[x2][y2].cliff_type);
                } else {
                    points[x0][y0].cliff_type = set_high(points[x0][y0].cliff_type);
                    points[x2][y2].cliff_type = set_low(points[x2][y2].cliff_type);
                }
            }
        }
    }
}
