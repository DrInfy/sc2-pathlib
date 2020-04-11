use pyo3::prelude::*;

enum Cliff {
    None = 0,
    Low = 1,
    High = 2,
    Both = 3,
}

#[pyclass]
pub struct MapPoint {
    ZoneIndex: i8,
    CliffType: Cliff,
    Pathable: bool,
    Walkable: bool,
    Climbable: bool,
    StructureIndex: i32,
}