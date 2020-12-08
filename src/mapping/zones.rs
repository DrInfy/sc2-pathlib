use pyo3::prelude::*;
use std::collections::HashSet;

use crate::path_find::euclidean_distance;

use super::{map::Map, map_point::MapPoint};

const DIFFERENCE: usize = 12;
const Y_MULT: usize = 1000000;

#[pymethods]
impl Map {
    pub fn calculate_zones(&mut self, sorted_base_locations: Vec<(f32, f32)>) {
        let mut index = 1;
        let copy_loc = sorted_base_locations.clone();

        for pos in sorted_base_locations {
            let x = pos.0 as usize;
            let y = pos.1 as usize;

            let target_height = self.points[x][y].height;
            // self.points[x][y].zone_index = index;
            flood_fill(self, x, y, target_height, index, pos, &copy_loc);
            index += 1;
        }
    }

    pub fn draw_zones(&self) -> Vec<Vec<usize>> {
        let width = self.ground_pathing.map.len();
        let height = self.ground_pathing.map[0].len();
        let mut walk_map: Vec<Vec<usize>> = vec![vec![0; height]; width];

        for x in 0..width {
            for y in 0..height {
                let point = &self.points[x][y];
                if point.walkable {
                    walk_map[x][y] = 255;
                    if point.zone_index > 0 {
                        walk_map[x][y] = 50 + 20 * (point.zone_index as usize);
                    }
                } else if point.is_choke {
                    walk_map[x][y] = 0;
                }
            }
        }

        walk_map
        
    }
}

impl Map {
    fn borrow(&mut self, x: usize, y: usize) -> &mut MapPoint { return &mut self.points[x][y]; }
}

fn flood_fill(map: &mut Map,
              x: usize,
              y: usize,
              target_height: usize,
              zone_index: i8,
              origin: (f32, f32),
              sorted_base_locations: &Vec<(f32, f32)>) {

    let zone = map.borrow(x, y).zone_index as usize;
    if zone == zone_index as usize || !map.borrow(x, y).walkable {
        return;
    }
    else if zone > 0 {
        let start = (x, y);
        let pos = sorted_base_locations[zone - 1];
        let end = (pos.0 as usize, pos.1 as usize);
        let end2 = (origin.0 as usize, origin.1 as usize);
        let heuristic = Some(2u8);

        let mapping = map.get_map_mut(1);
        let old_path = mapping.find_path(start, end, heuristic);
        let new_path = mapping.find_path(start, end2, heuristic);

        if old_path.1 < new_path.1 {
            // Old zone definition is better
            return;
        }
    }

    let mut point = map.borrow(x, y);

    
    point.zone_index = zone_index;
    
    if target_height > point.height + DIFFERENCE || target_height < point.height - DIFFERENCE {
        return; // Not the same zone anymore.
    }
    
    if point.is_choke {
        // Let's color the first grid as being the same zone
        return; // do not flood fill any of the following grid pixels.
    }

    if euclidean_distance(origin, (x as f32, y as f32)) > 30f32 {
        // Too far to look for other grids.
        return;
    }

    // if points[x][y].overlord_spot == target {
    if y > 0 {
        flood_fill(map, x, ((y as u32) - 1) as usize, target_height, zone_index, origin, sorted_base_locations);
    }
    if x > 0 {
        flood_fill(map, ((x as u32) - 1) as usize, y, target_height, zone_index, origin, sorted_base_locations);
    }
    if y < map.points[0].len() - 1 {
        flood_fill(map, x, y + 1, target_height, zone_index, origin, sorted_base_locations);
    }
    if x < map.points.len() - 1 {
        flood_fill(map, x + 1, y, target_height, zone_index, origin, sorted_base_locations);
    }
    // }

    return;
}
