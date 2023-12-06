use anyhow::{anyhow, Result};
use nom::{
    character::complete::{self, newline, space1},
    multi::many1,
    sequence::preceded,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

pub fn process_data(input: &str) -> Result<u32> {
    let (_, (times, distances)) =
        parse_times_and_distances(input).map_err(|e| anyhow!("Failed to parse input: {}", e))?;

    let margin_of_error = distances
        .into_iter()
        .zip(times)
        .map(|(distance, time)| winning_ways_of_race(distance, time))
        .product::<u32>();

    Ok(margin_of_error)
}

fn parse_times_and_distances(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, times) = tag("Time:").precedes(parse_nums).parse(input)?;
    let (input, _) = newline(input)?;
    let (input, distances) = tag("Distance:").precedes(parse_nums).parse(input)?;

    Ok((input, (times, distances)))
}

fn parse_nums(input: &str) -> IResult<&str, Vec<u32>> {
    many1(preceded(space1, complete::u32)).parse(input)
}

fn winning_ways_of_race(distance: u32, time: u32) -> u32 {
    (1..time).fold(0, |acc, hold| {
        if hold * (time - hold) > distance {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process_data() -> Result<()> {
        let result = process_data(
            "Time:      7  15   30
Distance:  9  40  200",
        )?;
        assert_eq!(288, result);

        Ok(())
    }

    #[rstest]
    #[case(
        "Time:      7  15   30
Distance:  9  40  200", [7u32, 15, 30], [9u32, 40, 200]
    )]
    fn test_parse_times_and_distances(
        #[case] input: &str,
        #[case] expected_times: [u32; 3],
        #[case] expected_distances: [u32; 3],
    ) {
        let (_, (times, distances)) = parse_times_and_distances(input).unwrap();
        assert_eq!(expected_times, times.as_slice());
        assert_eq!(expected_distances, distances.as_slice());
    }
}
