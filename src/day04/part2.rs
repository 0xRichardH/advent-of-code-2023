use std::collections::HashMap;

use anyhow::bail;
use anyhow::Result;
use regex::Regex;

pub fn process_data(input: &str) -> anyhow::Result<u32> {
    let re = Regex::new(r"Card.+?(?<card_no>\d)?: (?<winning_nums>.+)? \| (?<owned_nums>.+)?")?;
    let mut scratchcards = HashMap::new();

    input
        .lines()
        .try_for_each(|l| collect_scratchcards(&re, l, &mut scratchcards))?;

    let card_count = scratchcards.len();
    let mut cards = vec![0u32; card_count];
    for n in (1..=card_count).rev() {
        let no = n as u32;
        if let Some(sub_cards) = scratchcards.get(&no) {
            let mut count = 1u32;
            for &c in sub_cards {
                let c = (c - 1) as usize;
                count += cards[c];
            }
            cards[n - 1] = count;
        }
    }

    Ok(cards.iter().sum())
}

fn collect_scratchcards(
    re: &Regex,
    game: &str,
    scratchcards: &mut HashMap<u32, Vec<u32>>,
) -> Result<()> {
    if let Some(caps) = re.captures(game) {
        let card_no = caps["card_no"].parse::<u32>()?;
        let winning_nums = caps["winning_nums"]
            .split_whitespace()
            .flat_map(|s| s.parse())
            .collect::<Vec<u32>>();
        let owned_nums = caps["owned_nums"]
            .split_whitespace()
            .flat_map(|s| s.parse())
            .collect::<Vec<u32>>();
        let winning_count = get_common_numbers_count(winning_nums, owned_nums);

        scratchcards.insert(card_no, (card_no + 1..=winning_count + card_no).collect());

        return Ok(());
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
    use super::*;

    #[test]
    fn it_processes_data() -> anyhow::Result<()> {
        let input_str = "Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card  5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, process_data(input_str)?);

        Ok(())
    }
}
