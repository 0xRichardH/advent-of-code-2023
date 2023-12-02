use advent_of_code_2023::day01;

fn main() -> anyhow::Result<()> {
    let input_strings = include_str!("../inputs/day01-input.txt");
    let result = day01::calibrate(input_strings)?;
    println!("{:?}", result);

    Ok(())
}
