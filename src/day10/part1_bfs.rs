use std::collections::VecDeque;

use anyhow::{bail, Result};

#[derive(Debug, Clone)]
enum Tile {
    Ground,
    Start((i32, i32)),
    Direction([Direction; 2]),
    _Invalid,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn position(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

const DIRECTION_VECTORS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

pub fn process_data(input: &str) -> Result<u32> {
    let mut start = None;
    let tiles = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Tile::Ground,
                    'S' => {
                        let (i, j) = (i as i32, j as i32);
                        start = Some((i, j));
                        Tile::Start((i, j))
                    }
                    '|' => Tile::Direction([Direction::North, Direction::South]),
                    '-' => Tile::Direction([Direction::East, Direction::West]),
                    'L' => Tile::Direction([Direction::North, Direction::East]),
                    'J' => Tile::Direction([Direction::North, Direction::West]),
                    '7' => Tile::Direction([Direction::South, Direction::West]),
                    'F' => Tile::Direction([Direction::South, Direction::East]),
                    _ => Tile::_Invalid,
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<_>>();

    if start.is_none() {
        bail!("start not found");
    }
    let start = start.unwrap();
    let start_tile = Tile::Start(start);

    let mut seen = vec![vec![false; tiles[0].len()]; tiles.len()];
    let mut prev: Vec<(usize, usize)> = Vec::new();
    let mut queue = VecDeque::<(Tile, i32, i32)>::new();
    seen[start.0 as usize][start.1 as usize] = true;
    prev.push((start.0 as usize, start.1 as usize));
    DIRECTION_VECTORS.iter().for_each(|direction| {
        let (i, j) = calculate_position(start, direction.position());
        if is_overflowed(i, j, &tiles) {
            return;
        }
        if let Tile::Direction([d1, d2]) = &tiles[i as usize][j as usize] {
            if is_direction_linked(direction, d1) || is_direction_linked(direction, d2) {
                queue.push_back((start_tile.clone(), i, j));
            }
        }
    });

    while !queue.is_empty() {
        walk(&tiles, &mut seen, &mut prev, &mut queue);
    }

    Ok(prev.len() as u32 / 2)
}

fn calculate_position(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn is_overflowed(i: i32, j: i32, tiles: &[Vec<Tile>]) -> bool {
    i < 0 || j < 0 || i >= tiles.len() as i32 || j >= tiles[0].len() as i32
}

fn is_linked(direction: &Direction, pre_tile: &Tile, current_position: (i32, i32)) -> bool {
    if let Tile::Direction([d1, d2]) = pre_tile {
        return is_direction_linked(direction, d1) || is_direction_linked(direction, d2);
    }

    if let Tile::Start((i, j)) = pre_tile {
        return calculate_position(current_position, direction.position()) == (*i, *j);
    }

    false
}

fn is_direction_linked(d1: &Direction, d2: &Direction) -> bool {
    match d1 {
        Direction::North => d2 == &Direction::South,
        Direction::South => d2 == &Direction::North,
        Direction::East => d2 == &Direction::West,
        Direction::West => d2 == &Direction::East,
    }
}

fn walk(
    tiles: &[Vec<Tile>],
    seen: &mut [Vec<bool>],
    prev: &mut Vec<(usize, usize)>,
    queue: &mut VecDeque<(Tile, i32, i32)>,
) {
    let position = queue.pop_front();
    if position.is_none() {
        return;
    }

    let (prev_title, i, j) = position.unwrap();
    if is_overflowed(i, j, tiles) {
        return;
    }

    let (i, j) = (i as usize, j as usize);
    if seen[i][j] {
        return;
    }
    seen[i][j] = true;

    let tile = &tiles[i][j];

    if let &Tile::Start((i, j)) = tile {
        let (i, j) = (i as usize, j as usize);
        prev.push((i, j));
        return;
    }

    if let Tile::Direction([d1, d2]) = tile {
        if is_linked(d1, &prev_title, (i as i32, j as i32))
            || is_linked(d2, &prev_title, (i as i32, j as i32))
        {
            prev.push((i, j));
            let (a, b) = calculate_position((i as i32, j as i32), d1.position());
            queue.push_back((tile.clone(), a, b));
            let (a, b) = calculate_position((i as i32, j as i32), d2.position());
            queue.push_back((tile.clone(), a, b));
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        4
    )]
    #[case(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        4
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        8
    )]
    #[case(
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        8
    )]
    fn test_process_data(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, process_data(input).unwrap());
    }
}
