use crate::mapping::map_point;
use crate::mapping::map_point::Cliff;
use crate::path_find::pos::Pos;
use crate::path_find::pos::MULTF64;
use crate::path_find::PathFind;
use std::cmp;

pub fn solve_chokes(points: &mut Vec<Vec<map_point::MapPoint>>,
                    ground_pathing: &PathFind,
                    chokes: &mut Vec<((usize, usize), (usize, usize))>,
                    x: usize,
                    y: usize,
                    x_start: usize,
                    y_start: usize,
                    x_end: usize,
                    y_end: usize) {
    let pos_start = Pos(x, y);
    let choke_distance = 13f64;
    let choke_border_distance = 30f64;

    if points[pos_start.0][pos_start.1].is_border {
        let reachable_borders = ground_pathing.djiktra((x as f64, y as f64), choke_border_distance);
        let xmin = x_start;
        let xmax = x_end;
        let ymin = cmp::max(y as i64 - choke_distance as i64, y_start as i64) as usize;
        let ymax = cmp::min(y as i64 + choke_distance as i64, y_end as i64) as usize;

        for x_new in xmin..xmax {
            for y_new in ymin..ymax {
                if (x_new + y_new) % 2 == 0 || !points[x_new][y_new].is_border {
                    // Needs to be a border to be acceptable position
                    continue;
                }

                let pos = Pos(x_new, y_new);
                let flight_distance = pos.euclidean_distance(&pos_start) as f64 / MULTF64;

                if flight_distance > choke_distance || flight_distance < 2f64 {
                    continue;
                }

                let mut found = false;
                for pos_distance in &reachable_borders {
                    if (pos_distance.0).0 == x_new && (pos_distance.0).1 == y_new {
                        found = true;
                        break;
                    }
                }

                if found {
                    continue;
                }

                let dots = flight_distance as usize;
                let unit_vector =
                    ((pos.0 as f64 - x as f64) / flight_distance, (pos.1 as f64 - y as f64) / flight_distance);
                let mut wall_hit = false;
                let mut set_chokes = Vec::<(usize, usize)>::new();

                for i in 1..dots {
                    let draw_x = (x as f64 + unit_vector.0 * i as f64) as usize;
                    let draw_y = (y as f64 + unit_vector.1 * i as f64) as usize;
                    if (draw_x == x && draw_y == y) || (draw_x == pos.0 && draw_y == pos.1) {
                        continue;
                    }
                    if !points[draw_x][draw_y].walkable {
                        wall_hit = true;
                        break;
                    } else {
                        set_chokes.push((draw_x, draw_y));
                    }
                }

                if !wall_hit {
                    for dot in set_chokes {
                        points[dot.0][dot.1].is_choke = true;
                    }
                    points[x][y].is_choke = true;
                    points[pos.0][pos.1].is_choke = true;
                    chokes.push(((x, y), (pos.0, pos.1)));
                }
            }
        }
    }
}
