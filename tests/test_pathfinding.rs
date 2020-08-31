use common::get_pathfind;

mod common;

#[test]
fn test_find_path_automaton_le() {
    let path_find = get_pathfind("tests/AutomatonLE.txt");
    let r = path_find.find_path((32, 51), (150, 118), Some(0));
    let (_, distance) = r;
    assert_eq!(distance, 147.1656);
}


#[test]
fn test_find_path_4x4() {
    let path_find = get_pathfind("tests/maze4x4.txt");
    let r = path_find.find_path((0, 0), (3, 3), Some(0));
    let (_, distance) = r;
    assert_eq!(distance, 6.0);
}

#[test]
fn test_find_path_10x10() {
    let path_find = get_pathfind("tests/empty10x10.txt");
    let r = path_find.find_path((0, 0), (8, 9), Some(0));
    let (_, distance) = r;
    assert_eq!(distance, 12.3136);
}