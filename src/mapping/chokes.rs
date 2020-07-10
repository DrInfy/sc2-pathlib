use crate::mapping::map_point;
use crate::mapping::map_point::Cliff;
use crate::path_find::pos::Pos;
use crate::path_find::pos::{DIAGONAL_MINUS_CARDINAL, MULT, MULTF64, SQRT2};
use crate::path_find::PathFind;
use pathfinding::prelude::absdiff;
use pyo3::prelude::*;
use std::cmp;
use std::collections::HashSet;

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
        let xmin = x;
        let xmax = cmp::min(x as i64 + choke_distance as i64, x_end as i64) as usize;
        let ymin = cmp::max(y as i64 - choke_distance as i64, y_start as i64) as usize;
        let ymax = cmp::min(y as i64 + choke_distance as i64, y_end as i64) as usize;

        for x_new in xmin..xmax {
            for y_new in ymin..ymax {
                if !points[x_new][y_new].is_border {
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

#[pyclass]
#[derive(Clone)]
pub struct Choke {
    pub lines: Vec<((usize, usize), (usize, usize))>,
    pub side1: Vec<(usize, usize)>,
    pub side2: Vec<(usize, usize)>,
    pub pixels: Vec<(usize, usize)>,
}
#[pymethods]
impl Choke {
    #[getter(lines)]
    fn get_lines(&self) -> Vec<((usize, usize), (usize, usize))> { self.lines.clone() }

    #[getter(side1)]
    fn get_side1(&self) -> Vec<(usize, usize)> { self.side1.clone() }

    #[getter(side2)]
    fn get_side2(&self) -> Vec<(usize, usize)> { self.side2.clone() }
}

impl Choke {
    pub fn new(line: ((usize, usize), (usize, usize))) -> Self {
        let mut lines = Vec::<((usize, usize), (usize, usize))>::new();
        lines.push(line);
        let mut side1 = Vec::<(usize, usize)>::new();
        side1.push(line.0);
        let mut side2 = Vec::<(usize, usize)>::new();
        side2.push(line.1);
        let pixels = Vec::<(usize, usize)>::new();

        Choke { lines,
                side1,
                side2,
                pixels }
    }

    fn add_line(&mut self, point1: (usize, usize), point2: (usize, usize)) {
        self.lines.push((point1, point2));
        self.side1.push(point1);
        self.side2.push(point2);
    }
}

pub fn group_chokes(choke_lines: &mut Vec<((usize, usize), (usize, usize))>) -> Vec<Choke> {
    let mut result = Vec::<Choke>::new();
    let mut used_indices = HashSet::new();

    for i in 0..choke_lines.len() {
        if used_indices.contains(&i) {
            continue;
        }

        used_indices.insert(i);
        let mut current_choke = Choke::new(choke_lines[i]);
        let mut last_line_count = 0;
        let mut current_line_count = current_choke.lines.len();
        let MULT2 = MULT * 2;

        while last_line_count < current_line_count {
            for j in (i + 1)..choke_lines.len() {
                if used_indices.contains(&j) {
                    continue;
                }
                let check_line = choke_lines[j];
                for k in 0..current_choke.lines.len() {
                    let point1 = current_choke.side1[k];
                    let mut added = false;
                    if octile_distance(check_line.0, point1) <= MULT2 {
                        for l in 0..current_choke.lines.len() {
                            let point2 = current_choke.side2[l];
                            if octile_distance(check_line.1, point2) <= MULT2 {
                                used_indices.insert(j);
                                if octile_distance(check_line.0, point1) > 0
                                   || octile_distance(check_line.1, point2) > 0
                                {
                                    current_choke.add_line(check_line.0, check_line.1);
                                    added = true;
                                }
                                break;
                            }
                        }
                    }
                    if octile_distance(check_line.1, point1) <= MULT2 {
                        for l in 0..current_choke.lines.len() {
                            let point2 = current_choke.side2[l];
                            if octile_distance(check_line.0, point2) <= MULT2 {
                                used_indices.insert(j);
                                if octile_distance(check_line.1, point1) > 0
                                   && octile_distance(check_line.0, point2) > 0
                                {
                                    current_choke.add_line(check_line.1, check_line.0);
                                }
                                added = true;
                                break;
                            }
                        }
                    }
                    if added {
                        break;
                    }
                }
            }
            last_line_count = current_line_count;
            current_line_count = current_choke.lines.len();
        }

        result.push(current_choke);
    }

    return result;
}

#[inline]
pub fn octile_distance(first: (usize, usize), second: (usize, usize)) -> usize {
    let dx = absdiff(first.0, second.0);
    let dy = absdiff(first.1, second.1);

    if dx > dy {
        MULT * dx + DIAGONAL_MINUS_CARDINAL * dy
    } else {
        MULT * dy + DIAGONAL_MINUS_CARDINAL * dx
    }
}
