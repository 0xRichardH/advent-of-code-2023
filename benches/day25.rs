use advent_of_code_2023::day25::*;
use divan::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    process_data(black_box(include_str!("../inputs/day25-input.txt"))).unwrap();
}
