use pyo3::prelude::*;

use crate::{
    helpers::round_point2,
};

use super::{map::Map};

const DIFFERENCE: usize = 12;
const Y_MULT: usize = 1000000;

#[pymethods]
impl Map {
    pub fn calculate_connections(&mut self, location: (f32, f32)) {
        let pf = self.get_map_mut(0);
        
        let result = pf.djiktra(location, 400f32);
        
        let width = self.ground_pathing.map.len();
        let height = self.ground_pathing.map[0].len();

        // Reset
        for x in 0..width {
            for y in 0..height {
                self.points[x][y].connected = false;
            }
        }

        // Set non ground connected locations
        for data_point in result {
            let point = data_point.0;
            self.points[point.0][point.1].connected = true;
        }
    }

    pub fn is_connected(&mut self, location: (f32, f32)) -> bool {
        let location_int = round_point2(location);
        return self.points[location_int.0][location_int.1].connected;
    }

    pub fn remove_connection(&mut self, location: (f32, f32)) {
        let location_int = round_point2(location);
        self.points[location_int.0][location_int.1].connected = false;
    }
}