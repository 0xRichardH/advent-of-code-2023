use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, space1},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

pub fn process_data(input: &str) -> Result<u32> {
    input.lines().try_fold(0u32, |acc, line| {
        calculate_arrangement(line).map(|x| acc + x)
    })
}

fn calculate_arrangement(input: &str) -> Result<u32> {
    let (_, (records, nums)) =
        parse_record(input).map_err(|e| anyhow!("failed to parse input: {:?}", e))?;

    let result = count_arrangement(&records, &nums, 0);
    Ok(result)
}

fn parse_record(input: &str) -> IResult<&str, (Vec<char>, Vec<u32>)> {
    separated_pair(parse_springs_conditions, space1, parse_nums).parse(input)
}

fn parse_springs_conditions(input: &str) -> IResult<&str, Vec<char>> {
    many1(alt((
        complete::char('?'),
        complete::char('#'),
        complete::char('.'),
    )))
    .parse(input)
}

fn parse_nums(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), complete::u32).parse(input)
}

fn count_arrangement(record: &[char], nums: &[u32], idx: usize) -> u32 {
    if idx == record.len() {
        if is_valid(record, nums) {
            return 1;
        } else {
            return 0;
        }
    }

    if record[idx] == '?' {
        let mut new_record = record.to_vec();
        return ['.', '#'].iter().fold(0, |acc, c| {
            new_record[idx] = *c;
            acc + count_arrangement(&new_record, nums, idx + 1)
        });
    }

    count_arrangement(record, nums, idx + 1)
}

fn is_valid(record: &[char], nums: &[u32]) -> bool {
    let mut counter = 0;
    let mut seen = Vec::<u32>::new();
    for c in record {
        match c {
            '.' => {
                if counter > 0 {
                    seen.push(counter);
                    counter = 0;
                }
            }
            '#' => {
                counter += 1;
            }
            _ => return false,
        }
    }

    if counter > 0 {
        seen.push(counter);
    }

    seen == nums
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process_data() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(21, process_data(input).unwrap());
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_calculate_arrangement(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, calculate_arrangement(input).unwrap());
    }

    #[rstest]
    #[case(&['#', '.', '#', '.', '#', '#', '#'], &[1, 1, 3], true)]
    #[case(&['.', '#', '#', '#', '.', '#', '#', '.', '#', '.', '.', '.'], &[3,2,1],  true)]
    #[case(&['.', '#', '#', '#', '?', '#', '#', '.', '#', '.', '.', '.'], &[3,2,1],  false)]
    #[case(&['.', '#', '#', '#', '#', '#', '#', '.', '#', '.', '.', '.'], &[3,2,1],  false)]
    fn test_is_valid(#[case] record: &[char], #[case] nums: &[u32], #[case] expected: bool) {
        assert_eq!(expected, is_valid(record, nums));
    }
}
