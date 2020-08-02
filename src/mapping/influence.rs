use crate::path_find::pos::Pos;
use crate::path_find::pos::{self, MULTF64};
use crate::path_find::{octile_distance, rectangle, PathFind};
use pyo3::prelude::*;

// extern crate test;
use std::collections::HashSet;

use super::chokes::{group_chokes, Choke};

const DIFFERENCE: usize = 16;
const Y_MULT: usize = 1000000;
use crate::mapping::map::Map;

const MAPS_PURE_GROUND: usize = 0;
const MAPS_GROUND: usize = 1;
const MAPS_AIR: usize = 2;
const MAPS_BOTH: usize = 3;

#[pymethods]
impl Map {
    pub fn normalize_influence(&mut self, value: usize) {
        self.ground_pathing.normalize_influence(value);
        self.air_pathing.normalize_influence(value);
        self.colossus_pathing.normalize_influence(value);
        self.reaper_pathing.normalize_influence(value);
    }

    pub fn add_influence_walk(&mut self, positions: Vec<(f64, f64)>, influence: f64, distance: f64) {
        let mult = 1.0 / distance;
        let max_int = influence as usize;
        let mut maps = self.get_ground_influence_maps();

        for position_f in &positions {
            let position = (position_f.0.round() as usize, position_f.1.round() as usize);
            if maps[0].map[position.0][position.1] == 0 {
                continue;
            }

            let destinations = maps[0].find_destinations_in_inline(position, distance);
            maps[0].map[position.0][position.1] += max_int;

            for destination in destinations {
                let end_point = destination.0;
                let current_distance = destination.1;
                let value = influence * (1.0 - current_distance * mult);

                if current_distance < distance {
                    for mapping in maps.iter_mut() {
                        mapping.map[end_point.0][end_point.1] += value as usize
                    }
                }
            }
        }
    }

    pub fn add_influence_flat_hollow(&mut self, positions: Vec<(f64, f64)>, influence: f64, min: f64, max: f64) {
        let value = influence as usize;
        let mult_min = min * pos::MULTF64;
        let mult_max = max * pos::MULTF64;
        let mut maps: Vec<&mut PathFind>;
        maps = self.get_ground_influence_maps();

        let diameter = ((max * 2f64) as usize) + 2;
        let rect_size = (diameter, diameter);

        for position_f in &positions {
            let position = (position_f.0.round() as usize, position_f.1.round() as usize);
            let rect = rectangle::Rectangle::init_from_center2(position, rect_size, maps[0].width, maps[0].height);

            for x in rect.x..rect.x_end {
                for y in rect.y..rect.y_end {
                    let d = octile_distance(position, (x, y)) as f64;
                    if d < mult_max && d > mult_min {
                        for mapping in maps.iter_mut() {
                            let old_val = mapping.map[x][y];
                            if old_val > 0 {
                                mapping.map[x][y] = old_val + value;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn add_influence_fading(&mut self,
                                map_type: usize,
                                positions: Vec<(f64, f64)>,
                                influence: f64,
                                min: f64,
                                max: f64) {
        let mult = 1.0 / pos::MULTF64;
        let mult2 = 1.0 / (max - min) ;
        let value = influence as usize;
        let mult_min = min * pos::MULTF64;
        let mult_max = max * pos::MULTF64;
        let mut maps: Vec<&mut PathFind>;

        if map_type == MAPS_PURE_GROUND {
            maps = self.get_pure_ground_influence_maps();
        } else if map_type == MAPS_GROUND {
            maps = self.get_ground_influence_maps();
        } else if map_type == MAPS_AIR {
            maps = self.get_air_influence_maps();
        } else {
            maps = self.get_both_influence_maps();
        }

        let diameter = ((max * 2f64) as usize) + 2;
        let rect_size = (diameter, diameter);

        for position_f in &positions {
            let position = (position_f.0.round() as usize, position_f.1.round() as usize);
            let rect = rectangle::Rectangle::init_from_center2(position, rect_size, maps[0].width, maps[0].height);

            for x in rect.x..rect.x_end {
                for y in rect.y..rect.y_end {
                    let d = octile_distance(position, (x, y)) as f64;
                    if d < mult_max {
                        if d < mult_min {
                            for mapping in maps.iter_mut() {
                                let old_val = mapping.map[x][y];
                                if old_val > 0 {
                                    mapping.map[x][y] = old_val + value;
                                }
                            }
                        } else {
                            // Fading threshold
                            let value_fading = (influence * (1.0 - (d * mult - min) * mult2)) as usize;
                            for mapping in maps.iter_mut() {
                                let old_val = mapping.map[x][y];
                                if old_val > 0 && value_fading > 0{
                                    mapping.map[x][y] = old_val + value_fading;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Map {
    fn get_both_influence_maps(&mut self) -> Vec<&mut PathFind> {
        let mut maps = Vec::<&mut PathFind>::new();
        maps.push(&mut self.ground_pathing);
        maps.push(&mut self.air_pathing);

        if self.influence_colossus_map {
            maps.push(&mut self.colossus_pathing);
        }
        if self.influence_reaper_map {
            maps.push(&mut self.reaper_pathing);
        }

        return maps;
    }

    fn get_pure_ground_influence_maps(&mut self) -> Vec<&mut PathFind> {
        let mut maps = Vec::<&mut PathFind>::new();
        maps.push(&mut self.ground_pathing);

        if self.influence_reaper_map {
            maps.push(&mut self.reaper_pathing);
        }

        return maps;
    }

    fn get_ground_influence_maps(&mut self) -> Vec<&mut PathFind> {
        let mut maps = Vec::<&mut PathFind>::new();
        maps.push(&mut self.ground_pathing);

        if self.influence_colossus_map {
            maps.push(&mut self.colossus_pathing);
        }
        if self.influence_reaper_map {
            maps.push(&mut self.reaper_pathing);
        }

        return maps;
    }

    fn get_air_influence_maps(&mut self) -> Vec<&mut PathFind> {
        let mut maps = Vec::<&mut PathFind>::new();
        maps.push(&mut self.air_pathing);

        if self.influence_colossus_map {
            maps.push(&mut self.colossus_pathing);
        }

        return maps;
    }
}
