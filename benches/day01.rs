use advent_of_code_2023::day01::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::calibrate(divan::black_box(include_str!("../inputs/day01-input.txt",))).unwrap();
}

#[divan::bench]
fn part2() {
    part2::calibrate(divan::black_box(include_str!("../inputs/day01-input.txt",))).unwrap();
}
