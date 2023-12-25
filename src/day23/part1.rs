use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Paths,             // .
    Forest,            // #
    Slopes(Direction), // ^ > < V
}

type Point = (usize, usize);

pub fn process_data(input: &str) -> usize {
    let (trails_map, start, end) = parse_trails_map(input);
    let mut seen = HashSet::<Point>::new();
    let mut max_distance = 0;
    walk(&trails_map, &mut seen, start, end, 0, &mut max_distance);
    max_distance
}

fn parse_trails_map(input: &str) -> (Vec<Vec<Tile>>, Point, Point) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let len = input.lines().count();
    let trails_map = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.trim()
                .chars()
                .enumerate()
                .map(|(j, c)| {
                    let t = Tile::from(c);

                    if i == 0 && t == Tile::Paths {
                        start = (i, j);
                    }

                    if i == len - 1 && t == Tile::Paths {
                        end = (i, j);
                    }

                    t
                })
                .collect()
        })
        .collect();

    (trails_map, start, end)
}

fn walk(
    trails_map: &[Vec<Tile>],
    seen: &mut HashSet<Point>,
    start: Point,
    end: Point,
    distance: usize,
    max_distance: &mut usize,
) {
    if start == end {
        *max_distance = distance.max(*max_distance);
        return;
    }

    if seen.contains(&start) {
        return;
    }
    seen.insert(start);

    let (i, j) = start;

    let directions = if let Tile::Slopes(direct) = trails_map[i][j] {
        vec![direct]
    } else {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    };

    for d in directions {
        let (di, dj) = d.position();
        let (x, y) = (i as isize + di, j as isize + dj);
        if x < 0 || y < 0 || x >= trails_map.len() as isize || y >= trails_map[0].len() as isize {
            continue;
        }
        let (x, y) = (x as usize, y as usize);
        let t = trails_map[x][y];
        if t == Tile::Forest {
            continue;
        }
        walk(trails_map, seen, (x, y), end, distance + 1, max_distance);
    }

    seen.remove(&start);
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Forest,
            '^' => Tile::Slopes(Direction::Up),
            '>' => Tile::Slopes(Direction::Right),
            'v' => Tile::Slopes(Direction::Down),
            '<' => Tile::Slopes(Direction::Left),
            _ => Tile::Paths,
        }
    }
}

impl Direction {
    fn position(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(94, process_data(input));
    }
}
