use common::get_pathfind;
use sc2pathlib::helpers::point2_f32;
use sc2pathlib::helpers::round_point2;
use sc2pathlib::path_find::octile_distance;
use sc2pathlib::path_find::octile_distance_f32;

mod common;

#[test]
fn test_find_path_automaton_le() {
    let path_find = get_pathfind("tests/AutomatonLE.txt");
    let r = path_find.find_path_basic((32, 51), (150, 118), Some(0));
    let (_, distance) = r;
    assert_eq!(distance, 175.3076);
}

#[test]
fn test_find_path_4x4() {
    let path_find = get_pathfind("tests/maze4x4.txt");
    let r = path_find.find_path_basic((0, 0), (3, 3), Some(0));
    let (_, distance) = r;
    assert_eq!(distance, 6.0);
}

#[test]
fn test_find_path_10x10() {
    let path_find = get_pathfind("tests/empty10x10.txt");
    let r = path_find.find_path_basic((0, 0), (8, 9), Some(0));
    let (_, distance) = r;
    assert_eq!(distance, 12.3136);
}

#[test]
fn test_find_low_inside1() {
    // Assign
    let mut path_find = get_pathfind("tests/empty10x10.txt");
    path_find.normalize_influence(1);
    let enemy_pos = (4usize, 0usize);
    let start_pos = (5f32, 0f32);
    let all_pos: Vec<(usize, usize)> = vec![enemy_pos];
    path_find.add_walk_influence(all_pos, 100f32, 7f32);
    // Act
    let r = path_find.find_low_inside_walk(start_pos, point2_f32(enemy_pos), 8f32);
    // Assert
    let pos = round_point2(r.0);
    let influence = path_find.map[pos.0][pos.1];
    let distance = octile_distance_f32(enemy_pos, pos);

    assert!(distance <= 8f32);
    assert_eq!(influence, 1);
}

#[test]
fn test_find_low_inside2() {
    // Assign
    let mut path_find = get_pathfind("tests/empty10x10.txt");
    path_find.normalize_influence(1);
    let enemy_pos = (4usize, 0usize);
    let start_pos = (5f32, 0f32);
    let all_pos: Vec<(usize, usize)> = vec![enemy_pos];
    path_find.add_walk_influence(all_pos, 100f32, 7f32);
    // Act
    let r = path_find.find_low_inside_walk(start_pos, point2_f32(enemy_pos), 6f32);
    // Assert
    let pos = round_point2(r.0);
    let influence = path_find.map[pos.0][pos.1];
    let distance = octile_distance_f32(enemy_pos, pos);
    assert!(distance <= 6f32);
    assert_eq!(influence, 15);
}

#[test]
fn test_find_low_inside3() {
    // Assign
    let mut path_find = get_pathfind("tests/empty10x10.txt");
    path_find.normalize_influence(1);
    let enemy_pos = (4usize, 0usize);
    let start_pos = (8f32, 4f32);
    let all_pos: Vec<(usize, usize)> = vec![enemy_pos];
    path_find.add_walk_influence(all_pos, 100f32, 7f32);
    // Act
    let r = path_find.find_low_inside_walk(start_pos, point2_f32(enemy_pos), 8f32);
    // Assert
    let pos = round_point2(r.0);
    let influence = path_find.map[pos.0][pos.1];
    let distance = octile_distance_f32(enemy_pos, pos);

    assert!(distance <= 8f32);
    assert_eq!(influence, 1);
}

#[test]
fn test_find_low_inside_far1() {
    // Assign
    let mut path_find = get_pathfind("tests/empty10x10.txt");
    path_find.normalize_influence(1);
    let enemy_pos = (4usize, 0usize);
    let start_pos = (9f32, 9f32);
    let all_pos: Vec<(usize, usize)> = vec![enemy_pos];
    path_find.add_walk_influence(all_pos, 100f32, 7f32);
    // Act
    let r = path_find.find_low_inside_walk(start_pos, point2_f32(enemy_pos), 6f32);
    // Assert
    let pos = round_point2(r.0);
    let influence = path_find.map[pos.0][pos.1];
    let distance = octile_distance_f32(enemy_pos, pos);
    assert!(distance <= 6f32);
    assert_eq!(influence, 17);
}

#[test]
fn test_find_low_inside_far2() {
    // Assign
    let mut path_find = get_pathfind("tests/empty10x10.txt");
    path_find.normalize_influence(1);
    let enemy_pos = (4usize, 0usize);
    let start_pos = (9f32, 9f32);
    let all_pos: Vec<(usize, usize)> = vec![enemy_pos];
    path_find.add_walk_influence(all_pos, 100f32, 7f32);
    // Act
    let r = path_find.find_low_inside_walk(start_pos, point2_f32(enemy_pos), 8f32);
    // Assert
    let pos = round_point2(r.0);
    let influence = path_find.map[pos.0][pos.1];
    let distance = octile_distance_f32(enemy_pos, pos);
    assert!(distance <= 8f32);
    assert_eq!(influence, 1);
}
