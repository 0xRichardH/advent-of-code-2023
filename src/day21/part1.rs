use std::collections::{HashSet, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn process_data(input: &str, steps: usize) -> usize {
    let (start, grid) = parse_input(input);
    let Some(start) = start else {
        return 0;
    };

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut prev: HashSet<(usize, usize)> = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start, steps));

    while let Some(((i, j), steps)) = q.pop_front() {
        let spot = grid[i][j];
        if spot == '#' {
            continue;
        }

        if !seen.insert((i, j)) {
            continue;
        }
        if steps % 2 == 0 {
            prev.insert((i, j));
        }

        if steps == 0 {
            continue;
        }

        for (di, dj) in DIRECTIONS {
            let (ni, nj) = (i as i32 + di, j as i32 + dj);
            if ni < 0 || nj < 0 || ni >= grid.len() as i32 || nj >= grid[0].len() as i32 {
                continue;
            }
            let point = (ni as usize, nj as usize);
            q.push_back((point, steps - 1));
        }
    }

    // dbg!(&prev);
    prev.len()
}

fn parse_input(input: &str) -> (Option<(usize, usize)>, Vec<Vec<char>>) {
    let mut start = None;
    let grid = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.trim()
                .chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start = Some((i, j))
                    }
                    c
                })
                .collect()
        })
        .collect();

    (start, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(16, process_data(input, 6));
    }
}
