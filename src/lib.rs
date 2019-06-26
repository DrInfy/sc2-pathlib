use pyo3::prelude::*;
//use pyo3::wrap_pyfunction;
use pathfinding::prelude::{astar, dijkstra_all};
mod pos;



#[pyclass]
pub struct PathFind {
    map: Vec<Vec<usize>>
}

#[pymethods]
impl PathFind {
    #[new]
    fn new(obj: &PyRawObject, map: Vec<Vec<usize>>) {
        obj.init(PathFind { map})
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
  
    /// Find the path using influence values and returns the path and distance
    fn find_path(&self, start: (usize, usize), end: (usize, usize), 
            possible_influence: Option<bool>, possible_heuristic: Option<u8>) -> PyResult<(Vec<(usize, usize)>, f64)> {
        let start: pos::Pos = pos::Pos(start.0, start.1);
        let goal: pos::Pos = pos::Pos(end.0, end.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let use_influence = possible_influence.unwrap_or(true);


        let method: Box<Fn(&pos::Pos) -> Vec<(pos::Pos, usize)>> = match use_influence {
            true => Box::new(|p: &pos::Pos| p.successors(&grid)),
            _ => Box::new(|p: &pos::Pos| p.successors_no_influence(&grid))
        };

        let heuristic: Box<Fn(&pos::Pos)->usize> = match possible_heuristic.unwrap_or(0) {
            0 => Box::new(|p: &pos::Pos| p.manhattan_distance(&goal)),
            1 => Box::new(|p: &pos::Pos| p.quick_distance(&goal)),
            _ => Box::new(|p: &pos::Pos| p.euclidean_distance(&goal)),
        };

        let result = astar(&start, method, heuristic, |p| *p == goal);
        
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
        let result = dijkstra_all(&start, |p| p.successors_no_influence(&grid));

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