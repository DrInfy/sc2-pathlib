use pyo3::prelude::*;

#[derive(Clone)]
pub enum Cliff {
    None = 0,
    Low = 1,
    High = 2,
    Both = 3,
}

#[pyclass]
#[derive(Clone)]
pub struct MapPoint {
    pub ZoneIndex: i8,
    pub CliffType: Cliff,
    pub Pathable: bool,
    pub Walkable: bool,
    pub Climbable: bool,
    pub StructureIndex: i32,
    pub Height: usize,
}

impl MapPoint {
    pub fn new() -> Self {
        let ZoneIndex = 0_i8;
        let CliffType = Cliff::None;
        let Pathable = false;
        let Walkable = false;
        let Climbable = false;
        let StructureIndex = 0_i32;
        let Height = 0;
        MapPoint { ZoneIndex,
                   CliffType,
                   Pathable,
                   Walkable,
                   Climbable,
                   StructureIndex,
                   Height }
    }
}
