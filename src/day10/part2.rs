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

    let mut seen = vec![vec![false; tiles[0].len()]; tiles.len()];
    let mut prev: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; tiles[0].len()]; tiles.len()];
    let mut queue = VecDeque::<(Tile, i32, i32)>::new();
    seen[start.0 as usize][start.1 as usize] = true;
    prev[start.0 as usize][start.1 as usize] = Some((start.0 as usize, start.1 as usize));
    DIRECTION_VECTORS.iter().for_each(|direction| {
        let (i, j) = calculate_position(start, direction.position());
        if is_overflowed(i, j, &tiles) {
            return;
        }
        if let Tile::Direction([d1, d2]) = &tiles[i as usize][j as usize] {
            if is_direction_linked(direction, d1) || is_direction_linked(direction, d2) {
                queue.push_back((Tile::Start(start), i, j));
            }
        }
    });

    while !queue.is_empty() {
        walk(&tiles, &mut seen, &mut prev, &mut queue);
    }

    let counter = tiles
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(j, _)| prev[i][*j].is_none())
                .filter(|(j, _)| {
                    let inversions_count = count_inversions((i, *j), &tiles, &prev);
                    inversions_count % 2 == 1
                })
                .count()
        })
        .sum::<usize>();

    Ok(counter as u32)
}

// Count the number of "inversions" in a row
fn count_inversions(
    (i, j): (usize, usize),
    tiles: &[Vec<Tile>],
    prev: &[Vec<Option<(usize, usize)>>],
) -> u32 {
    tiles[i][..j]
        .iter()
        .enumerate()
        .filter(|(idx, _)| prev[i][*idx].is_some())
        .fold(0, |acc, (_idx, t)| {
            if let Tile::Direction([d1, d2]) = t {
                if d1 == &Direction::North || d2 == &Direction::North {
                    return acc + 1;
                }
            }

            acc
        })
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
    prev: &mut [Vec<Option<(usize, usize)>>],
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
    if let Tile::Direction([d1, d2]) = tile {
        if is_linked(d1, &prev_title, (i as i32, j as i32))
            || is_linked(d2, &prev_title, (i as i32, j as i32))
        {
            prev[i][j] = Some((i, j));
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
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        4
    )]
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        8
    )]
    #[case(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        10
    )]
    fn test_process_data(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, process_data(input).unwrap());
    }
}
