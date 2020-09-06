use crate::mapping::map_point;
use crate::path_find::pos::Pos;
use crate::path_find::pos::{DIAGONAL_MINUS_CARDINAL, MULT, MULTF32, SQRT2};
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
    let choke_distance = 13f32;
    let choke_border_distance = 30f32;

    if points[pos_start.0][pos_start.1].is_border {
        let reachable_borders = ground_pathing.djiktra((x as f32, y as f32), choke_border_distance);
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
                let flight_distance = pos.euclidean_distance(&pos_start) as f32 / MULTF32;

                if flight_distance > choke_distance || flight_distance < 2f32 {
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

                let dots = flight_distance as i64;
                let unit_vector =
                    ((pos.0 as f32 - x as f32) / flight_distance, (pos.1 as f32 - y as f32) / flight_distance);
                let mut wall_hit = false;

                for i in 1..dots {
                    let draw_x = (x as f32 + unit_vector.0 * i as f32) as usize;
                    let draw_y = (y as f32 + unit_vector.1 * i as f32) as usize;
                    if (draw_x == x && draw_y == y) || (draw_x == pos.0 && draw_y == pos.1) {
                        continue;
                    }
                    if !points[draw_x][draw_y].walkable {
                        wall_hit = true;
                        break;
                    }
                }

                if !wall_hit && dots > 4 {
                    // Cross reference to X shape
                    let center = ((pos.0 + x) / 2, (pos.1 + y) / 2);
                    let perdicular_unit_vector = (-unit_vector.1, unit_vector.0);
                    let half_dots = dots / 2;
                    for i in -half_dots..half_dots {
                        let draw_x = (center.0 as f32 + perdicular_unit_vector.0 * i as f32) as usize;
                        let draw_y = (center.1 as f32 + perdicular_unit_vector.1 * i as f32) as usize;

                        if !points[draw_x][draw_y].walkable {
                            wall_hit = true;
                            break;
                        }
                    }
                }

                if !wall_hit {
                    chokes.push(((x, y), (pos.0, pos.1)));
                }
            }
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Choke {
    pub main_line: ((f32, f32), (f32, f32)),
    pub lines: Vec<((usize, usize), (usize, usize))>,
    pub side1: Vec<(usize, usize)>,
    pub side2: Vec<(usize, usize)>,
    pub pixels: Vec<(usize, usize)>,
    pub min_length: f32,
}
#[pymethods]
impl Choke {
    #[getter(lines)]
    fn get_lines(&self) -> Vec<((usize, usize), (usize, usize))> { self.lines.clone() }

    #[getter(side1)]
    fn get_side1(&self) -> Vec<(usize, usize)> { self.side1.clone() }

    #[getter(side2)]
    fn get_side2(&self) -> Vec<(usize, usize)> { self.side2.clone() }
    #[getter(main_line)]
    fn get_main_line(&self) -> ((f32, f32), (f32, f32)) { self.main_line.clone() }

    #[getter(pixels)]
    fn get_pixels(&self) -> Vec<(usize, usize)> { self.pixels.clone() }

    #[getter(min_length)]
    fn get_min_length(&self) -> f32 { self.min_length }
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
        // Real main line is created later on by calculating averages
        let main_line = (((line.0).0 as f32, (line.0).1 as f32), ((line.1).0 as f32, (line.1).1 as f32));
        let min_length = distance(line.0, line.1);
        Choke { main_line,
                lines,
                side1,
                side2,
                pixels,
                min_length }
    }

    fn add_line(&mut self, point1: (usize, usize), point2: (usize, usize)) {
        self.lines.push((point1, point2));
        if !self.side1.contains(&point1) {
            self.side1.push(point1);
        }
        if !self.side2.contains(&point2) {
            self.side2.push(point2);
        }
    }

    fn finalize(&mut self, points: &mut Vec<Vec<map_point::MapPoint>>) {
        self.remove_excess_lines();
        self.calc_final_line();
    }

    fn remove_excess_lines(&mut self) {
        let mut distances = Vec::<f32>::new();
        let mut min_distance = 999f32;
        for line in &self.lines {
            let d = distance(line.0, line.1);
            distances.push(d);
            if d < min_distance {
                min_distance = d;
            }
        }
        let max = self.lines.len();
        for i in (0..max).rev() {
            if distances[i] > min_distance + 2.5f32 {
                // Remove line
                self.lines.remove(i);
            }
        }

        self.min_length = min_distance;
    }

    fn set_points(&mut self, points: &mut Vec<Vec<map_point::MapPoint>>) {
        for line in &self.lines {
            let pos1 = Pos((line.0).0, (line.0).1);
            let pos2 = Pos((line.1).0, (line.1).1);

            points[pos1.0][pos1.1].is_choke = true;
            points[pos2.0][pos2.1].is_choke = true;

            let flight_distance = pos1.euclidean_distance(&pos2) as f32 / MULTF32;

            let dots = flight_distance as usize;
            let unit_vector =
                ((pos2.0 as f32 - pos1.0 as f32) / flight_distance, (pos2.1 as f32 - pos1.1 as f32) / flight_distance);

            for i in 1..dots {
                let draw_x = (pos1.0 as f32 + unit_vector.0 * i as f32) as usize;
                let draw_y = (pos1.1 as f32 + unit_vector.1 * i as f32) as usize;
                if (draw_x == pos1.0 && draw_y == pos1.1) || (draw_x == pos2.0 && draw_y == pos2.1) {
                    continue;
                }

                points[draw_x][draw_y].is_choke = true;
                let new_point = (draw_x, draw_y);

                if !self.pixels.contains(&new_point) {
                    self.pixels.push((draw_x, draw_y));
                }
            }
        }
    }

    fn calc_final_line(&mut self) {
        let mut x_sum: usize = 0;
        let mut y_sum: usize = 0;
        for point in &self.side1 {
            x_sum += point.0;
            y_sum += point.1;
        }
        let point1 = (x_sum as f32 / self.side1.len() as f32, y_sum as f32 / self.side1.len() as f32);

        x_sum = 0;
        y_sum = 0;
        for point in &self.side2 {
            x_sum += point.0;
            y_sum += point.1;
        }
        let point2 = (x_sum as f32 / self.side2.len() as f32, y_sum as f32 / self.side2.len() as f32);

        self.main_line = (point1, point2);
    }
}

pub fn group_chokes(choke_lines: &mut Vec<((usize, usize), (usize, usize))>,
                    points: &mut Vec<Vec<map_point::MapPoint>>)
                    -> Vec<Choke> {
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

        while last_line_count < current_line_count {
            for j in (i + 1)..choke_lines.len() {
                if used_indices.contains(&j) {
                    continue;
                }
                let check_line = choke_lines[j];
                for k in 0..current_choke.side1.len() {
                    let point1 = current_choke.side1[k];
                    let mut added = false;
                    if octile_distance(check_line.0, point1) <= SQRT2 {
                        for l in 0..current_choke.side2.len() {
                            let point2 = current_choke.side2[l];
                            if octile_distance(check_line.1, point2) <= SQRT2 {
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
                    if octile_distance(check_line.1, point1) <= SQRT2 {
                        for l in 0..current_choke.side2.len() {
                            let point2 = current_choke.side2[l];
                            if octile_distance(check_line.0, point2) <= SQRT2 {
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

    let max = result.len();
    for i in (0..max).rev() {
        // let mut choke = &result[i];
        result[i].finalize(points);
        if result[i].lines.len() < 4 {
            // Doesn't really seem like a choke
            result.remove(i);
        } else {
            result[i].set_points(points);
        }
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

#[inline]
fn distance(first: (usize, usize), second: (usize, usize)) -> f32 {
    let pos1 = Pos(first.0, first.1);
    let pos2 = Pos(second.0, second.1);

    return pos1.euclidean_distance(&pos2) as f32 / MULTF32;
}
