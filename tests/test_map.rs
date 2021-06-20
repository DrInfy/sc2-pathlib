use common::{get_choke_map, read_vec_from_file};
use sc2pathlib::mapping::{map::Map, vision::VisionUnit};
mod common;

#[test]
fn test_find_path_map() {
    let grid = read_vec_from_file("tests/maze4x4.txt");
    let grid2 = read_vec_from_file("tests/maze4x4.txt");
    let grid3 = read_vec_from_file("tests/maze4x4.txt");
    let map = Map::new(grid, grid2, grid3, 1, 1, 3, 3);
    let r = map.find_path_basic(0, (0f32, 0f32), (3f32, 3f32), Some(0));
    let (_, distance) = r;
    assert_eq!(distance, 6.0);
}

#[test]
fn test_find_map_borders() {
    let map = get_choke_map();
    let r = map.get_borders();
    assert_eq!(r.len(), 76);
}

#[test]
fn test_find_map_chokes() {
    let map = get_choke_map();
    let r = map.get_chokes();
    assert_eq!(r.len(), 1);
}

#[test]
fn test_ray_vision() {
    let mut map = get_choke_map();
    let vision_unit = VisionUnit::new(false, false, (19f32, 8f32), 10f32);
    map.add_vision_unit(vision_unit);
    map.calculate_vision_map();

    assert_eq!(map.vision_status((12f32, 8f32)), 1);
    assert_eq!(map.vision_status((20f32, 8f32)), 1);
    assert_eq!(map.vision_status((25f32, 8f32)), 0);
    assert_eq!(map.vision_status((27f32, 8f32)), 0);
}

#[test]
fn test_flying_vision() {
    let mut map = get_choke_map();
    let vision_unit = VisionUnit::new(false, true, (19f32, 8f32), 10f32);
    map.add_vision_unit(vision_unit);
    map.calculate_vision_map();

    assert_eq!(map.vision_status((21f32, 8f32)), 1);
    assert_eq!(map.vision_status((27f32, 8f32)), 1);
    assert_eq!(map.vision_status((31f32, 8f32)), 0);
}