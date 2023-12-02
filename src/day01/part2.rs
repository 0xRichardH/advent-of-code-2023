use anyhow::bail;

use super::trie::TrieNode;

const ALPHABET_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn calibrate(input: &str) -> anyhow::Result<u32> {
    let mut trie = TrieNode::new();
    for alphabet_num in ALPHABET_NUMBERS {
        trie.insert(alphabet_num);
    }

    let result = input
        .lines()
        .flat_map(|s| parse_numbers_from_str(&trie, s).ok())
        .sum::<u32>();

    Ok(result)
}

fn parse_numbers_from_str(trie: &TrieNode, input: &str) -> anyhow::Result<u32> {
    let mut tmp_alphabet = String::new();
    let numbers = input
        .chars()
        .flat_map(|c| {
            if c.is_ascii_digit() {
                tmp_alphabet.clear();
                return c.to_digit(10);
            }

            if c.is_alphabetic() {
                tmp_alphabet.push(c);

                while !tmp_alphabet.is_empty() {
                    let node_result = trie.search(&tmp_alphabet);
                    if node_result.is_none() {
                        tmp_alphabet.remove(0);
                        continue;
                    }

                    if node_result.unwrap().is_end_of_word() {
                        if let Some(num) = alphabet_to_number(tmp_alphabet.as_str()) {
                            tmp_alphabet.remove(0);
                            return Some(num);
                        }

                        tmp_alphabet.remove(0);
                    }

                    break;
                }
            }

            None
        })
        .collect::<Vec<u32>>();

    match numbers.len() {
        1 => {
            let num = numbers[0];
            Ok(num * 10 + num)
        }
        2.. => {
            let num1 = numbers[0];
            let num2 = numbers[numbers.len() - 1];
            Ok(num1 * 10 + num2)
        }
        _ => bail!("invalid numbers"),
    }
}

fn alphabet_to_number(alphabet: &str) -> Option<u32> {
    match alphabet {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        142
    )]
    #[case(
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        281
    )]
    fn it_shoud_calibrate_it_from_str(
        #[case] input: &str,
        #[case] expected: u32,
    ) -> anyhow::Result<()> {
        assert_eq!(expected, calibrate(input)?);
        Ok(())
    }

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("onespnbfninetgqmtbdmr17pmjt", 17)]
    #[case("94six8329", 99)]
    #[case("7nineseven", 77)]
    #[case("271", 21)]
    #[case("8sevenseveneightvgfdkdglq89", 89)]
    #[case("ntlznczfone7ninesixxtxbkvpkonebmbc", 11)]
    /// this test case is from the real input
    /// it tests two overlapping numbers
    /// where the second number should succeed
    #[case("fivezg8jmf6hrxnhgxxttwoneg", 51)]
    fn it_should_parse_numbers_from_str(#[case] input: &str, #[case] expected: u32) {
        let mut trie = TrieNode::new();
        for alphabet_num in ALPHABET_NUMBERS {
            trie.insert(alphabet_num);
        }
        assert_eq!(expected, parse_numbers_from_str(&trie, input).unwrap());
    }
}
