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
