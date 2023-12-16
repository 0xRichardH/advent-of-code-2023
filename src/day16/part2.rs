use rayon::prelude::*;
use std::collections::HashSet;

const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    fn next(&self, current: (i32, i32)) -> (i32, i32) {
        let position = match self {
            Direction::Up => UP,
            Direction::Down => DOWN,
            Direction::Left => LEFT,
            Direction::Right => RIGHT,
        };
        (current.0 + position.0, current.1 + position.1)
    }
}

pub fn process_data(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut counter = 0;

    let counter_result = (0..grid.len())
        .into_par_iter()
        .map(|x| {
            let (a, b) = rayon::join(
                || get_total_beams_count(&grid, (x as i32, 0), &Direction::Right),
                || {
                    get_total_beams_count(
                        &grid,
                        (x as i32, grid[0].len() as i32 - 1),
                        &Direction::Left,
                    )
                },
            );
            a.max(b)
        })
        .max();
    if let Some(c) = counter_result {
        counter = c.max(counter);
    }

    let counter_result = (0..grid[0].len())
        .into_par_iter()
        .map(|y| {
            let (a, b) = rayon::join(
                || get_total_beams_count(&grid, (0, y as i32), &Direction::Down),
                || get_total_beams_count(&grid, (grid.len() as i32 - 1, y as i32), &Direction::Up),
            );
            a.max(b)
        })
        .max();
    if let Some(c) = counter_result {
        counter = c.max(counter);
    }

    counter
}

fn get_total_beams_count(grid: &[Vec<char>], start: (i32, i32), direction: &Direction) -> u64 {
    let mut seen = HashSet::new();
    let mut direction_seen = HashSet::new();

    walk(grid, start, direction, &mut seen, &mut direction_seen);

    seen.len() as u64
}

fn walk(
    grid: &[Vec<char>],
    next: (i32, i32),
    direction: &Direction,
    seen: &mut HashSet<(i32, i32)>,
    d_seen: &mut HashSet<((i32, i32), Direction)>,
) {
    // validate index bounds
    if next.0 < 0 || next.1 < 0 {
        return;
    }

    let (x, y) = (next.0 as usize, next.1 as usize);
    let row = grid.get(x);
    if row.is_none() {
        return;
    }
    let tile = row.unwrap().get(y);
    if tile.is_none() {
        return;
    }
    let tile = tile.unwrap();

    // seen
    seen.insert(next);
    let d_seen_key = (next, direction.clone());
    if d_seen.contains(&d_seen_key) {
        return;
    }
    d_seen.insert(d_seen_key);

    // get the next direction
    let mut directions = Vec::with_capacity(2);
    match tile {
        '.' => directions.push(direction),
        '/' => match direction {
            Direction::Up => directions.push(&Direction::Right),
            Direction::Down => directions.push(&Direction::Left),
            Direction::Right => directions.push(&Direction::Up),
            Direction::Left => directions.push(&Direction::Down),
        },
        '\\' => match direction {
            Direction::Up => directions.push(&Direction::Left),
            Direction::Down => directions.push(&Direction::Right),
            Direction::Right => directions.push(&Direction::Down),
            Direction::Left => directions.push(&Direction::Up),
        },
        '-' => match direction {
            Direction::Up | Direction::Down => {
                // split
                directions.push(&Direction::Left);
                directions.push(&Direction::Right);
            }
            Direction::Left | Direction::Right => directions.push(direction),
        },
        '|' => match direction {
            Direction::Up | Direction::Down => directions.push(direction),
            Direction::Left | Direction::Right => {
                // split
                directions.push(&Direction::Up);
                directions.push(&Direction::Down);
            }
        },
        _ => (),
    }

    for d in directions {
        let next = d.next(next);
        walk(grid, next, d, seen, d_seen);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(51, process_data(input));
    }
}
