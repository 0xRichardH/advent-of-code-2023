use std::collections::HashMap;

const CYCLE: usize = 1000000000;

pub fn process_data(input: &str) -> u32 {
    let mut platforms = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut scores = Vec::new();
    let mut pattern = HashMap::new();
    for _ in 0..1000 {
        cycle(&mut platforms);
        scores.push(score(&platforms));
        let pattern_key = platforms
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<String>();
        pattern
            .entry(pattern_key.clone())
            .and_modify(|e| *e += 1)
            .or_insert(1);
        if pattern[&pattern_key] > 2 {
            break;
        }
    }

    // the number of unique states (*v == 1) before the start of the repeating pattern.
    let offset = pattern.values().filter(|&v| *v == 1).count();
    // the length of the repeating pattern, determined by counting the states that appear more than once
    let cycle_length = pattern.values().filter(|v| **v > 1).count();

    // index calculation
    // (CYCLE - offset - 1) adjusts the cycle number to account for the initial unique states (the offset) and aligns the cycle count to a 0-based index.
    // The modulo operation % cycle_length is used to find the position within the repeating cycle.
    let index = (CYCLE - offset - 1) % cycle_length;

    scores[offset + index]
}

fn cycle(platforms: &mut Vec<Vec<char>>) {
    // 1 north
    roll(platforms);

    // 2 west
    rotate_right(platforms);
    roll(platforms);
    rotate_left(platforms);

    // 3 south
    rotate_180(platforms);
    roll(platforms);
    rotate_180(platforms);

    // 4 east
    rotate_left(platforms);
    roll(platforms);
    rotate_right(platforms);
}

fn roll(platforms: &mut [Vec<char>]) {
    let row_len = platforms[0].len();

    for j in 0..row_len {
        switch(platforms, (0, j), None);
    }
}

fn score(platforms: &[Vec<char>]) -> u32 {
    let len = platforms.len();
    let row_len = platforms[0].len();

    (0..row_len).fold(0, |total_load, j| {
        (0..len).fold(total_load, |load, i| {
            if platforms[i][j] == 'O' {
                load + len - i
            } else {
                load
            }
        })
    }) as u32
}

fn rotate_left(platforms: &mut Vec<Vec<char>>) {
    let len = platforms[0].len();
    *platforms = (0..len)
        .rev()
        .map(|i| platforms.iter().map(|v| v[i]).collect())
        .collect();
}

fn rotate_right(platforms: &mut Vec<Vec<char>>) {
    let len = platforms.len();
    *platforms = (0..len)
        .map(|i| platforms.iter().rev().map(|v| v[i]).collect())
        .collect();
}

fn rotate_180(platforms: &mut Vec<Vec<char>>) {
    platforms.reverse();

    for row in platforms.iter_mut() {
        row.reverse();
    }
}

fn switch(platforms: &mut [Vec<char>], (i, j): (usize, usize), dot: Option<(usize, usize)>) {
    let len = platforms.len();
    if i >= len {
        return;
    }

    if dot.is_none() {
        let dot = if platforms[i][j] == '.' {
            Some((i, j))
        } else {
            None
        };

        switch(platforms, (i + 1, j), dot);
        return;
    }

    match platforms[i][j] {
        '.' => switch(platforms, (i + 1, j), dot),
        '#' => switch(platforms, (i + 1, j), None),
        'O' => {
            let (ai, bj) = dot.unwrap();
            platforms[i][j] = '.';
            platforms[ai][bj] = 'O';
            switch(platforms, (i + 1, j), Some((ai + 1, bj)))
        }
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prcess_data() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(64, process_data(input));
    }
}
