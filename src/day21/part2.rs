use std::collections::{HashSet, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn process_data(input: &str, steps: isize) -> i64 {
    let (start, grid) = parse_input(input);
    let Some(start) = start else {
        return 0;
    };

    fit_quadratic(start, steps, &grid)
}

fn parse_input(input: &str) -> (Option<(isize, isize)>, Vec<Vec<char>>) {
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
                        start = Some((i as isize, j as isize))
                    }
                    c
                })
                .collect()
        })
        .collect();

    (start, grid)
}

fn fit_quadratic(start_position: (isize, isize), steps: isize, grid: &[Vec<char>]) -> i64 {
    let height = grid.len() as isize;
    let remainder = steps % height; // it is 65

    let mut start = 0;
    let mut prev_start = 0;
    let mut values = Vec::new();
    let mut counter = 0;
    while values.len() < 3 {
        counter += 1;

        let visited_count = finding(grid, start_position, counter) as i64;
        if counter >= remainder && (counter - remainder) % height == 0 {
            let delta = visited_count - start;
            let step = [visited_count, delta, delta - prev_start];

            values.push(step[values.len()]);
            start = visited_count;
            prev_start = delta;
        }
    }

    let a = values[2] / 2;
    let b = values[1] - 3 * a;
    let c = values[0] - a - b;

    let n = (1 + steps / height) as i64;

    a * n * n + b * n + c
}

fn finding(grid: &[Vec<char>], start: (isize, isize), steps: isize) -> isize {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    let mut prev: HashSet<(isize, isize)> = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start, steps));

    while let Some(((i, j), steps)) = q.pop_front() {
        let spot = grid[i.rem_euclid(height) as usize][j.rem_euclid(width) as usize];
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
            // if ni < 0 || nj < 0 || ni >= grid.len() as i32 || nj >= grid[0].len() as i32 {
            //     continue;
            // }

            let point = (ni as isize, nj as isize);
            q.push_back((point, steps - 1));
        }
    }

    prev.len() as isize
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(6, 16)]
    #[case(10, 50)]
    fn test_process_data(#[case] steps: isize, #[case] expected: i64) {
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
        assert_eq!(expected, process_data(input, steps));
    }
}
