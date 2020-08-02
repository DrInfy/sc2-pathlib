#![allow(dead_code)]

use pyo3::prelude::*;
pub mod mapping;
pub mod path_find;

/// This module is a python module implemented in Rust.
#[pymodule]
fn sc2pathlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<path_find::PathFind>()?;
    m.add_class::<mapping::map::Map>()?;
    Ok(())
}
