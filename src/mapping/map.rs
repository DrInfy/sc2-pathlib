use crate::path_find::PathFind;
use pyo3::prelude::*;
extern crate test;

use crate::mapping::map_point;
use crate::mapping::map_point::Cliff;

#[pyclass]
pub struct Map {
    pub ground_pathing: PathFind,
    pub air_pathing: PathFind,
    pub colossus_pathing: PathFind,
    pub reaper_pathing: PathFind,
    pub points: Vec<Vec<map_point::MapPoint>>,
}

#[cfg(test)]
impl Map {
    pub fn new_internal(pathing: Vec<Vec<usize>>,
                        placement: Vec<Vec<usize>>,
                        height_map: Vec<Vec<usize>>,
                        x_start: usize,
                        y_start: usize,
                        x_end: usize,
                        y_end: usize) -> Self {
        let width = pathing.len();
        let height = pathing[0].len();
        let mut points = vec![vec![map_point::MapPoint::new(); height]; width];

        let mut walk_map = vec![vec![0; height]; width];
        let mut fly_map = vec![vec![0; height]; width];
        let mut reaper_map = vec![vec![0; height]; width];

        for x in 0..width {
            for y in 0..height {
                let walkable = pathing[x][y] > 0 || placement[x][y] > 0;
                let pathable = x_start <= x && x <= x_end && y_start <= y && y <= y_end;
                points[x][y].walkable = walkable;
                points[x][y].pathable = pathable;
                points[x][y].height = height_map[x][y];

                if pathable {
                    fly_map[x][y] = 1;
                }
                if walkable {
                    walk_map[x][y] = 1;
                    reaper_map[x][y] = 1;
                }
            }
        }

        for x in 2..width - 3 {
            for y in 2..height - 3 {
                if !points[x][y].walkable {
                    continue;
                }

                modify_climb(&mut points, x as i32, y as i32, -1, -1);

                modify_climb(&mut points, x as i32, y as i32, -1, 1);
            }
        }

        let ground_pathing = PathFind::new_internal(walk_map);
        let air_pathing = PathFind::new_internal(fly_map);
        let colossus_pathing = PathFind::new_internal(reaper_map.clone());
        let reaper_pathing = PathFind::new_internal(reaper_map);

        Map { ground_pathing,
              air_pathing,
              colossus_pathing,
              reaper_pathing,
              points }
    }
}

#[pymethods]
impl Map {
    #[new]
    fn new(obj: &PyRawObject,
           pathing: Vec<Vec<usize>>,
           placement: Vec<Vec<usize>>,
           height_map: Vec<Vec<usize>>,
           x_start: usize,
           y_start: usize,
           x_end: usize,
           y_end: usize) {
        let width = pathing.len();
        let height = pathing[0].len();
        let mut points = vec![vec![map_point::MapPoint::new(); height]; width];

        let mut walk_map = vec![vec![0; height]; width];
        let mut fly_map = vec![vec![0; height]; width];
        let mut reaper_map = vec![vec![0; height]; width];

        for x in 0..width {
            for y in 0..height {
                let walkable = pathing[x][y] > 0 || placement[x][y] > 0;
                let pathable = x_start <= x && x <= x_end && y_start <= y && y <= y_end;
                points[x][y].walkable = walkable;
                points[x][y].pathable = pathable;
                points[x][y].height = height_map[x][y];

                if pathable {
                    fly_map[x][y] = 1;
                }
                if walkable {
                    walk_map[x][y] = 1;
                    reaper_map[x][y] = 1;
                }
            }
        }

        for x in 2..width - 3 {
            for y in 2..height - 3 {
                if !points[x][y].walkable {
                    continue;
                }

                modify_climb(&mut points, x as i32, y as i32, -1, -1);

                modify_climb(&mut points, x as i32, y as i32, -1, 1);
            }
        }

        let ground_pathing = PathFind::new_internal(walk_map);
        let air_pathing = PathFind::new_internal(fly_map);
        let colossus_pathing = PathFind::new_internal(reaper_map.clone());
        let reaper_pathing = PathFind::new_internal(reaper_map);

        obj.init(Map { ground_pathing,
                       air_pathing,
                       colossus_pathing,
                       reaper_pathing,
                       points })
    }

    #[getter(ground_pathing)]
    fn get_ground_pathing(&self) -> PyResult<Vec<Vec<usize>>> { Ok(self.ground_pathing.map.clone()) }

    fn reset(&mut self) -> PyResult<()> {
        self.ground_pathing.reset_void();
        self.air_pathing.reset_void();
        self.colossus_pathing.reset_void();
        self.reaper_pathing.reset_void();
        Ok(())
    }

    pub fn create_block(&mut self, center: (f32, f32), size: (usize, usize)) {
        self.ground_pathing.create_block(center, size);
        self.colossus_pathing.create_block(center, size);
        self.reaper_pathing.create_block(center, size);
    }

    pub fn create_blocks(&mut self, centers: Vec<(f32, f32)>, size: (usize, usize)) {
        self.ground_pathing.create_blocks_rust(&centers, size);
        self.colossus_pathing.create_blocks_rust(&centers, size);
        self.reaper_pathing.create_blocks_rust(&centers, size);
    }

    pub fn remove_blocks(&mut self, centers: Vec<(f32, f32)>, size: (usize, usize)) {
        self.ground_pathing.remove_blocks_rust(&centers, size);
        self.colossus_pathing.remove_blocks_rust(&centers, size);
        self.reaper_pathing.remove_blocks_rust(&centers, size);
    }
}

fn modify_climb(points: &mut Vec<Vec<map_point::MapPoint>>, x: i32, y: i32, x_dir: i32, y_dir: i32) {
    let x0 = x as usize;
    let y0 = y as usize;
    let x1 = (x + x_dir) as usize;
    let y1 = (y + y_dir) as usize;
    let x2 = (x + x_dir * 2) as usize;
    let y2 = (y + y_dir * 2) as usize;
    // let mut check_point = points[x0][y0];
    // let mut next_point = points[(x + x_dir) as usize][(y + y_dir) as usize];
    // let mut last_point = points[(x + x_dir * 2) as usize][(y + y_dir * 2) as usize];

    if points[x1][y1].walkable || !points[x2][y2].walkable {
        return; // Not climbable
    }

    let h0 = points[x0][y0].height;
    let h2 = points[x2][y2].height;

    // Difference between levels is 15.9375 in standard map height maps
    // Difference between levels is 2 in standard sc2 measurement units.
    if h0 + 15 <= h2 && h2 <= h0 + 17 {
        if points[x0][y0].cliff_type == Cliff::High {
            points[x0][y0].cliff_type = Cliff::Both;
        } else {
            points[x0][y0].cliff_type = Cliff::Low;
        }
        points[x1][y1].climbable = true;

        if points[x2][y2].cliff_type == Cliff::High {
            points[x2][y2].cliff_type = Cliff::Both;
        } else {
            points[x2][y2].cliff_type = Cliff::High;
        }
    }

    if h2 + 15 <= h0 && h0 <= h2 + 17 {
        if points[x2][y2].cliff_type == Cliff::High {
            points[x2][y2].cliff_type = Cliff::Both;
        } else {
            points[x2][y2].cliff_type = Cliff::Low;
        }
        points[x1][y1].climbable = true;

        if points[x0][y0].cliff_type == Cliff::High {
            points[x0][y0].cliff_type = Cliff::Both;
        } else {
            points[x0][y0].cliff_type = Cliff::High;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn read_vec_from_file(file_path: &str) -> Vec<Vec<usize>> {
        let f = BufReader::new(File::open(file_path).unwrap());
        let mut arr = Vec::<Vec<usize>>::new();

        for line in f.lines().map(|x| x.unwrap()) {
            let mut maze_line = vec![];
            for mini_line in line.chars().map(|n| n.to_digit(2).unwrap()) {
                maze_line.push(mini_line as usize)
            }

            arr.push(maze_line);
        }
        rotate90clockwise(arr)
    }

    fn rotate90clockwise(vec: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let N = vec[0].len();
        let mut new_arr: Vec<Vec<usize>> = vec.clone();
        // Traverse each cycle
        for i in 0..(N / 2) {
            for j in i..(N - i - 1) {
                let temp = vec[i][j];
                new_arr[i][j] = vec[N - 1 - j][i];
                new_arr[N - 1 - j][i] = vec[N - 1 - i][N - 1 - j];
                new_arr[N - 1 - i][N - 1 - j] = vec[j][N - 1 - i];
                new_arr[j][N - 1 - j] = temp;
            }
        }
        new_arr
    }

    #[test]
    fn test_find_path_map() {
        let grid = read_vec_from_file("tests/maze4x4.txt");
        let grid2 = read_vec_from_file("tests/maze4x4.txt");
        let grid3 = read_vec_from_file("tests/maze4x4.txt");
        let map = Map::new_internal(grid, grid2, grid3, 0, 0, 4, 4);
        let path_find = map.ground_pathing;
        let r = path_find.find_path((0, 0), (3, 3), Some(0));
        let (_, distance) = r.unwrap();
        assert_eq!(distance, 6.0);
    }
}
