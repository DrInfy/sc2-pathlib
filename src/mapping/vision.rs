use pyo3::prelude::*;

use crate::{
    helpers::round_point2,
    path_find::{octile_distance_f32, rectangle},
};

use super::map_point;
#[derive(Copy, Clone)]
pub enum VisionStatus {
    NotSeen,
    Seen,
    Detected,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct VisionUnit {
    detector: bool,
    flying: bool,
    position: (f32, f32),
    sight_range: f32,
}

#[pymethods]
impl VisionUnit {
    #[new]
    pub fn new(detector: bool, flying: bool, position: (f32, f32), sight_range: f32) -> Self {
        VisionUnit { detector,
                     flying,
                     position,
                     sight_range }
    }
}

#[pyclass]
pub struct VisionMap {
    units: Vec<VisionUnit>,
    pub points: Vec<Vec<VisionStatus>>,
    width: usize,
    height: usize,
}

#[pymethods]
impl VisionMap {
    pub fn clear(&mut self) {
        self.units.clear();

        for y in &mut self.points {
            for x in y {
                *x = VisionStatus::NotSeen;
            }
        }
    }

    pub fn add_unit(&mut self, unit: VisionUnit) { self.units.push(unit); }

    pub fn vision_status(& self, position: (f32, f32)) -> usize {
        let int_point = round_point2(position);
        match self.points[int_point.0][int_point.1] {
            VisionStatus::NotSeen => 0,
            VisionStatus::Seen => 1,
            VisionStatus::Detected => 2,
        }
    }
}

impl VisionMap {
    pub fn new_internal(width: usize, height: usize) -> Self {
        let units = vec![];
        let points = vec![vec![VisionStatus::NotSeen; height]; width];
        let width = points.len();
        let height = points[0].len();

        VisionMap { units,
                    points,
                    width,
                    height }
    }

    pub fn calculate_vision_map(&mut self, map_points: &Vec<Vec<map_point::MapPoint>>) {
        for unit in self.units.iter() {
            if unit.flying {
                if unit.detector {
                    set_detection(&mut self.points, &unit.position, unit.sight_range);
                } else {
                    set_vision(&mut self.points, &unit.position, unit.sight_range);
                }
            } else {
                if unit.detector {
                    calc_ground_detection(&mut self.points, map_points, &unit.position, unit.sight_range);
                } else {
                    calc_ground_vision(&mut self.points, map_points, &unit.position, unit.sight_range);
                }
            }
        }
    }

    pub fn draw_vision(&self) -> Vec<Vec<usize>> {
        let mut vision_map = vec![vec![0; self.height]; self.width];

        for x in 0..self.width {
            for y in 0..self.height {
                match self.points[x][y] {
                    VisionStatus::NotSeen => vision_map[x][y] = 0,
                    VisionStatus::Seen => vision_map[x][y] = 1,
                    VisionStatus::Detected => vision_map[x][y] = 2,
                }
            }
        }

        vision_map
    }
}

fn set_detection(points: &mut Vec<Vec<VisionStatus>>, position: &(f32, f32), sight_range: f32) {
    let u_position = round_point2(*position);
    let size = ((sight_range * 2f32) as usize, (sight_range * 2f32) as usize);
    let width = points.len();
    let height = points[0].len();

    let rect = rectangle::Rectangle::init_from_center2(u_position, size, width, height);

    for x in rect.x..rect.x_end {
        for y in rect.y..rect.y_end {
            let d = octile_distance_f32(u_position, (x, y));

            if d <= sight_range {
                points[x][y] = VisionStatus::Detected;
            }
        }
    }
}

fn set_vision(points: &mut Vec<Vec<VisionStatus>>, position: &(f32, f32), sight_range: f32) {
    let u_position = round_point2(*position);
    let size = ((sight_range * 2f32) as usize, (sight_range * 2f32) as usize);
    let width = points.len();
    let height = points[0].len();

    let rect = rectangle::Rectangle::init_from_center2(u_position, size, width, height);

    for x in rect.x..rect.x_end {
        for y in rect.y..rect.y_end {
            let d = octile_distance_f32(u_position, (x, y));

            if d <= sight_range && matches!(points[x][y], VisionStatus::NotSeen) {
                points[x][y] = VisionStatus::Seen;
            }
        }
    }
}

fn calc_ground_detection(points: &mut Vec<Vec<VisionStatus>>,
                         map_points: &Vec<Vec<map_point::MapPoint>>,
                         position: &(f32, f32),
                         sight_range: f32) {
    let circumference = 2f32 * sight_range * std::f32::consts::PI;
    let rays = circumference as usize;
    let step_mult = 1.3f32;
    let steps = (sight_range * step_mult) as usize;

    let u_position = round_point2(*position);
    let current_height = map_points[u_position.0][u_position.1].height;
    let mut max_height_seen = current_height / 8 * 8 + 7;

    if current_height % 8 >= 4 {
        max_height_seen += 8;
    }

    for index in 0..rays {
        let angle = index as f32 / rays as f32 * 2f32 * std::f32::consts::PI;
        let v_x = angle.sin();
        let v_y = angle.cos();

        for step in 0..steps {
            // Rays are only drawn until non-walkable is found and thus the cannot go out of bounds
            let step_f32 = step as f32 / step_mult;
            let new_pos =
                ((position.0 as f32 + v_x * step_f32) as usize, (position.1 as f32 + v_y * step_f32) as usize);

            // TODO: Same for height difference
            if map_points[new_pos.0][new_pos.1].height > max_height_seen {
                // Ray can't reach further
                break;
            }

            points[new_pos.0][new_pos.1] = VisionStatus::Detected;
        }
    }
}

fn calc_ground_vision(points: &mut Vec<Vec<VisionStatus>>,
                      map_points: &Vec<Vec<map_point::MapPoint>>,
                      position: &(f32, f32),
                      sight_range: f32) {
    let circumference = 2f32 * sight_range * std::f32::consts::PI;
    let rays = circumference as usize;
    let step_mult = 1.3f32;
    let steps = (sight_range * step_mult) as usize;

    let u_position = round_point2(*position);
    let current_height = map_points[u_position.0][u_position.1].height;
    let mut max_height_seen = current_height / 8 * 8 + 7;

    if current_height % 8 >= 4 {
        max_height_seen += 8;
    }

    for index in 0..rays {
        let angle = index as f32 / rays as f32 * 2f32 * std::f32::consts::PI;
        let v_x = angle.sin();
        let v_y = angle.cos();

        for step in 0..steps {
            // Rays are only drawn until non-walkable is found and thus the cannot go out of bounds
            let step_f32 = step as f32 / step_mult;
            let new_pos =
                ((position.0 + v_x * step_f32) as usize, (position.1 + v_y * step_f32) as usize);

            // TODO: Same for height difference
            if map_points[new_pos.0][new_pos.1].height > max_height_seen {
                // Ray can't reach further
                break;
            }

            if matches!(points[new_pos.0][new_pos.1], VisionStatus::NotSeen) {
                points[new_pos.0][new_pos.1] = VisionStatus::Seen;
            }
        }
    }
}
