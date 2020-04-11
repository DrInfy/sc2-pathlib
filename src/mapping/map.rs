use crate::path_find::PathFind;
use pyo3::prelude::*;

use crate::mapping::map_point;

#[pyclass]
pub struct Map {
    ground_pathing: PathFind,
    air_pathing: PathFind,
    colossus_pathing: PathFind,
    reaper_pathing: PathFind,
    points: Vec<Vec<map_point::MapPoint>>,
}

fn modify_climb(points: &mut Vec<Vec<map_point::MapPoint>>, x: i32, y: i32, x_dir: i32, y_dir: i32) {
    let mut check_point = &points[x as usize][y as usize];
    let mut next_point = &points[(x + x_dir) as usize][(y + y_dir) as usize];
    let mut last_point = &points[(x + x_dir * 2) as usize][(y + y_dir * 2) as usize];

    if next_point.Walkable || !last_point.Walkable {
        return; // Not climbable
    }

    // TODO: Solve high / low cliff
}

#[pymethods]
impl Map {
    #[new]
    fn new(obj: &PyRawObject,
           pathing: Vec<Vec<usize>>,
           placement: Vec<Vec<usize>>,
           height: Vec<Vec<usize>>,
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

        for x in 0..width - 1 {
            for y in 0..height - 1 {
                points[x][y].Walkable = pathing[x][y] > 0 || placement[x][y] > 0;
                points[x][y].Pathable = x_start <= x && x <= x_end && y_start <= y && y <= y_end;
                if points[x][y].Pathable {
                    fly_map[x][y] = 1;
                }
            }
        }

        for x in 2..width - 3 {
            for y in 2..height - 3 {
                if !points[x][y].Walkable {
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
