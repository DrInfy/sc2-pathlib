use std::io::{BufReader, BufRead};
use std::fs::File;
use sc2pathlib::path_find;


fn rot90(vec: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let len = vec[0].len();
    let mut new_arr: Vec<Vec<usize>> = vec.clone();
    // Traverse each cycle
    for i in 0..(len / 2) {
        for j in i..(len - i - 1) {
            let temp = vec[i][j];
            new_arr[i][j] = vec[len - 1 - j][i];
            new_arr[len - 1 - j][i] = vec[len - 1 - i][len - 1 - j];
            new_arr[len - 1 - i][len - 1 - j] = vec[j][len - 1 - i];
            new_arr[j][len - 1 - j] = temp;
        }
    }
    new_arr
}


fn read_vec_from_file(file_path: &str) -> Vec<Vec<usize>> {
    let f = BufReader::new(File::open(file_path).unwrap());
    let mut arr = Vec::<Vec<usize>>::new();

    for line in f.lines().map(|x| x.unwrap()) {
        let mut maze_line = vec![];
        for mini_line in line.chars().map(|n| n.to_digit(2).unwrap()) {
            maze_line.push(mini_line as usize)
        }

        arr.push(maze_line);
    }
    rot90(arr)
}

pub fn get_pathfind(file: &str) -> path_find::PathFind {
    let map = read_vec_from_file(file);
    path_find::PathFind::new_internal(map)
}