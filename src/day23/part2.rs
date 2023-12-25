use std::collections::HashSet;

use indexmap::{IndexMap, IndexSet};

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

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
type Graph = IndexMap<Point, IndexMap<Point, usize>>;

pub fn process_data(input: &str) -> usize {
    let (trails_map, start, end) = parse_trails_map(input);
    let graph = restructure_grap(&trails_map, start, end);

    let mut seen = HashSet::<Point>::new();
    walk(&graph, &mut seen, start, end)
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

fn restructure_grap(trails_map: &[Vec<Tile>], start: Point, end: Point) -> Graph {
    // find the neighbors
    let mut points = IndexSet::<Point>::new();
    points.insert(start);
    points.insert(end);
    for (i, row) in trails_map.iter().enumerate() {
        for (j, t) in row.iter().enumerate() {
            if *t == Tile::Forest {
                continue;
            }
            let mut neighbors_counter = 0;
            for d in DIRECTIONS {
                let (di, dj) = d.position();
                let (x, y) = (i as isize + di, j as isize + dj);
                if x < 0
                    || y < 0
                    || x >= trails_map.len() as isize
                    || y >= trails_map[0].len() as isize
                {
                    continue;
                }
                let (ni, nj) = (x as usize, y as usize);
                if trails_map[ni][nj] == Tile::Forest {
                    continue;
                }
                neighbors_counter += 1;
            }
            if neighbors_counter >= 3 {
                points.insert((i, j));
            }
        }
    }

    // restructure the graph
    let mut graph = Graph::from_iter(points.iter().map(|p| (*p, IndexMap::new())));
    for &src_p in points.iter() {
        let mut seen = HashSet::<Point>::new();
        let mut stack = vec![(src_p, 0)];

        while let Some((p, n)) = stack.pop() {
            if seen.contains(&p) {
                continue;
            }
            seen.insert(p);

            if n != 0 && points.contains(&p) {
                graph.entry(src_p).and_modify(|m| {
                    m.insert(p, n);
                });
                continue;
            }

            for d in DIRECTIONS {
                let (di, dj) = d.position();
                let (x, y) = (p.0 as isize + di, p.1 as isize + dj);
                if x < 0
                    || y < 0
                    || x >= trails_map.len() as isize
                    || y >= trails_map[0].len() as isize
                {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                let t = trails_map[x][y];
                if t == Tile::Forest {
                    continue;
                }

                stack.push(((x, y), n + 1));
            }
        }
    }

    graph
}

fn walk(graph: &Graph, seen: &mut HashSet<Point>, start: Point, end: Point) -> usize {
    if start == end {
        return 0;
    }

    let mut max_distance = 0;
    seen.insert(start);

    let Some(point) = graph.get(&start) else {
        return 0;
    };

    for (p, n) in point {
        if seen.contains(p) {
            continue;
        }

        max_distance = max_distance.max(n + walk(graph, seen, *p, end));
    }
    seen.remove(&start);

    max_distance
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
        assert_eq!(154, process_data(input));
    }
}
