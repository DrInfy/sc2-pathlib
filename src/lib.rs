use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pathfinding::prelude::{absdiff, astar, dijkstra_all};
use std::time::Instant;
use std::collections::HashMap;
use std::cmp::min;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);
//static SQRT2: f32 = 1.4142135623730950488016887242097;
static SQRT2: usize = 14142;
static MULT: usize = 10000;
static MULTf64: f64 = 10000.0;

impl Pos {
    fn distance(&self, other: &Pos) -> usize {
        //((((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)) as f64).sqrt() * MULTf64) as usize 

        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) * MULT

        //let xd = absdiff(self.0, other.0);
        //let yd = absdiff(self.1, other.1);
        //let diag = min(xd, yd);
        //(xd + yd - diag - diag) * MULT + diag * SQRT2
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

        if val_left > 0
        {
            arr.push((Pos(x - 1, y), val_left * MULT));

            if val_down > 0
            {
                let diag_val = grid[x - 1][y - 1];

                if diag_val > 0 {
                    arr.push((Pos(x - 1, y - 1), diag_val * SQRT2));
                }
            }

            if val_up > 0
            {
                let diag_val = grid[x - 1][y + 1];

                if diag_val > 0 {
                    arr.push((Pos(x - 1, y + 1), diag_val * SQRT2));
                }
            }
        }

        if val_right > 0
        {
            arr.push((Pos(x + 1, y), val_right * MULT));

            if val_down > 0
            {
                let diag_val = grid[x + 1][y - 1];

                if diag_val > 0 {
                    arr.push((Pos(x + 1, y - 1), diag_val * SQRT2));
                }
            }

            if val_up > 0
            {
                let diag_val = grid[x + 1][y + 1];

                if diag_val > 0 {
                    arr.push((Pos(x + 1, y + 1), diag_val * SQRT2));
                }
            }
        }

        if val_up > 0
        {
            arr.push((Pos(x, y + 1), val_up * MULT));
        }

        if val_down > 0
        {
            arr.push((Pos(x, y - 1), val_down * MULT));
        }

        arr
    }
}

#[pyclass]
pub struct PathFind {
    map: Vec<Vec<usize>>,
}

#[pymethods]
impl PathFind {
    #[new]
    fn new(obj: &PyRawObject, map: Vec<Vec<usize>>) {
        obj.init(PathFind { map })
    }

    /// Tests a path and returns a string defining the tested path
    fn debug_path(&mut self, start: (usize, usize), end: (usize, usize)) -> PyResult<String> {
        let start: Pos = Pos(start.0, start.1);
        let goal: Pos = Pos(end.0, end.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        
        let mut dict = HashMap::<Pos,Vec<(Pos, usize)>>::new();
        let height = grid[0].len();

        for x in 0..grid.len() {
        for y in 0..height {
            let temp = Pos(x, y);
            let successors = temp.successors(&grid);
            dict.insert(temp, successors);
        }
        }
        let test = dict.get(&start).unwrap();

        let now = Instant::now();
        let result = astar(&start, |p| dict.get(&p).unwrap().clone(), |p| p.distance(&goal), |p| *p == goal);
        let time_taken = now.elapsed().as_micros() as f32 / 1000.0;

        let unwrapped = result.unwrap();
        let path = unwrapped.0;
        let distance = unwrapped.1;

        let steps = path.len().to_string();
        let mut path_text = String::new();
        for step in path {
            path_text.push_str(&format!("{},{} ", step.0, step.1));
        }
        Ok(format!("time taken: {} ms len: {} distance: {} start: {},{} goal: {},{} Path: {}", time_taken, steps, distance, start.0, start.1, end.0, end.1, &path_text))
    }

    /// Find the path and returns the path
    fn find_path(&mut self,start: (usize, usize), end: (usize, usize)) -> PyResult<(Vec<(usize, usize)>, f64)> {
        let start: Pos = Pos(start.0, start.1);
        let goal: Pos = Pos(end.0, end.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        
        let result = astar(&start, |p| p.successors(&grid), |p| p.distance(&goal), |p| *p == goal);
        
        let mut path: Vec<(usize, usize)>;
        let distance: f64;

        if result.is_none(){
            path = Vec::<(usize, usize)>::new();
            distance = 0.0
        }
        else {
            let unwrapped = result.unwrap();
            distance = (unwrapped.1 as f64) / MULTf64;
            path = Vec::<(usize, usize)>::with_capacity(unwrapped.0.len());
            for pos in unwrapped.0 {
                path.push((pos.0, pos.1))    
            }
        }
        
        Ok((path, distance))
    }
}

#[pyfunction]
fn debug_all_paths(grid: Vec<Vec<usize>>, start: (usize, usize)) -> PyResult<String> {
    let start: Pos = Pos(start.0, start.1);
    let now = Instant::now();

    let result = dijkstra_all(&start, |p| p.successors(&grid));
    let time_taken = now.elapsed().as_micros() as f32 / 1000.0;

    Ok(format!("time taken: {} ms to solve all paths to point {},{}.", time_taken, start.0, start.1))

}

/// This module is a python module implemented in Rust.
#[pymodule]
fn sc2pathlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PathFind>()?;
    m.add_wrapped(wrap_pyfunction!(debug_all_paths))?;

    Ok(())
}