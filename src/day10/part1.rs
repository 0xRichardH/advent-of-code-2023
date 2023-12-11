use anyhow::{bail, Result};

#[derive(Debug)]
enum Tile {
    Ground,
    Start,
    Direction([Direction; 2]),
    _Invalid,
}

#[derive(Debug)]
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
                        start = Some((i, j));
                        Tile::Start
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

    let positions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    let mut seen = vec![vec![false; tiles[0].len()]; tiles.len()];

    let start = start.unwrap();
    seen[start.0][start.1] = true;
    let steps = positions.iter().fold(0, |steps, p| {
        let current = (start.0 as i32, start.1 as i32);
        let mut ss: Vec<(i32, i32)> = Vec::new();
        find_farthest_steps(current, p, &tiles, &mut ss, &mut seen);
        steps.max(ss.len() as u32)
    });

    Ok(steps / 2)
}

fn find_farthest_steps(
    current: (i32, i32),
    next_d: &Direction,
    tiles: &Vec<Vec<Tile>>,
    steps: &mut Vec<(i32, i32)>,
    seen: &mut Vec<Vec<bool>>,
) {
    steps.push(current);

    let (i, j) = calculate_position(current, next_d.position());

    // if the i, j if overflowed, the pop
    if i < 0 || j < 0 {
        steps.pop();
        return;
    }
    let (i, j) = (i as usize, j as usize);

    if i >= tiles.len() || j >= tiles[0].len() {
        steps.pop();
        return;
    }

    // Handle the tile
    match &tiles[i][j] {
        Tile::Ground => {
            steps.pop();
        }
        Tile::Direction([a, b]) => {
            if seen[i][j] {
                steps.pop();
                return;
            }
            seen[i][j] = true;
            let new_current = (i as i32, j as i32);

            // not being able to get back to the previous position
            if calculate_position(new_current, a.position()) != current
                && calculate_position(new_current, b.position()) != current
            {
                steps.pop();
            }

            // recursion: go to next directions
            find_farthest_steps(new_current, a, tiles, steps, seen);
            find_farthest_steps(new_current, b, tiles, steps, seen);
        }
        _ => (),
    }
}

fn calculate_position(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
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
