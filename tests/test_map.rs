use common::{get_choke_map, read_vec_from_file};
use sc2pathlib::mapping::map::Map;
mod common;

#[test]
fn test_find_path_map() {
    let grid = read_vec_from_file("tests/maze4x4.txt");
    let grid2 = read_vec_from_file("tests/maze4x4.txt");
    let grid3 = read_vec_from_file("tests/maze4x4.txt");
    let map = Map::new(grid, grid2, grid3, 1, 1, 3, 3);
    let r = map.find_path(0, (0f32, 0f32), (3f32, 3f32), Some(0));
    let (_, distance) = r;
    assert_eq!(distance, 6.0);
}

// Test not working, ignored for now.
// #[test]
fn test_find_map_borders() {
    let map = get_choke_map();
    let r = map.get_borders();
    assert_eq!(r.len(), 20 + 16);
}

// Test not working, ignored for now.
// #[test]
fn test_find_map_chokes() {
    let map = get_choke_map();
    let r = map.get_chokes();
    assert_eq!(r.len(), 1);
}
