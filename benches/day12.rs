use advent_of_code_2023::day12::*;
use divan::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

// skip because it's too slow
// which is using brutal force
// #[divan::bench]
// fn part1() {
//     part1::process_data(black_box(include_str!("../inputs/day12-input.txt"))).unwrap();
// }

// #[divan::bench]
// fn part2() {
//     part2::process_data(black_box(include_str!("../inputs/day12-input.txt"))).unwrap();
// }
