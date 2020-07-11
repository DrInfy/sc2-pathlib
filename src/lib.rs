#![feature(test)]

use pyo3::prelude::*;
extern crate test;
mod mapping;
mod path_find;

/// This module is a python module implemented in Rust.
#[pymodule]
fn sc2pathlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<path_find::PathFind>()?;
    m.add_class::<mapping::map::Map>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use test::bench::Bencher;

    fn rotate90clockwise(vec: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let N = vec[0].len();
        let mut new_arr: Vec<Vec<usize>> = vec.clone();
        // Traverse each cycle
        for i in 0..(N / 2) {
            for j in i..(N - i - 1) {
                let temp = vec[i][j];
                new_arr[i][j] = vec[N - 1 - j][i];
                new_arr[N - 1 - j][i] = vec[N - 1 - i][N - 1 - j];
                new_arr[N - 1 - i][N - 1 - j] = vec[j][N - 1 - i];
                new_arr[j][N - 1 - j] = temp;
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
        rotate90clockwise(arr)
    }

    fn get_pathfind(file: &str) -> path_find::PathFind {
        let map = read_vec_from_file(file);
        path_find::PathFind::new_internal(map)
    }

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

    #[test]
    fn test_normalize_influence_automaton_le() {
        let mut path_find = get_pathfind("tests/AutomatonLE.txt");
        let sum = path_find.test_normalize_influence(20);
        assert_eq!(sum, 320320);
    }

    #[bench]
    fn bench_find_path_automaton_le(b: &mut Bencher) {
        let path_find = get_pathfind("tests/AutomatonLE.txt");
        // Run bench
        b.iter(|| {
             path_find.find_path((32, 51), (150, 118), Some(0));
         });
    }

    #[bench]
    fn bench_find_path_4x4(b: &mut Bencher) {
        let path_find = get_pathfind("tests/maze4x4.txt");
        // Run bench
        b.iter(|| {
             path_find.find_path((0, 0), (0, 2), Some(0));
         });
    }

    #[bench]
    fn bench_find_path_10x10(b: &mut Bencher) {
        let path_find = get_pathfind("tests/empty10x10.txt");
        // Run bench
        b.iter(|| {
             path_find.find_path((0, 0), (8, 9), Some(0));
         });
    }
}
