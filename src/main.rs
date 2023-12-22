#[allow(unused_imports)]
use advent_of_code_2023::*;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "day01-part1")]
    run_day01_part1()?;

    #[cfg(feature = "day01-part2")]
    run_day01_part2()?;

    #[cfg(feature = "day02-part1")]
    run_day02_part1()?;

    #[cfg(feature = "day02-part2")]
    run_day02_part2()?;

    #[cfg(feature = "day03-part1")]
    run_day03_part1()?;

    #[cfg(feature = "day03-part2")]
    run_day03_part2()?;

    #[cfg(feature = "day04-part1")]
    run_day04_part1()?;

    #[cfg(feature = "day04-part2")]
    run_day04_part2()?;

    #[cfg(feature = "day05-part1")]
    run_day05_part1()?;

    #[cfg(feature = "day05-part2")]
    run_day05_part2()?;

    #[cfg(feature = "day06-part1")]
    run_day06_part1()?;

    #[cfg(feature = "day06-part2")]
    run_day06_part2()?;

    #[cfg(feature = "day07-part1")]
    run_day07_part1()?;

    #[cfg(feature = "day07-part2")]
    run_day07_part2()?;

    #[cfg(feature = "day08-part1")]
    run_day08_part1()?;

    #[cfg(feature = "day08-part2")]
    run_day08_part2()?;

    #[cfg(feature = "day09-part1")]
    run_day09_part1()?;

    #[cfg(feature = "day09-part2")]
    run_day09_part2()?;

    #[cfg(feature = "day10-part1")]
    run_day10_part1()?;

    #[cfg(feature = "day10-part1-bfs")]
    run_day10_part1_bfs()?;

    #[cfg(feature = "day10-part2")]
    run_day10_part2()?;

    #[cfg(feature = "day11-part1")]
    run_day11_part1();

    #[cfg(feature = "day11-part2")]
    run_day11_part2();

    #[cfg(feature = "day12-part1")]
    run_day12_part1()?;

    #[cfg(feature = "day12-part2")]
    run_day12_part2()?;

    #[cfg(feature = "day13-part1")]
    run_day13_part1();

    #[cfg(feature = "day13-part2")]
    run_day13_part2();

    #[cfg(feature = "day14-part1")]
    run_day14_part1();

    #[cfg(feature = "day14-part2")]
    run_day14_part2();

    #[cfg(feature = "day15-part1")]
    run_day15_part1();

    #[cfg(feature = "day15-part2")]
    run_day15_part2();

    #[cfg(feature = "day16-part1")]
    run_day16_part1();

    #[cfg(feature = "day16-part2")]
    run_day16_part2();

    #[cfg(feature = "day17-part1")]
    run_day17_part1();

    #[cfg(feature = "day17-part2")]
    run_day17_part2();

    #[cfg(feature = "day18-part1")]
    run_day18_part1();

    #[cfg(feature = "day18-part2")]
    run_day18_part2();

    #[cfg(feature = "day19-part1")]
    run_day19_part1();

    #[cfg(feature = "day19-part2")]
    run_day19_part2();

    #[cfg(feature = "day20-part1")]
    run_day20_part1();

    #[cfg(feature = "day20-part2")]
    run_day20_part2();

    #[cfg(feature = "day22-part1")]
    run_day22_part1();

    Ok(())
}

#[cfg(feature = "day01-part1")]
fn run_day01_part1() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day01-input.txt");
    let result = day01::part1::calibrate(input_strings)?;
    println!("daily01-part1 = {}", result);

    Ok(())
}

#[cfg(feature = "day01-part2")]
fn run_day01_part2() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day01-input.txt");
    let result = day01::part2::calibrate(input_strings)?;
    println!("daily01-part2 = {}", result);

    Ok(())
}

#[cfg(feature = "day02-part1")]
fn run_day02_part1() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day02-input.txt");
    let result = day02::part1::process_data(input_strings)?;
    println!("daily02-part1 = {}", result);

    Ok(())
}

#[cfg(feature = "day02-part2")]
fn run_day02_part2() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day02-input.txt");
    let result = day02::part2::process_data(input_strings)?;
    println!("daily02-part2 = {}", result);

    Ok(())
}

#[cfg(feature = "day03-part1")]
fn run_day03_part1() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day03-input.txt");
    let result = day03::part1::process_data(input_strings)?;
    println!("daily03-part1 = {}", result); // 532445
    Ok(())
}

#[cfg(feature = "day03-part2")]
fn run_day03_part2() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day03-input.txt");
    let result = day03::part2::process_data(input_strings)?;
    println!("daily03-part2 = {}", result); // 79842967
    Ok(())
}

#[cfg(feature = "day04-part1")]
fn run_day04_part1() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day04-input.txt");
    let result = day04::part1::process_data(input_strings)?;
    println!("daily04-part1 = {}", result); // 15205
    Ok(())
}

#[cfg(feature = "day04-part2")]
fn run_day04_part2() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day04-input.txt");
    let result = day04::part2::process_data(input_strings)?;
    println!("daily04-part2 = {}", result); // 6189740
    Ok(())
}

#[cfg(feature = "day05-part1")]
fn run_day05_part1() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day05-input.txt");
    let result = day05::part1::process_data(input_strings)?;
    println!("daily05-part1 = {}", result); // 486613012
    Ok(())
}

#[cfg(feature = "day05-part2")]
fn run_day05_part2() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day05-input.txt");
    let result = day05::part2::process_data(input_strings)?;
    println!("daily05-part2 = {}", result); // 56931769
    Ok(())
}

#[cfg(feature = "day06-part1")]
fn run_day06_part1() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day06-input.txt");
    let result = day06::part1::process_data(input_strings)?;
    println!("daily06-part1 = {}", result); // 3317888
    Ok(())
}

#[cfg(feature = "day06-part2")]
fn run_day06_part2() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day06-input.txt");
    let result = day06::part2::process_data(input_strings)?;
    println!("daily06-part2 = {}", result); // 24655068
    Ok(())
}

#[cfg(feature = "day07-part1")]
fn run_day07_part1() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day07-input.txt");
    let result = day07::part1::process_data(input_strings)?;
    println!("daily07-part1 = {}", result); // 250058342
    Ok(())
}

#[cfg(feature = "day07-part2")]
fn run_day07_part2() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day07-input.txt");
    let result = day07::part2::process_data(input_strings)?;
    println!("daily07-part2 = {}", result); // 250506580
    Ok(())
}

#[cfg(feature = "day08-part1")]
fn run_day08_part1() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day08-input.txt");
    let result = day08::part1::process_data(input_strings)?;
    println!("daily08-part1 = {}", result); // 16271
    Ok(())
}

#[cfg(feature = "day08-part2")]
fn run_day08_part2() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day08-input.txt");
    let result = day08::part2::process_data(input_strings)?;
    println!("daily08-part2 = {}", result); // 14265111103729
    Ok(())
}

#[cfg(feature = "day09-part1")]
fn run_day09_part1() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day09-input.txt");
    let result = day09::part1::process_data(input_strings)?;
    println!("daily09-part1 = {}", result); // 1993300041
    Ok(())
}

#[cfg(feature = "day09-part2")]
fn run_day09_part2() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day09-input.txt");
    let result = day09::part2::process_data(input_strings)?;
    println!("daily09-part2 = {}", result); // 1038
    Ok(())
}

#[cfg(feature = "day10-part1")]
fn run_day10_part1() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day10-input.txt");
    let result = day10::part1::process_data(input_strings)?;
    println!("daily10-part1 = {}", result); // 7066
    Ok(())
}

#[cfg(feature = "day10-part1-bfs")]
fn run_day10_part1_bfs() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day10-input.txt");
    let result = day10::part1_bfs::process_data(input_strings)?;
    println!("daily10-part1 = {}", result); // 7066
    Ok(())
}

#[cfg(feature = "day10-part2")]
fn run_day10_part2() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day10-input.txt");
    let result = day10::part2::process_data(input_strings)?;
    println!("daily10-part2 = {}", result); // 401
    Ok(())
}

#[cfg(feature = "day11-part1")]
fn run_day11_part1() {
    let input_strings = include_str!("../inputs/day11-input.txt");
    let result = day11::part1::process_data(input_strings);
    println!("daily11-part1 = {}", result); // 9418609
}

#[cfg(feature = "day11-part2")]
fn run_day11_part2() {
    let input_strings = include_str!("../inputs/day11-input.txt");
    let result = day11::part2::process_data(input_strings, 1000000);
    println!("daily11-part2 = {}", result); // 593821230983
}

#[cfg(feature = "day12-part1")]
fn run_day12_part1() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day12-input.txt");
    let result = day12::part1::process_data(input_strings)?;
    println!("daily12-part1 = {}", result); // 7857
    Ok(())
}

#[cfg(feature = "day12-part2")]
fn run_day12_part2() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day12-input.txt");
    let result = day12::part2::process_data(input_strings)?;
    println!("daily12-part2 = {}", result); // 28606137449920
    Ok(())
}

#[cfg(feature = "day13-part1")]
fn run_day13_part1() {
    let input_strings = include_str!("../inputs/day13-input.txt");
    let result = day13::part1::process_data(input_strings);
    println!("daily13-part1 = {}", result); // 37113
}

#[cfg(feature = "day13-part2")]
fn run_day13_part2() {
    let input_strings = include_str!("../inputs/day13-input.txt");
    let result = day13::part2::process_data(input_strings);
    println!("daily13-part2 = {}", result); // 30449
}

#[cfg(feature = "day14-part1")]
fn run_day14_part1() {
    let input_strings = include_str!("../inputs/day14-input.txt");
    let result = day14::part1::process_data(input_strings);
    println!("daily14-part1 = {}", result); // 109654
}

#[cfg(feature = "day14-part2")]
fn run_day14_part2() {
    let input_strings = include_str!("../inputs/day14-input.txt");
    let result = day14::part2::process_data(input_strings);
    println!("daily14-part2 = {}", result); // 94876
}

#[cfg(feature = "day15-part1")]
fn run_day15_part1() {
    let input_strings = include_str!("../inputs/day15-input.txt");
    let result = day15::part1::process_data(input_strings);
    println!("daily15-part1 = {}", result); // 510013
}

#[cfg(feature = "day15-part2")]
fn run_day15_part2() {
    let input_strings = include_str!("../inputs/day15-input.txt");
    let result = day15::part2::process_data(input_strings);
    println!("daily15-part2 = {}", result); // 268497
}

#[cfg(feature = "day16-part1")]
fn run_day16_part1() {
    let input_strings = include_str!("../inputs/day16-input.txt");
    let result = day16::part1::process_data(input_strings);
    println!("daily16-part1 = {}", result); // 8125
}

#[cfg(feature = "day16-part2")]
fn run_day16_part2() {
    let input_strings = include_str!("../inputs/day16-input.txt");
    let result = day16::part2::process_data(input_strings);
    println!("daily16-part2 = {}", result); // 8489
}

#[cfg(feature = "day17-part1")]
fn run_day17_part1() {
    let input_strings = include_str!("../inputs/day17-input.txt");
    let result = day17::part1::process_data(input_strings);
    println!("daily17-part1 = {}", result); // 1013
}

#[cfg(feature = "day17-part2")]
fn run_day17_part2() {
    let input_strings = include_str!("../inputs/day17-input.txt");
    let result = day17::part2::process_data(input_strings);
    println!("daily17-part2 = {}", result); // 1215
}

#[cfg(feature = "day18-part1")]
fn run_day18_part1() {
    let input_strings = include_str!("../inputs/day18-input.txt");
    let result = day18::part1::process_data(input_strings);
    println!("daily18-part1 = {}", result); // 70253
}

#[cfg(feature = "day18-part2")]
fn run_day18_part2() {
    let input_strings = include_str!("../inputs/day18-input.txt");
    let result = day18::part2::process_data(input_strings);
    println!("daily18-part2 = {}", result); // 131265059885080
}

#[cfg(feature = "day19-part1")]
fn run_day19_part1() {
    let input_strings = include_str!("../inputs/day19-input.txt");
    let result = day19::part1::process_data(input_strings);
    println!("daily19-part1 = {}", result); // 480738
}

#[cfg(feature = "day19-part2")]
fn run_day19_part2() {
    let input_strings = include_str!("../inputs/day19-input.txt");
    let result = day19::part2::process_data(input_strings);
    println!("daily19-part2 = {}", result); // 131550418841958
}

#[cfg(feature = "day20-part1")]
fn run_day20_part1() {
    let input_strings = include_str!("../inputs/day20-input.txt");
    let result = day20::part1::process_data(input_strings);
    println!("daily20-part1 = {}", result); // 684125385
}

#[cfg(feature = "day20-part2")]
fn run_day20_part2() {
    let input_strings = include_str!("../inputs/day20-input.txt");
    let result = day20::part2::process_data(input_strings);
    println!("daily20-part2 = {}", result); // 225872806380073
}

#[cfg(feature = "day22-part1")]
fn run_day22_part1() {
    let input_strings = include_str!("../inputs/day22-input.txt");
    let result = day22::part1::process_data(input_strings);
    println!("daily22-part1 = {}", result); //
}
