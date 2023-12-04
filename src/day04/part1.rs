use anyhow::{bail, Result};
use regex::Regex;

pub fn process_data(input: &str) -> Result<u32> {
    let re = Regex::new(r"Card.+?\d?: (?<winning_nums>.+)? \| (?<owned_nums>.+)?")?;
    input.lines().map(|l| calcuate_points(&re, l)).sum()
}

fn calcuate_points(re: &Regex, game: &str) -> Result<u32> {
    if let Some(caps) = re.captures(game) {
        let winning_nums = caps["winning_nums"]
            .split_whitespace()
            .flat_map(|s| s.parse())
            .collect::<Vec<u32>>();
        let owned_nums = caps["owned_nums"]
            .split_whitespace()
            .flat_map(|s| s.parse())
            .collect::<Vec<u32>>();
        let winning_count = get_common_numbers_count(winning_nums, owned_nums);

        // The first match makes the card worth one point
        // and each match after the first doubles the point value of that card
        let points = match winning_count {
            0..=1 => winning_count,
            _ => 2u32.pow(winning_count - 1),
        };
        return Ok(points);
    }

    bail!("cannot calculate points from {}", game)
}

fn get_common_numbers_count(mut nums1: Vec<u32>, mut nums2: Vec<u32>) -> u32 {
    nums1.sort();
    nums2.sort();

    let mut j = 0;
    let mut i = 0;
    let mut counter = 0u32;

    while i < nums1.len() && j < nums2.len() {
        match nums1[i].cmp(&nums2[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Equal => {
                i += 1;
                j += 1;
                counter += 1
            }
            std::cmp::Ordering::Greater => j += 1,
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    #[case("Card  7: 57 93  4  6  2 34 18 80 99  9 |  9 53 58 19 35  6 46 87 86 36 59 17 26 54 39 52 99 20 69 18 25 30 34 41 42", 16)]
    #[case("Card   8:  2 15 17 11 64 59 45 41 61 19 |  4 36 62 43 94 41 24 25 13 83 97 86 61 90 67  7 15 58 18 19 38 17 49 52 37", 16)]
    fn it_calcuates_points(#[case] input: &str, #[case] expected: u32) {
        let re = Regex::new(r"Card.+?\d?: (?<winning_nums>.+)? \| (?<owned_nums>.+)?").unwrap();
        assert_eq!(calcuate_points(&re, input).unwrap(), expected);
    }

    #[test]
    fn it_should_process_data() {
        let sum = process_data(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        )
        .unwrap();
        assert_eq!(13, sum);
    }
}
