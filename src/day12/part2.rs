use std::collections::HashMap;

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, space1},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

pub fn process_data(input: &str) -> Result<u64> {
    input.lines().try_fold(0u64, |acc, line| {
        calculate_arrangement(line).map(|x| acc + x)
    })
}

fn calculate_arrangement(input: &str) -> Result<u64> {
    let (_, (mut records, mut nums)) =
        parse_record(input).map_err(|e| anyhow!("failed to parse input: {:?}", e))?;

    records.push('?');
    records = records.repeat(5);
    records.pop();
    nums = nums.repeat(5);

    let mut dp_mem = HashMap::<(usize, usize, u64), u64>::new();
    let result = count_arrangement(&records, &nums, 0, 0, 0, &mut dp_mem);
    Ok(result)
}

fn parse_record(input: &str) -> IResult<&str, (Vec<char>, Vec<u64>)> {
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

fn parse_nums(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(","), complete::u64).parse(input)
}

fn count_arrangement(
    record: &[char],
    nums: &[u64],
    record_idx: usize,
    nums_idx: usize,
    current_hashtag_count: u64,
    dp_mem: &mut HashMap<(usize, usize, u64), u64>,
) -> u64 {
    let dp_mem_key = (record_idx, nums_idx, current_hashtag_count);
    if let Some(result) = dp_mem.get(&dp_mem_key) {
        return *result;
    }

    // we have reached the end of the record
    if record_idx == record.len() {
        if nums_idx == nums.len() && current_hashtag_count == 0 {
            return 1;
        }

        if nums_idx == nums.len() - 1 && current_hashtag_count == nums[nums_idx] {
            return 1;
        }

        return 0;
    }

    let count = ['.', '#'].iter().fold(0, |acc, c| {
        if record[record_idx] != '?' && record[record_idx] != *c {
            return acc;
        }

        let counter = match c {
            '.' => {
                if current_hashtag_count == 0 || nums_idx >= nums.len() {
                    return count_arrangement(record, nums, record_idx + 1, nums_idx, 0, dp_mem);
                }

                if record_idx < record.len() && nums[nums_idx] == current_hashtag_count {
                    return count_arrangement(
                        record,
                        nums,
                        record_idx + 1,
                        nums_idx + 1,
                        0,
                        dp_mem,
                    );
                }

                return acc;
            }
            '#' => {
                if nums_idx >= nums.len() {
                    return acc;
                }

                count_arrangement(
                    record,
                    nums,
                    record_idx + 1,
                    nums_idx,
                    current_hashtag_count + 1,
                    dp_mem,
                )
            }
            _ => acc,
        };

        acc + counter
    });

    dp_mem.insert(dp_mem_key, count);
    count
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
        assert_eq!(525152, process_data(input).unwrap());
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    fn test_calculate_arrangement(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(expected, calculate_arrangement(input).unwrap());
    }
}
