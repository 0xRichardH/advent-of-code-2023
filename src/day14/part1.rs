pub fn process_data(input: &str) -> u32 {
    let mut platforms = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    calculate_total_load(&mut platforms)
}

fn calculate_total_load(platforms: &mut [Vec<char>]) -> u32 {
    let len = platforms.len();
    let row_len = platforms[0].len();

    (0..row_len).fold(0, |total_load, j| {
        switch(platforms, (0, j), None);
        (0..len).fold(total_load, |load, i| {
            if platforms[i][j] == 'O' {
                load + len - i
            } else {
                load
            }
        })
    }) as u32
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
            switch(platforms, (ai + 1, bj), None)
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
        assert_eq!(136, process_data(input));
    }
}
