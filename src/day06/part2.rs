use anyhow::{anyhow, Result};
use nom::{
    character::complete::{digit1, newline, space1},
    multi::many1,
    sequence::preceded,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

pub fn process_data(input: &str) -> Result<u64> {
    let (_, (time, distance)) =
        parse_time_and_distance(input).map_err(|e| anyhow!("Failed to parse input: {}", e))?;

    let time = time.parse()?;
    let distance = distance.parse()?;
    let margin_of_error = winning_ways_of_race(distance, time);

    Ok(margin_of_error)
}

fn parse_time_and_distance(input: &str) -> IResult<&str, (String, String)> {
    let (input, time) = tag("Time:").precedes(parse_nums).parse(input)?;
    let (input, _) = newline(input)?;
    let (input, distance) = tag("Distance:").precedes(parse_nums).parse(input)?;

    Ok((input, (time, distance)))
}

fn parse_nums(input: &str) -> IResult<&str, String> {
    let (input, digits) = many1(preceded(space1, digit1)).parse(input)?;
    Ok((input, digits.concat()))
}

fn winning_ways_of_race(distance: u64, time: u64) -> u64 {
    let start = (distance as f64 / time as f64).ceil() as u64;
    let end = time - start;
    (start..=end).fold(0, |acc, hold| {
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
        assert_eq!(71503, result);

        Ok(())
    }

    #[rstest]
    #[case(
        "Time:      7  15   30
Distance:  9  40  200",
        "71530",
        "940200"
    )]
    fn test_parse_time_and_distance(
        #[case] input: &str,
        #[case] expected_times: String,
        #[case] expected_distances: String,
    ) {
        let (_, (time, distance)) = parse_time_and_distance(input).unwrap();
        assert_eq!(expected_times, time);
        assert_eq!(expected_distances, distance);
    }
}
