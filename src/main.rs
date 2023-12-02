use advent_of_code_2023::day01;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "day01-part1")]
    run_day01_part01()?;

    Ok(())
}

#[cfg(feature = "day01-part1")]
fn run_day01_part01() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day01-input.txt");
    let result = day01::calibrate(input_strings)?;
    println!("daily01-part1 = {}", result);

    Ok(())
}
