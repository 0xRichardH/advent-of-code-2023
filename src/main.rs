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
