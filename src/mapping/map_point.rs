use pyo3::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Cliff {
    None = 0b0000,
    Low = 0b0001,
    High = 0b0010,
    Both = 0b0011,
}

#[pyclass]
#[derive(Clone)]
pub struct MapPoint {
    pub zone_index: i8,
    pub cliff_type: Cliff,
    pub pathable: bool,
    pub walkable: bool,
    pub climbable: bool,
    pub structure_index: i32,
    pub height: usize,
}

impl MapPoint {
    pub fn new() -> Self {
        let zone_index = 0_i8;
        let cliff_type = Cliff::None;
        let pathable = false;
        let walkable = false;
        let climbable = false;
        let structure_index = 0_i32;
        let height = 0;
        MapPoint { zone_index,
                   cliff_type,
                   pathable,
                   walkable,
                   climbable,
                   structure_index,
                   height }
    }
}
