use pyo3::prelude::*;
//use pyo3::wrap_pyfunction;
use pathfinding::prelude::{absdiff, astar, dijkstra_all, dijkstra_partial};
use std::cmp::{min, max};
mod pos;
mod pos_large;
mod rectangle;



#[pyclass]
pub struct PathFind {
    map: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    normal_influence: usize
}

#[inline]
pub fn octile_distance(first: (usize, usize), other: (usize, usize)) -> usize{
    let dx = absdiff(first.0, other.0);
    let dy = absdiff(first.1, other.1);

    if dx > dy{
        pos::MULT * dx + pos::DIAGONAL_MINUS_CARDINAL * dy
    }
    else{
        pos::MULT * dy + pos::DIAGONAL_MINUS_CARDINAL * dx
    }
}

#[pymethods]
impl PathFind {
    #[new]
    fn new(obj: &PyRawObject, map: Vec<Vec<usize>>) {
        let width = map.len();
        let height = map[0].len();
        let normal_influence: usize = 1;
        obj.init(PathFind { map, width, height, normal_influence})
    }

    // object.width
    #[getter(width)]
    fn get_width(&self)-> PyResult<usize>{
        Ok(self.width)
    }

    // object.height
    #[getter(height)]
    fn get_height(&self)-> PyResult<usize>{
        Ok(self.height)
    }

    // object.normal_influence
    #[getter(normal_influence)]
    fn get_normal_influence(&self)-> PyResult<usize>{
        Ok(self.normal_influence)
    }

    // object.map
    #[getter(map)]
    fn get_map(&self)-> PyResult<Vec<Vec<usize>>>{
        Ok(self.map.clone())
    }

    // object.map(2dArray)
    #[setter(map)]
    fn set_map(&mut self, value:Vec<Vec<usize>>) -> PyResult<()> {
        self.map = value;
        Ok(())
    }

    // Creates a block on the grid that is not pathable
    // center = center of building
    fn create_block(&mut self, center: (f32,f32), size: (usize, usize)) {
        
        let rect = rectangle::Rectangle::init_from_center(center, size, self.width, self.height);

        for x in rect.x..rect.x_end {
            for y in rect.y..rect.y_end{
                self.map[x][y] = 0;
            }
        }
    }

    // Creates a block on the grid that is not pathable
    // center = center of building
    fn create_blocks(&mut self, centers: Vec<(f32,f32)>, size: (usize, usize)) {
        
        for center in centers {
            let rect = rectangle::Rectangle::init_from_center(center, size, self.width, self.height);

            for x in rect.x..rect.x_end {
                for y in rect.y..rect.y_end{
                    self.map[x][y] = 0;
                }
            }
        }
    }

    // Removes a block on the grid and makes it pathable
    // center = center of block
    fn remove_block(&mut self, center: (f32,f32), size: (usize, usize)) {
        let rect = rectangle::Rectangle::init_from_center(center, size, self.width, self.height);

        for x in rect.x..rect.x_end {
            for y in rect.y..rect.y_end{
                self.map[x][y] = self.normal_influence;
            }
        }
    }

    // Removes multiple blocks on the grid and makes it pathable
    // center = center of block
    fn remove_blocks(&mut self, centers: Vec<(f32,f32)>, size: (usize, usize)) {
        
        for center in centers {
            let rect = rectangle::Rectangle::init_from_center(center, size, self.width, self.height);

            for x in rect.x..rect.x_end {
                for y in rect.y..rect.y_end{
                    self.map[x][y] = self.normal_influence;
                }
            }
        }
    }

    fn normalize_influence(&mut self, value: usize) {
        self.normal_influence = value;
        
        for x in 0..self.width {
            for y in 0..self.height {
                if self.map[x][y] > 0 {
                    self.map[x][y] = value;
                }
            }
        }
    }

    /// Adds influence based on euclidean distance
    fn add_influence(&mut self, positions: Vec<(usize, usize)>, max: f32, distance: f32) -> PyResult<()> {
        let mult = 1.0 / (distance * pos::MULTF64 as f32);

        for x in 0..self.width {
            for y in 0..self.height {
                if self.map[x][y] > 0 {
                    let mut added_value: usize = 0;

                    for position in &positions {
                        let value = max * (1.0 - (octile_distance(*position, (x, y)) as f32) * mult);
                        if value > 0.0 {
                            added_value += value as usize;
                        }
                    }

                    self.map[x][y] += added_value;
                }
            }
        }
        
        Ok(())
    }

    /// Adds influence based on euclidean distance
    fn add_influence_flat(&mut self, positions: Vec<(usize, usize)>, max: f32, distance: f32) -> PyResult<()> {
        let value = max as usize;
        let mult_distance = distance * pos::MULTF64 as f32;

        for x in 0..self.width {
            for y in 0..self.height {
                if self.map[x][y] > 0 {
                    let mut added_value: usize = 0;

                    for position in &positions {
                        if (octile_distance(*position, (x, y)) as f32) < mult_distance {
                            added_value += value;
                        }
                    }

                    self.map[x][y] += added_value;
                }
            }
        }
        
        Ok(())
    }

    /// Adds influence based on walk distance
    fn add_walk_influence(&mut self, positions: Vec<(usize, usize)>, max: f64, distance: f64) -> PyResult<()> {
        let mult = 1.0 / distance;
        let max_int = max as usize;

        for position in &positions {

            if self.map[position.0][position.1] == 0 {
                continue;   
            }

            let destinations = self.find_destinations_in_inline(*position, distance);
            self.map[position.0][position.1] += max_int;

            for destination in destinations {
                let end_point = destination.0;
                let current_distance = destination.1;
                let value = max * (1.0 - current_distance * mult);

                if current_distance < distance {
                    self.map[end_point.0][end_point.1] += value as usize
                }
            }
        }
        
        Ok(())
    }

    /// Adds influence based on walk distance
    fn add_walk_influence_flat(&mut self, positions: Vec<(usize, usize)>, max: f64, distance: f64) -> PyResult<()> {
        let max_int = max as usize;

        for position in &positions {
            if self.map[position.0][position.1] == 0 {
                continue;   
            }

            let destinations = self.find_destinations_in_inline(*position, distance);

            
            self.map[position.0][position.1] += max_int;

            for destination in destinations {
                let end_point = destination.0;
                self.map[end_point.0][end_point.1] += max_int

                // let current_distance = destination.1;
                // // let value = max * (1.0 - current_distance * mult);

                // if current_distance < distance {
                // }
            }
        }
        
        Ok(())
    }

    /// Finds the first reachable position within specified walking distance from the center point with lowest value
    fn lowest_influence_walk(&self, center: (usize, usize), distance: f64) -> PyResult<((usize, usize), f64)> {
        Ok(self.lowest_influence_walk_inline(center, distance))
    }

    #[inline]
    fn lowest_influence_walk_inline(&self, center: (usize, usize), distance: f64) -> ((usize, usize), f64) {
        let destinations = self.find_destinations_in_inline(center, distance);

        let mut min_value = std::usize::MAX;
        let mut min_distance = std::f64::MAX;
        let mut min_position = center.clone();

        for destination in destinations {
            let pos = destination.0;
            let new_val = self.map[pos.0][pos.1];
            if new_val == 0 { continue; }

            let distance = destination.1;
            
            if new_val < min_value || (new_val == min_value && distance < min_distance) {
                min_value = new_val;
                min_distance = distance;
                min_position = pos;
            }
        }
        
        (min_position, min_distance)
    }
    

    /// Finds the first reachable position within specified distance from the center point with lowest value
    fn lowest_influence(&self, center: (f32, f32), distance: usize) -> PyResult<((usize, usize), f64)> {
        Ok(self.inline_lowest_value(center, distance))
    }

    #[inline]
    fn inline_lowest_value(&self, center: (f32, f32), distance: usize) -> ((usize, usize), f64) {
        let pos_x: usize = center.0 as usize;
        let pos_y: usize = center.1 as usize;

        let w: usize = distance;
        let h: usize = distance;
        let w_start: usize = max(0, (pos_x as f32 - (w as f32 / 2 as f32)).ceil() as usize);
        let h_start: usize = max(0, (pos_y as f32 - (h as f32 / 2 as f32)).ceil() as usize);
        let w_end: usize = min(self.width, w + w_start);
        let h_end: usize = min(self.height, h + h_start);

        let mut min_value = std::usize::MAX;
        let mut min_distance = std::usize::MAX;
        let mut min_position = (pos_x, pos_y);
        let target_pos = (pos_x, pos_y);

        for x in w_start..w_end {
            for y in h_start..h_end{
                let new_val = self.map[x][y];
                if new_val == 0 { continue; }

                let distance = octile_distance((x,y), target_pos);
                
                if new_val < min_value || (new_val == min_value && distance < min_distance) {
                    min_value = new_val;
                    min_distance = distance;
                    min_position = (x,y);
                }
            }
        }

        (min_position, min_distance as f64 / pos::MULTF64)
    }
    
     /// Find the shortest path values without considering influence and returns the path and distance
    fn find_path(&self, start: (usize, usize), end: (usize, usize), possible_heuristic: Option<u8>) -> PyResult<(Vec<(usize, usize)>, f64)> {
        let start: pos::Pos = pos::Pos(start.0, start.1);
        let goal: pos::Pos = pos::Pos(end.0, end.1);
        let grid: &Vec<Vec<usize>> = &self.map;

        let result: Option<(Vec<pos::Pos>, usize)>;
        match possible_heuristic.unwrap_or(0) {
            0 => result = astar(&start, |p| p.successors(grid), |p| p.manhattan_distance(&goal), |p| *p == goal),
            1 => result = astar(&start, |p| p.successors(grid), |p| p.octile_distance(&goal),  |p| *p == goal),
            _ => result = astar(&start, |p| p.successors(grid), |p| p.euclidean_distance(&goal),  |p| *p == goal),
        };
        
        let mut path: Vec<(usize, usize)>;
        let distance: f64;

        if result.is_none(){
            path = Vec::<(usize, usize)>::new();
            distance = 0.0
        }
        else {
            let unwrapped = result.unwrap();
            distance = (unwrapped.1 as f64) / pos::MULTF64;
            path = Vec::<(usize, usize)>::with_capacity(unwrapped.0.len());
            for pos in unwrapped.0 {
                path.push((pos.0, pos.1))    
            }
        }
        
        Ok((path, distance))
    }

    /// Find the shortest path values without considering influence and returns the path and distance
    fn find_path_large(&self, start: (usize, usize), end: (usize, usize), possible_heuristic: Option<u8>) -> PyResult<(Vec<(usize, usize)>, f64)> {
        let start: pos_large::PosLarge = pos_large::PosLarge(start.0, start.1);
        let goal: pos_large::PosLarge = pos_large::PosLarge(end.0, end.1);
        let grid: &Vec<Vec<usize>> = &self.map;

        let result: Option<(Vec<pos_large::PosLarge>, usize)>;
        match possible_heuristic.unwrap_or(0) {
            0 => result = astar(&start, |p| p.successors(grid), |p| p.manhattan_distance(&goal), |p| *p == goal),
            1 => result = astar(&start, |p| p.successors(grid), |p| p.octile_distance(&goal),  |p| *p == goal),
            _ => result = astar(&start, |p| p.successors(grid), |p| p.euclidean_distance(&goal),  |p| *p == goal),
        };
        
        let mut path: Vec<(usize, usize)>;
        let distance: f64;

        if result.is_none(){
            path = Vec::<(usize, usize)>::new();
            distance = 0.0
        }
        else {
            let unwrapped = result.unwrap();
            distance = (unwrapped.1 as f64) / pos::MULTF64;
            path = Vec::<(usize, usize)>::with_capacity(unwrapped.0.len());
            for pos in unwrapped.0 {
                path.push((pos.0, pos.1))    
            }
        }
        
        Ok((path, distance))
    }

    /// Find the path using influence values and returns the path and distance
    fn find_path_influence(&self, start: (usize, usize), end: (usize, usize), possible_heuristic: Option<u8>) -> PyResult<(Vec<(usize, usize)>, f64)> {
        let start = pos::InfluencedPos(start.0, start.1);
        let goal = pos::InfluencedPos(end.0, end.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let infl = self.normal_influence;

        let result: Option<(Vec<pos::InfluencedPos>, usize)>;

        match possible_heuristic.unwrap_or(0) {
            0 => result = astar(&start, |p| p.successors(grid), |p| p.manhattan_distance(&goal, infl), |p| *p == goal),
            1 => result = astar(&start, |p| p.successors(grid), |p| p.octile_distance(&goal, infl),  |p| *p == goal),
            _ => result = astar(&start, |p| p.successors(grid), |p| p.euclidean_distance(&goal, infl),  |p| *p == goal),
        };
        
        let mut path: Vec<(usize, usize)>;
        let distance: f64;

        if result.is_none(){
            path = Vec::<(usize, usize)>::new();
            distance = 0.0
        }
        else {
            let unwrapped = result.unwrap();
            distance = (unwrapped.1 as f64) / pos::MULTF64;
            path = Vec::<(usize, usize)>::with_capacity(unwrapped.0.len());
            for pos in unwrapped.0 {
                path.push((pos.0, pos.1))    
            }
        }
        
        Ok((path, distance))
    }

    /// Find the path using influence values and returns the path and distance
    fn find_path_influence_large(&self, start: (usize, usize), end: (usize, usize), possible_heuristic: Option<u8>) -> PyResult<(Vec<(usize, usize)>, f64)> {
        let start = pos_large::InfluencedPosLarge(start.0, start.1);
        let goal = pos_large::InfluencedPosLarge(end.0, end.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let infl = self.normal_influence;

        let result: Option<(Vec<pos_large::InfluencedPosLarge>, usize)>;

        match possible_heuristic.unwrap_or(0) {
            0 => result = astar(&start, |p| p.successors(grid), |p| p.manhattan_distance(&goal, infl), |p| *p == goal),
            1 => result = astar(&start, |p| p.successors(grid), |p| p.octile_distance(&goal, infl),  |p| *p == goal),
            _ => result = astar(&start, |p| p.successors(grid), |p| p.euclidean_distance(&goal, infl),  |p| *p == goal),
        };
        
        let mut path: Vec<(usize, usize)>;
        let distance: f64;

        if result.is_none(){
            path = Vec::<(usize, usize)>::new();
            distance = 0.0
        }
        else {
            let unwrapped = result.unwrap();
            distance = (unwrapped.1 as f64) / pos::MULTF64;
            path = Vec::<(usize, usize)>::with_capacity(unwrapped.0.len());
            for pos in unwrapped.0 {
                path.push((pos.0, pos.1))    
            }
        }
        
        Ok((path, distance))
    }

    /// Finds all reachable destinations from selected start point. Ignores influence.
    fn find_all_destinations(&self, start: (usize, usize)) -> PyResult<Vec<((usize, usize), f64)>> {
        let start: pos::Pos = pos::Pos(start.0, start.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let result = dijkstra_all(&start, |p| p.successors(&grid));

        let mut destination_collection: Vec<((usize, usize), f64)> = Vec::<((usize, usize), f64)>::with_capacity(result.len());

        for found_path in result {
            let x = ((found_path.1).0).0;
            let y = ((found_path.1).0).1;
            let d = ((found_path.1).1 as f64) / pos::MULTF64;
            destination_collection.push(((x, y), d));
        }

        Ok(destination_collection)
    }

    /// Finds all reachable destinations from selected start point. Ignores influence.
    fn find_destinations_in(&self, start: (usize, usize), distance: f64 ) -> PyResult<Vec<((usize, usize), f64)>> {
        Ok(self.find_destinations_in_inline(start, distance))
    }

    #[inline]
    fn find_destinations_in_inline(&self, start: (usize, usize), distance: f64 ) -> Vec<((usize, usize), f64)> {
        let start: pos::Pos = pos::Pos(start.0, start.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let u_distance =  (distance * pos::MULTF64) as usize;

        let result = dijkstra_partial(&start, |p| p.successors(&grid), |p| p.octile_distance(&start) > u_distance);

        let hash_map = result.0;
        let mut destination_collection: Vec<((usize, usize), f64)> = Vec::<((usize, usize), f64)>::with_capacity(hash_map.len());

        for found_path in hash_map {
            let x = (found_path.0).0;
            let y = (found_path.0).1;
            //let x = ((found_path.1).0).0;
            //let y = ((found_path.1).0).1;
            let d = ((found_path.1).1 as f64) / pos::MULTF64;
            destination_collection.push(((x, y), d));
        }

        destination_collection
    }
}


/// This module is a python module implemented in Rust.
#[pymodule]
fn sc2pathlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PathFind>()?;
    Ok(())
}