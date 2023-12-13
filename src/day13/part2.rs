pub fn process_data(input: &str) -> u32 {
    let patterns = parse_patterns(input);
    patterns.iter().fold(0, |acc, pattern| {
        let mut num = 0;

        num += get_mirror_count(pattern) * 100;

        num += get_mirror_count(transpose(pattern.to_vec()).as_slice());

        acc + num as u32
    })
}

fn parse_patterns(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.chars().collect()).collect())
        .collect()
}

fn get_mirror_count(pattern: &[Vec<char>]) -> usize {
    for i in 1..pattern.len() {
        let mut above = pattern[..i].to_vec();
        above.reverse();
        let below = &pattern[i..];

        let len = above.len().min(below.len());

        // you discover that every mirror has exactly one smudge: exactly one . or # should be the opposite type.
        // to check if we have exactly the one smudge
        let smudge_count = above[..len]
            .iter()
            .zip(&below[..len])
            .map(|(a, b)| {
                a.iter().zip(b.iter()).fold(0, |acc, (ac, bc)| {
                    if ac != bc {
                        return acc + 1;
                    }

                    acc
                })
            })
            .sum::<usize>();
        if smudge_count == 1 {
            return i;
        }
    }
    0
}

fn transpose<T: Copy>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    (0..len)
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(400, process_data(input));
    }
}
