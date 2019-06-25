use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pathfinding::prelude::{absdiff, astar, dijkstra_all};
use std::time::Instant;
use std::cmp::min;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);
//static SQRT2: f32 = 1.4142135623730950488016887242097;
static SQRT2: usize = 14142;
static MULT: usize = 10000;
static MULTF64: f64 = 10000.0;

impl Pos {
    fn manhattan_distance(&self, other: &Pos) -> usize {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) * MULT
    }

    fn euclidean_distance(&self, other: &Pos) -> usize {
        ((((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)) as f64).sqrt() * MULTF64) as usize
    }

    fn quick_distance(&self, other: &Pos) -> usize {
        let xd = absdiff(self.0, other.0);
        let yd = absdiff(self.1, other.1);
        let diag = min(xd, yd);
        (xd + yd - diag - diag) * MULT + diag * SQRT2
    }

    fn successors(&self, grid: &Vec<Vec<usize>>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut arr = Vec::<(Pos, usize)>::with_capacity(8);
        //let arr = Vec<(Pos, f32)>();

        let mut val_left: usize = 0;
        let mut val_down: usize = 0;
        let mut val_right: usize = 0;
        let mut val_up: usize = 0;

        if x > 0 {
            val_left =  grid[x - 1][y];
        }

        if y > 0 {
            val_down = grid[x][y - 1];
        }

        if x + 1 < grid.len() {
            val_right = grid[x + 1][y];
        }

        if y + 1 < grid[0].len() {
            val_up = grid[x][y + 1];
        }

        if val_left > 0 {
            arr.push((Pos(x - 1, y), val_left * MULT));

            if val_down > 0 {
                let diag_val = grid[x - 1][y - 1];

                if diag_val > 0 {
                    arr.push((Pos(x - 1, y - 1), diag_val * SQRT2));
                }
            }

            if val_up > 0 {
                let diag_val = grid[x - 1][y + 1];

                if diag_val > 0 {
                    arr.push((Pos(x - 1, y + 1), diag_val * SQRT2));
                }
            }
        }

        if val_right > 0 {
            arr.push((Pos(x + 1, y), val_right * MULT));

            if val_down > 0 {
                let diag_val = grid[x + 1][y - 1];

                if diag_val > 0 {
                    arr.push((Pos(x + 1, y - 1), diag_val * SQRT2));
                }
            }

            if val_up > 0 {
                let diag_val = grid[x + 1][y + 1];

                if diag_val > 0 {
                    arr.push((Pos(x + 1, y + 1), diag_val * SQRT2));
                }
            }
        }

        if val_up > 0 {
            arr.push((Pos(x, y + 1), val_up * MULT));
        }

        if val_down > 0 {
            arr.push((Pos(x, y - 1), val_down * MULT));
        }

        arr
    }

    fn successors_no_influence(&self, grid: &Vec<Vec<usize>>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut arr = Vec::<(Pos, usize)>::with_capacity(8);
        //let arr = Vec<(Pos, f32)>();

        let mut val_left: usize = 0;
        let mut val_down: usize = 0;
        let mut val_right: usize = 0;
        let mut val_up: usize = 0;

        if x > 0 {
            val_left =  grid[x - 1][y];
        }

        if y > 0 {
            val_down = grid[x][y - 1];
        }

        if x + 1 < grid.len() {
            val_right = grid[x + 1][y];
        }

        if y + 1 < grid[0].len() {
            val_up = grid[x][y + 1];
        }

        if val_left > 0 {
            arr.push((Pos(x - 1, y), MULT));

            if val_down > 0 {
                let diag_val = grid[x - 1][y - 1];

                if diag_val > 0 {
                    arr.push((Pos(x - 1, y - 1), SQRT2));
                }
            }

            if val_up > 0 {
                let diag_val = grid[x - 1][y + 1];

                if diag_val > 0 {
                    arr.push((Pos(x - 1, y + 1), SQRT2));
                }
            }
        }

        if val_right > 0 {
            arr.push((Pos(x + 1, y), val_right * MULT));

            if val_down > 0 {
                let diag_val = grid[x + 1][y - 1];

                if diag_val > 0 {
                    arr.push((Pos(x + 1, y - 1), SQRT2));
                }
            }

            if val_up > 0 {
                let diag_val = grid[x + 1][y + 1];

                if diag_val > 0 {
                    arr.push((Pos(x + 1, y + 1), SQRT2));
                }
            }
        }

        if val_up > 0 {
            arr.push((Pos(x, y + 1), MULT));
        }

        if val_down > 0 {
            arr.push((Pos(x, y - 1), MULT));
        }

        arr
    }
}

#[pyclass]
pub struct PathFind {
    map: Vec<Vec<usize>>,
    dict: Vec<Vec<Vec<(Pos, usize)>>>
}

#[pymethods]
impl PathFind {
    #[new]
    fn new(obj: &PyRawObject, map: Vec<Vec<usize>>) {
        let mut dict : Vec<Vec<Vec<(Pos, usize)>>> = Vec::<Vec<Vec<(Pos, usize)>>>::new();
        let height = map[0].len();

        for x in 0..map.len() {
            let mut column = Vec::<Vec<(Pos, usize)>>::new();

            for y in 0..height {
                let temp = Pos(x, y);
                let successors = temp.successors(&map);
                column.push(successors)
            }

            dict.push(column);
        }

        obj.init(PathFind { map, dict })
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

    /// Find the path using influence values and returns the path and distance
    fn find_path(&self, start: (usize, usize), end: (usize, usize), 
            possible_influence: Option<bool>, possible_heuristic: Option<u8>) -> PyResult<(Vec<(usize, usize)>, f64)> {
        let start: Pos = Pos(start.0, start.1);
        let goal: Pos = Pos(end.0, end.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let dict: &Vec<Vec<Vec<(Pos, usize)>>> = &self.dict;
        let use_influence = possible_influence.unwrap_or(true);


        let method: Box<Fn(&Pos) -> Vec<(Pos, usize)>> = match use_influence {
            true => Box::new(|p: &Pos| p.successors(&grid)),
            _ => Box::new(|p: &Pos| p.successors_no_influence(&grid))
        };

        let heuristic: Box<Fn(&Pos)->usize> = match possible_heuristic.unwrap_or(0) {
            0 => Box::new(|p: &Pos| p.manhattan_distance(&goal)),
            1 => Box::new(|p: &Pos| p.quick_distance(&goal)),
            _ => Box::new(|p: &Pos| p.euclidean_distance(&goal)),
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
            distance = (unwrapped.1 as f64) / MULTF64;
            path = Vec::<(usize, usize)>::with_capacity(unwrapped.0.len());
            for pos in unwrapped.0 {
                path.push((pos.0, pos.1))    
            }
        }
        
        Ok((path, distance))
    }

    /// Finds all reachable destinations from selected start point. Ignores influence.
    fn find_all_destinations(&self, start: (usize, usize)) -> PyResult<Vec<((usize, usize), f64)>> {
        let start: Pos = Pos(start.0, start.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let result = dijkstra_all(&start, |p| p.successors_no_influence(&grid));

        let mut destination_collection: Vec<((usize, usize), f64)> = Vec::<((usize, usize), f64)>::with_capacity(result.len());
        let distance: f64;

        for found_path in result {
            let x = ((found_path.1).0).0;
            let y = ((found_path.1).0).1;
            let d = ((found_path.1).1 as f64) / MULTF64;
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