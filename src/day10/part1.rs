use anyhow::{bail, Result};

#[derive(Debug)]
enum Tile {
    Ground,
    Start,
    Direction([Direction; 2]),
    Invalid,
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
                    _ => Tile::Invalid,
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<_>>();

    if start.is_none() {
        bail!("start not found");
    }

    let positions = [
        Direction::North.position(),
        Direction::South.position(),
        Direction::East.position(),
        Direction::West.position(),
    ];
    let mut seen = vec![vec![false; tiles[0].len()]; tiles.len()];

    let start = start.unwrap();
    seen[start.0][start.1] = true;
    let steps = positions.iter().fold(0, |steps, p| {
        let position = calculate_position((start.0 as i32, start.1 as i32), *p);
        if let Some(s) = find_farthest_steps(position, &tiles, 0, &mut seen, start) {
            steps.max(s)
        } else {
            steps
        }
    });

    Ok(steps / 2)
}

fn find_farthest_steps(
    position: (i32, i32),
    tiles: &Vec<Vec<Tile>>,
    mut steps: u32,
    seen: &mut Vec<Vec<bool>>,
    init_position: (usize, usize),
) -> Option<u32> {
    let (i, j) = position;
    if i < 0 || j < 0 {
        return None;
    }
    let (i, j) = (i as usize, j as usize);

    if i == init_position.0 && j == init_position.1 {
        return Some(steps + 1);
    }

    if i >= tiles.len() || j >= tiles[0].len() {
        return None;
    }

    let tile = tiles.get(i)?.get(j)?;
    if let Tile::Direction([a, b]) = tile {
        steps += 1;
        if seen[i][j] {
            return Some(steps);
        }
        seen[i][j] = true;

        let position_a = calculate_position(position, a.position());
        let steps_a = find_farthest_steps(position_a, tiles, steps, seen, init_position)?;

        let position_b = calculate_position(position, b.position());
        let steps_b = find_farthest_steps(position_b, tiles, steps, seen, init_position)?;

        return Some(steps_a.max(steps_b));
    }

    None
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
