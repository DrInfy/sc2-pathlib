use pathfinding::prelude::{absdiff, astar, dijkstra_all, dijkstra_partial};
use pyo3::prelude::*;

mod angles;
pub mod pos;
mod pos_large;
pub mod rectangle;
mod search_grid;

#[pyclass]
pub struct PathFind {
    pub map: Vec<Vec<usize>>,
    original_map: Vec<Vec<usize>>,
    pub width: usize,
    pub height: usize,
    normal_influence: usize,
    auto_correct: bool,
    free_finder: search_grid::FreeFinder,
}

#[inline]
pub fn octile_distance(first: (usize, usize), other: (usize, usize)) -> usize {
    let dx = absdiff(first.0, other.0);
    let dy = absdiff(first.1, other.1);

    if dx > dy {
        pos::MULT * dx + pos::DIAGONAL_MINUS_CARDINAL * dy
    } else {
        pos::MULT * dy + pos::DIAGONAL_MINUS_CARDINAL * dx
    }
}

#[inline]
pub fn octile_distance_f64(first: (usize, usize), other: (usize, usize)) -> f64 {
    (octile_distance(first, other) as f64) / pos::MULTF64
}

#[inline]
pub fn euclidean_distance(first: (f64, f64), other: (f64, f64)) -> f64 {
    let a = first.0 - other.0;
    let b = first.1 - other.1;
    let dist2 = a * a + b * b;
    dist2.sqrt()
}

#[cfg(test)]
impl PathFind {
    pub fn test_normalize_influence(&mut self, value: usize) -> usize {
        self.normalize_influence(value);
        let mut sum: usize = 0;
        for x in &self.map {
            sum += x.iter().sum::<usize>();
        }
        sum
    }
}

impl PathFind {
    pub fn new_internal(map: Vec<Vec<usize>>) -> Self {
        let width = map.len();
        let original_map = map.clone();
        let height = map[0].len();
        let normal_influence: usize = 1;
        let auto_correct: bool = true;
        let free_finder = search_grid::FreeFinder::new();

        PathFind { map,
                   original_map,
                   width,
                   height,
                   normal_influence,
                   auto_correct,
                   free_finder }
    }
    // Removes multiple blocks on the grid and makes it pathable
    // center = center of block
    pub fn remove_blocks_rust(&mut self, centers: &[(f64, f64)], size: (usize, usize)) {
        for center in centers {
            let rect = rectangle::Rectangle::init_from_center(*center, size, self.width, self.height);

            for x in rect.x..rect.x_end {
                for y in rect.y..rect.y_end {
                    self.map[x][y] = self.normal_influence;
                }
            }
        }
    }

    // Creates a block on the grid that is not pathable
    // center = center of building
    pub fn create_blocks_rust(&mut self, centers: &[(f64, f64)], size: (usize, usize)) {
        for center in centers {
            let rect = rectangle::Rectangle::init_from_center(*center, size, self.width, self.height);

            for x in rect.x..rect.x_end {
                for y in rect.y..rect.y_end {
                    self.map[x][y] = 0;
                }
            }
        }
    }
}

#[pymethods]
impl PathFind {
    #[new]
    fn new(map: Vec<Vec<usize>>) -> Self {
        let width = map.len();
        let original_map = map.clone();
        let height = map[0].len();
        let normal_influence: usize = 1;
        let auto_correct: bool = true;
        let free_finder = search_grid::FreeFinder::new();

        PathFind { map,
                   original_map,
                   width,
                   height,
                   normal_influence,
                   auto_correct,
                   free_finder }
    }

    // object.width
    #[getter(width)]
    fn get_width(&self) -> PyResult<usize> { Ok(self.width) }

    // object.height
    #[getter(height)]
    fn get_height(&self) -> PyResult<usize> { Ok(self.height) }

    // object.normal_influence
    #[getter(normal_influence)]
    fn get_normal_influence(&self) -> PyResult<usize> { Ok(self.normal_influence) }

    // object.map
    #[getter(map)]
    fn get_map(&self) -> PyResult<Vec<Vec<usize>>> { Ok(self.map.clone()) }

    // object.map(2dArray)
    #[setter(map)]
    fn set_map(&mut self, value: Vec<Vec<usize>>) -> PyResult<()> {
        self.map = value;
        Ok(())
    }

    // object.auto_correct
    #[getter(auto_correct)]
    fn get_auto_correct(&self) -> PyResult<bool> { Ok(self.auto_correct) }

    // object.map(bool)
    #[setter(auto_correct)]
    fn set_auto_correct(&mut self, value: bool) -> PyResult<()> {
        self.auto_correct = value;
        Ok(())
    }

    pub fn reset(&mut self) -> PyResult<()> {
        self.map = self.original_map.clone();
        Ok(())
    }

    pub fn reset_void(&mut self) { self.map = self.original_map.clone(); }

    // Creates a block on the grid that is not pathable
    // center = center of building
    pub fn create_block(&mut self, center: (f64, f64), size: (usize, usize)) {
        let rect = rectangle::Rectangle::init_from_center(center, size, self.width, self.height);

        for x in rect.x..rect.x_end {
            for y in rect.y..rect.y_end {
                self.map[x][y] = 0;
            }
        }
    }

    // Creates a block on the grid that is not pathable
    // center = center of building
    pub fn create_blocks(&mut self, centers: Vec<(f64, f64)>, size: (usize, usize)) {
        for center in centers {
            let rect = rectangle::Rectangle::init_from_center(center, size, self.width, self.height);

            for x in rect.x..rect.x_end {
                for y in rect.y..rect.y_end {
                    self.map[x][y] = 0;
                }
            }
        }
    }

    // Removes a block on the grid and makes it pathable
    // center = center of block
    pub fn remove_block(&mut self, center: (f64, f64), size: (usize, usize)) {
        let rect = rectangle::Rectangle::init_from_center(center, size, self.width, self.height);

        for x in rect.x..rect.x_end {
            for y in rect.y..rect.y_end {
                self.map[x][y] = self.normal_influence;
            }
        }
    }

    pub fn normalize_influence(&mut self, value: usize) {
        self.normal_influence = value;

        for y in &mut self.map {
            for x in y {
                if *x > 0 {
                    *x = value;
                }
            }
        }
    }

    /// Adds influence based on euclidean distance
    fn add_influence(&mut self, positions: Vec<(usize, usize)>, max: f64, distance: f64) -> PyResult<()> {
        let mult = 1.0 / (distance * pos::MULTF64);
        let diameter = ((distance * 2f64) as usize) + 2;
        let rect_size = (diameter, diameter);

        for position in positions {
            let rect = rectangle::Rectangle::init_from_center2(position, rect_size, self.width, self.height);

            for x in rect.x..rect.x_end {
                for y in rect.y..rect.y_end {
                    let value = max * (1.0 - (octile_distance(position, (x, y)) as f64) * mult);
                    if value > 0.0 && self.map[x][y] > 0 {
                        self.map[x][y] += value as usize;
                    }
                }
            }
        }

        Ok(())
    }

    /// Adds influence based on euclidean distance
    fn add_influence_flat(&mut self, positions: Vec<(usize, usize)>, max: f64, distance: f64) -> PyResult<()> {
        let value = max as usize;
        let mult_distance = distance * pos::MULTF64;

        let diameter = ((distance * 2f64) as usize) + 2;
        let rect_size = (diameter, diameter);

        for position in positions {
            let rect = rectangle::Rectangle::init_from_center2(position, rect_size, self.width, self.height);

            for x in rect.x..rect.x_end {
                for y in rect.y..rect.y_end {
                    if (octile_distance(position, (x, y)) as f64) < mult_distance {
                        self.map[x][y] += value;
                    }
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
    fn add_walk_influence_flat(&mut self, positions: Vec<(usize, usize)>, max: f64, distance: f64) {
        let max_int = max as usize;

        for position in &positions {
            let corrected_position = self.get_closest_pathable(*position);

            if self.map[corrected_position.0][corrected_position.1] == 0 {
                continue;
            }

            let destinations = self.find_destinations_in_inline(corrected_position, distance);
            self.map[position.0][position.1] += max_int;

            for destination in destinations {
                let end_point = destination.0;
                self.map[end_point.0][end_point.1] += max_int
            }
        }
    }

    /// Finds the first reachable position within specified walking distance from the center point with lowest value
    pub fn lowest_influence_walk(&self, center: (usize, usize), distance: f64) -> ((usize, usize), f64) {
        let corrected_center = self.get_closest_pathable(center);

        return self.lowest_influence_walk_inline(corrected_center, distance);
    }

    #[inline]
    fn lowest_influence_walk_inline(&self, center: (usize, usize), distance: f64) -> ((usize, usize), f64) {
        let destinations = self.find_destinations_in_inline(center, distance);

        let mut min_value = std::usize::MAX;
        let mut min_distance = std::f64::MAX;
        let mut min_position = center;

        for destination in destinations {
            let pos = destination.0;
            let new_val = self.map[pos.0][pos.1];
            if new_val == 0 {
                continue;
            }

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
    fn lowest_influence(&self, center: (f64, f64), distance: usize) -> ((usize, usize), f64) {
        self.inline_lowest_value(center, distance)
    }

    #[inline]
    pub fn inline_lowest_value(&self, center: (f64, f64), distance: usize) -> ((usize, usize), f64) {
        let rect = rectangle::Rectangle::init_from_center(center, (distance, distance), self.width, self.height);

        let mut min_value = std::usize::MAX;
        let mut min_distance = std::usize::MAX;
        let mut min_position = (center.0 as usize, center.1 as usize);
        let target_pos = (center.0 as usize, center.1 as usize);

        for x in rect.x..rect.x_end {
            for y in rect.y..rect.y_end {
                let new_val = self.map[x][y];
                if new_val == 0 {
                    continue;
                }

                let distance = octile_distance((x, y), target_pos);

                if new_val < min_value || (new_val == min_value && distance < min_distance) {
                    min_value = new_val;
                    min_distance = distance;
                    min_position = (x, y);
                }
            }
        }

        (min_position, min_distance as f64 / pos::MULTF64)
    }

    /// Find the shortest path values without considering influence and returns the path and distance
    pub fn find_path(&self,
                     start: (usize, usize),
                     end: (usize, usize),
                     possible_heuristic: Option<u8>)
                     -> (Vec<(usize, usize)>, f64) {
        let corrected_start = self.get_closest_pathable(start);
        let corrected_end = self.get_closest_pathable(end);

        let start: pos::Pos = pos::Pos(corrected_start.0, corrected_start.1);
        let goal: pos::Pos = pos::Pos(corrected_end.0, corrected_end.1);
        let grid: &Vec<Vec<usize>> = &self.map;

        let result: Option<(Vec<pos::Pos>, usize)>;
        match possible_heuristic.unwrap_or(0) {
            0 => result = astar(&start, |p| p.successors(grid), |p| p.manhattan_distance(&goal), |p| *p == goal),
            1 => result = astar(&start, |p| p.successors(grid), |p| p.octile_distance(&goal), |p| *p == goal),
            _ => result = astar(&start, |p| p.successors(grid), |p| p.euclidean_distance(&goal), |p| *p == goal),
        };

        let mut path: Vec<(usize, usize)>;
        let distance: f64;

        match result {
            None => {
                path = Vec::<(usize, usize)>::new();
                distance = 0.0
            }
            Some(t) => {
                distance = (t.1 as f64) / pos::MULTF64;
                path = Vec::<(usize, usize)>::with_capacity(t.0.len());
                for pos in t.0 {
                    path.push((pos.0, pos.1))
                }
            }
        }

        (path, distance)
    }

    /// Find the shortest path values without considering influence and returns the path and distance
    pub fn find_path_large(&self,
                           start: (usize, usize),
                           end: (usize, usize),
                           possible_heuristic: Option<u8>)
                           -> (Vec<(usize, usize)>, f64) {
        let corrected_start = self.get_closest_pathable(start);
        let corrected_end = self.get_closest_pathable(end);

        let start: pos_large::PosLarge = pos_large::PosLarge(corrected_start.0, corrected_start.1);
        let goal: pos_large::PosLarge = pos_large::PosLarge(corrected_end.0, corrected_end.1);
        let grid: &Vec<Vec<usize>> = &self.map;

        let result: Option<(Vec<pos_large::PosLarge>, usize)>;
        match possible_heuristic.unwrap_or(0) {
            0 => result = astar(&start, |p| p.successors(grid), |p| p.manhattan_distance(&goal), |p| *p == goal),
            1 => result = astar(&start, |p| p.successors(grid), |p| p.octile_distance(&goal), |p| *p == goal),
            _ => result = astar(&start, |p| p.successors(grid), |p| p.euclidean_distance(&goal), |p| *p == goal),
        };

        let mut path: Vec<(usize, usize)>;
        let distance: f64;

        match result {
            None => {
                path = Vec::<(usize, usize)>::new();
                distance = 0.0
            }
            Some(t) => {
                distance = (t.1 as f64) / pos::MULTF64;
                path = Vec::<(usize, usize)>::with_capacity(t.0.len());
                for pos in t.0 {
                    path.push((pos.0, pos.1))
                }
            }
        }

        (path, distance)
    }

    /// Find the path using influence values and returns the path and distance
    pub fn find_path_influence(&self,
                               start: (usize, usize),
                               end: (usize, usize),
                               possible_heuristic: Option<u8>)
                               -> (Vec<(usize, usize)>, f64) {
        let corrected_start = self.get_closest_pathable(start);
        let corrected_end = self.get_closest_pathable(end);

        self.find_path_influence_inline(corrected_start, corrected_end, possible_heuristic)
    }

    #[inline]
    fn find_path_influence_inline(&self,
                                  corrected_start: (usize, usize),
                                  corrected_end: (usize, usize),
                                  possible_heuristic: Option<u8>)
                                  -> (Vec<(usize, usize)>, f64) {
        let start = pos::InfluencedPos(corrected_start.0, corrected_start.1);
        let goal = pos::InfluencedPos(corrected_end.0, corrected_end.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let infl = self.normal_influence;

        let result: Option<(Vec<pos::InfluencedPos>, usize)>;

        match possible_heuristic.unwrap_or(0) {
            0 => result = astar(&start, |p| p.successors(grid), |p| p.manhattan_distance(&goal, infl), |p| *p == goal),
            1 => result = astar(&start, |p| p.successors(grid), |p| p.octile_distance(&goal, infl), |p| *p == goal),
            _ => result = astar(&start, |p| p.successors(grid), |p| p.euclidean_distance(&goal, infl), |p| *p == goal),
        };

        let mut path: Vec<(usize, usize)>;
        let distance: f64;

        match result {
            None => {
                path = Vec::<(usize, usize)>::new();
                distance = 0.0
            }
            Some(t) => {
                distance = (t.1 as f64) / pos::MULTF64;
                path = Vec::<(usize, usize)>::with_capacity(t.0.len());
                for pos in t.0 {
                    path.push((pos.0, pos.1))
                }
            }
        }

        (path, distance)
    }

    /// Find the path using influence values and returns the path and distance
    pub fn find_path_influence_large(&self,
                                     start: (usize, usize),
                                     end: (usize, usize),
                                     possible_heuristic: Option<u8>)
                                     -> (Vec<(usize, usize)>, f64) {
        let corrected_start = self.get_closest_pathable(start);
        let corrected_end = self.get_closest_pathable(end);

        let start = pos_large::InfluencedPosLarge(corrected_start.0, corrected_start.1);
        let goal = pos_large::InfluencedPosLarge(corrected_end.0, corrected_end.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let infl = self.normal_influence;

        let result: Option<(Vec<pos_large::InfluencedPosLarge>, usize)>;

        match possible_heuristic.unwrap_or(0) {
            0 => result = astar(&start, |p| p.successors(grid), |p| p.manhattan_distance(&goal, infl), |p| *p == goal),
            1 => result = astar(&start, |p| p.successors(grid), |p| p.octile_distance(&goal, infl), |p| *p == goal),
            _ => result = astar(&start, |p| p.successors(grid), |p| p.euclidean_distance(&goal, infl), |p| *p == goal),
        };

        let mut path: Vec<(usize, usize)>;
        let distance: f64;

        match result {
            None => {
                path = Vec::<(usize, usize)>::new();
                distance = 0.0
            }
            Some(t) => {
                distance = (t.1 as f64) / pos::MULTF64;
                path = Vec::<(usize, usize)>::with_capacity(t.0.len());
                for pos in t.0 {
                    path.push((pos.0, pos.1))
                }
            }
        }

        (path, distance)
    }

    /// Finds all reachable destinations from selected start point. Ignores influence.
    fn find_all_destinations(&self, start: (usize, usize)) -> PyResult<Vec<((usize, usize), f64)>> {
        let start: pos::Pos = pos::Pos(start.0, start.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let result = dijkstra_all(&start, |p| p.successors(&grid));

        let mut destination_collection: Vec<((usize, usize), f64)> =
            Vec::<((usize, usize), f64)>::with_capacity(result.len());

        for found_path in result {
            let x = ((found_path.1).0).0;
            let y = ((found_path.1).0).1;
            let d = ((found_path.1).1 as f64) / pos::MULTF64;
            destination_collection.push(((x, y), d));
        }

        Ok(destination_collection)
    }

    /// Finds all reachable destinations from selected start point. Ignores influence.
    fn find_destinations_in(&self, start: (usize, usize), distance: f64) -> PyResult<Vec<((usize, usize), f64)>> {
        Ok(self.find_destinations_in_inline(start, distance))
    }

    #[inline]
    pub fn find_destinations_in_inline(&self, start: (usize, usize), distance: f64) -> Vec<((usize, usize), f64)> {
        let start: pos::Pos = pos::Pos(start.0, start.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let u_distance = (distance * pos::MULTF64) as usize;

        let result = dijkstra_partial(&start, |p| p.successors(&grid), |p| p.octile_distance(&start) > u_distance);

        let hash_map = result.0;
        let mut destination_collection: Vec<((usize, usize), f64)> =
            Vec::<((usize, usize), f64)>::with_capacity(hash_map.len());

        for found_path in hash_map {
            let x = (found_path.0).0;
            let y = (found_path.0).1;
            let d = ((found_path.1).1 as f64) / pos::MULTF64;
            destination_collection.push(((x, y), d));
        }

        destination_collection
    }

    #[inline]
    fn find_destinations_in_inline_influence(&self,
                                             start: (usize, usize),
                                             distance: f64)
                                             -> Vec<((usize, usize), f64)> {
        let start: pos::InfluencedPos = pos::InfluencedPos(start.0, start.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let u_distance = (distance * (self.normal_influence as f64) * pos::MULTF64) as usize;

        let result = dijkstra_partial(&start,
                                      |p| p.successors(&grid),
                                      |p| p.octile_distance(&start, self.normal_influence) > u_distance);

        let hash_map = result.0;
        let mut destination_collection: Vec<((usize, usize), f64)> =
            Vec::<((usize, usize), f64)>::with_capacity(hash_map.len());

        for found_path in hash_map {
            let x = (found_path.0).0;
            let y = (found_path.0).1;
            let d = ((found_path.1).1 as f64) / pos::MULTF64;
            destination_collection.push(((x, y), d));
        }

        destination_collection
    }

    #[inline]
    fn get_closest_pathable(&self, start: (usize, usize)) -> (usize, usize) {
        if self.auto_correct || self.map[start.0][start.1] > 0 {
            start
        } else {
            self.free_finder.find_free(start, &self.map, self.width, self.height)
        }
    }

    /// Finds a compromise where low influence matches with close position to the start position.
    pub fn find_low_inside_walk(&self, start: (f64, f64), target: (f64, f64), distance: f64) -> ((f64, f64), f64) {
        let start_int = (start.0 as usize, start.1 as usize);
        let target_int = (target.0 as usize, target.1 as usize);

        let current_distance = euclidean_distance(start, target);

        let corrected_start = self.get_closest_pathable(start_int);
        let corrected_target = self.get_closest_pathable(target_int);
        let angle = angles::angle_between_f64(start, target);
        let u_distance = distance as usize;
        let rect = rectangle::Rectangle::init_from_center2(corrected_target,
                                                           (u_distance, u_distance),
                                                           self.width,
                                                           self.height);

        let mut destinations = Vec::<((usize, usize), usize)>::new();

        for x in rect.x..rect.x_end {
            for y in rect.y..rect.y_end {
                let new_val = self.map[x][y];
                if new_val > 0 {
                    destinations.push(((x, y), new_val));
                }
            }
        }

        if destinations.is_empty() {
            // Cannot find path to target
            return ((0.0, 0.0), -1.0);
        } else {
            let mut best_target: ((f64, f64), f64) = ((0.0, 0.0), -1.0);

            // Get a backup position that's closest to start up position
            for destination in destinations {
                let point = destination.0;
                let distance_from_start = octile_distance_f64(start_int, point);

                if distance_from_start < best_target.1 || best_target.1 < 0.0 {
                    let point_f64 = (point.0 as f64 + 0.5, point.1 as f64 + 0.5);
                    best_target = (point_f64, distance_from_start);
                }
            }

            //let best_path = self.find_path_influence_inline(corrected_start, best_target.0, Some(1u8));

            if current_distance < distance + 4.0 {
                let best_influence = self.map[(best_target.0).0 as usize][(best_target.0).1 as usize];
                //let mut best_distance_from_target = octile_distance_f64(best_target.0, target_int);
                let destinations_from_start = self.find_destinations_in_inline(corrected_start, 5.0);
                let mut angle_distance =
                    angles::angle_distance(angle, angles::angle_between_f64(best_target.0, target));
                let mut best_score = best_influence as f64 * (1.0 + angle_distance * 0.25);

                for destination in destinations_from_start {
                    let point = destination.0;
                    let point_f64 = (point.0 as f64 + 0.5, point.1 as f64 + 0.5);
                    let influence = self.map[point.0][point.1];
                    //let distance_from_target = euclidean_distance(point_f64, target);
                    angle_distance = angles::angle_distance(angle, angles::angle_between_f64(point_f64, target));
                    let score = influence as f64 * (1.0 + angle_distance * 0.25);

                    if score < best_score {
                        best_score = score;
                        best_target = (point_f64, destination.1);
                    }
                }
            }

            return best_target;
        }
    }

    pub fn invert_djiktra(&self, start: (f64, f64), distance: f64) -> Vec<((usize, usize), f64)> {
        let start_int = (start.0 as usize, start.1 as usize);
        let start: pos::InvertPos = pos::InvertPos(start_int.0, start_int.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let u_distance = (distance * pos::MULTF64) as usize;

        let result = dijkstra_partial(&start, |p| p.successors(&grid), |p| p.octile_distance(&start) > u_distance);

        let hash_map = result.0;
        let mut destination_collection: Vec<((usize, usize), f64)> =
            Vec::<((usize, usize), f64)>::with_capacity(hash_map.len());

        for found_path in hash_map {
            let x = (found_path.0).0;
            let y = (found_path.0).1;
            let d = ((found_path.1).1 as f64) / pos::MULTF64;
            destination_collection.push(((x, y), d));
        }

        return destination_collection;
    }

    pub fn djiktra(&self, start: (f64, f64), distance: f64) -> Vec<((usize, usize), f64)> {
        let start_int = (start.0 as usize, start.1 as usize);
        let start: pos::Pos = pos::Pos(start_int.0, start_int.1);
        let grid: &Vec<Vec<usize>> = &self.map;
        let u_distance = (distance * pos::MULTF64) as usize;

        let result = dijkstra_partial(&start, |p| p.successors(&grid), |p| p.octile_distance(&start) > u_distance);

        let hash_map = result.0;
        let mut destination_collection: Vec<((usize, usize), f64)> =
            Vec::<((usize, usize), f64)>::with_capacity(hash_map.len());

        for found_path in hash_map {
            let x = (found_path.0).0;
            let y = (found_path.0).1;
            let d = ((found_path.1).1 as f64) / pos::MULTF64;
            destination_collection.push(((x, y), d));
        }

        return destination_collection;
    }
}
