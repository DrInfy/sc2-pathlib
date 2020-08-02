use criterion::{black_box, criterion_group, criterion_main, Criterion};
use common::get_pathfind;
mod common;

fn bench_astar_automaton(c: &mut Criterion) {
    let path_find = get_pathfind("tests/AutomatonLE.txt");
    c.bench_function("find_path_automaton", |b| {
        b.iter(|| {
            path_find.find_path((32, 51), (150, 118), Some(0));
        })
    });
}

fn bench_astar_4x4(c: &mut Criterion) {
    let path_find = get_pathfind("tests/maze4x4.txt");
    // Run bench
    c.bench_function("find_path_4x4", |b| {
        b.iter(|| {
            path_find.find_path((0, 0), (0, 2), Some(0));
        })
    });
}


fn bench_astar_10x10(c: &mut Criterion) {
    let path_find = get_pathfind("tests/empty10x10.txt");
    // Run bench
    c.bench_function("find_path_10x10", |b| {
    b.iter(|| {
         path_find.find_path((0, 0), (8, 9), Some(0));
     })});
}

criterion_group!(benches, bench_astar_automaton, bench_astar_4x4, bench_astar_10x10);
criterion_main!(benches);
