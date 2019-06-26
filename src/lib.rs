use pyo3::prelude::*;
//use pyo3::wrap_pyfunction;
use pathfinding::prelude::{absdiff, astar, dijkstra_all};
use std::cmp::{min, max};
mod pos;



#[pyclass]
pub struct PathFind {
    map: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    normal_influence: usize
}

fn quick_distance(first: (usize, usize), other: (usize, usize)) -> usize {
    let xd = absdiff(first.0, other.0);
    let yd = absdiff(first.1, other.1);
    let diag = min(xd, yd);
    (xd + yd - diag - diag) * pos::MULT + diag * pos::SQRT2
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
    fn get_normal_influenceidth(&self)-> PyResult<usize>{
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

    // Creates a building on the grid that is not pathable
    // Position = center of building
    fn create_building(&mut self, position: (f32,f32), size: (usize, usize))
    {
        let mut grid: Vec<Vec<usize>> = self.map.clone();
        
        let pos_x: f32 = position.0;
        let pos_y: f32 = position.1;

        let w: usize = size.0;
        let h: usize = size.1;
        let w_start: usize;
        let h_start: usize;

        w_start = (pos_x as f32 - (w as f32 / 2 as f32)).ceil() as usize;
        h_start = (pos_y as f32 - (h as f32 / 2 as f32)).ceil() as usize;

        for x in w_start..(w+w_start) {
            for y in h_start..(h+h_start){
                grid[x][y] = 0;
            }
        }
        self.map = grid;
    }

    fn normalize_influence(&mut self, value: usize) {
        let height = self.map[0].len();
        self.normal_influence = value;
        
        for x in 0..self.map.len() {
            for y in 0..height {
                if self.map[x][y] > 0 {
                    self.map[x][y] = value;
                }
            }
        }
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

                let distance = quick_distance((x,y), target_pos);
                
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

        let heuristic: Box<dyn Fn(&pos::Pos)->usize> = match possible_heuristic.unwrap_or(0) {
            0 => Box::new(|p: &pos::Pos| p.manhattan_distance(&goal)),
            1 => Box::new(|p: &pos::Pos| p.quick_distance(&goal)),
            _ => Box::new(|p: &pos::Pos| p.euclidean_distance(&goal)),
        };

        let result = astar(&start, |p| p.successors(grid), heuristic, |p| *p == goal);
        
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
        let inf = self.normal_influence;
        // let heuristic: Box<dyn Fn(&pos::InfluencedPos)->usize> = match possible_heuristic.unwrap_or(0) {
        //     0 => Box::new(|p: &pos::InfluencedPos| p.manhattan_distance(&goal, self.normal_influence)),
        //     1 => Box::new(|p: &pos::InfluencedPos| p.quick_distance(&goal, self.normal_influence)),
        //     _ => Box::new(|p: &pos::InfluencedPos| p.euclidean_distance(&goal, self.normal_influence)),
        // };

        let result = astar(&start, |p| p.successors(grid), |p| p.manhattan_distance(&goal, inf), |p| *p == goal);
        
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
}


/// This module is a python module implemented in Rust.
#[pymodule]
fn sc2pathlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PathFind>()?;
    Ok(())
}