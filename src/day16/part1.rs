use std::collections::HashSet;

use crate::utils::matrix::display_grid;

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
    display_grid(&grid);

    let mut seen = HashSet::new();
    let mut direction_seen = HashSet::new();
    let start = (0, 0);
    seen.insert(start);
    direction_seen.insert((start, Direction::Right));
    walk(
        &grid,
        start,
        &Direction::Right,
        &mut seen,
        &mut direction_seen,
    );

    // debug
    let mut test = vec![vec!['.'; grid[0].len()]; grid.len()];
    for (x, y) in seen.iter() {
        test[*x as usize][*y as usize] = '#';
    }
    display_grid(&test);

    dbg!(direction_seen.len());

    seen.len() as u64
}

fn walk(
    grid: &[Vec<char>],
    current: (i32, i32),
    direction: &Direction,
    seen: &mut HashSet<(i32, i32)>,
    d_seen: &mut HashSet<((i32, i32), Direction)>,
) {
    // validate index bounds
    let next = direction.next(current);
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

    // seen (beam)
    seen.insert(next);

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
        let d_seen_key = (next, d.clone());
        if d_seen.contains(&d_seen_key) {
            continue;
        }
        d_seen.insert(d_seen_key);
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
        assert_eq!(46, process_data(input));
    }
}
