use advent_of_code_2023::day04::*;
use divan::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process_data(black_box(include_str!("../inputs/day04-input.txt"))).unwrap();
}

// #[divan::bench]
// fn part2() {
//     part2::process_data(black_box(include_str!("../inputs/day03-input.txt"))).unwrap();
// }
