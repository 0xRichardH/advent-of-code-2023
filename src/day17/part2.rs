use std::collections::{BinaryHeap, HashSet};

const MAX_STRAIGHT_STEPS: usize = 10;
const MIN_STRAIGHT_STEPS: usize = 4;
const UP: (isize, isize) = (-1, 0);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);
const RIGHT: (isize, isize) = (0, 1);
const DIRECTIONS: [(isize, isize); 4] = [UP, DOWN, LEFT, RIGHT];

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    heat: usize,
    position: (usize, usize),
    direction: (isize, isize),
    direction_counter: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heat
            .cmp(&self.heat)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Hash)]
struct SeenKey {
    position: (usize, usize),
    direction: (isize, isize),
    direction_counter: usize,
}

impl From<State> for SeenKey {
    fn from(s: State) -> Self {
        Self {
            position: s.position,
            direction: s.direction,
            direction_counter: s.direction_counter,
        }
    }
}

pub fn process_data(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect::<Vec<Vec<u32>>>();

    let starting_point = (0, 0); // top-left
    let ending_point = (grid.len() - 1, grid[0].len() - 1); // bottom-right
    find_shortest_path(&grid, starting_point, ending_point)
}

fn find_shortest_path(
    grid: &[Vec<u32>],
    starting_point: (usize, usize),
    ending_point: (usize, usize),
) -> usize {
    let mut seen = HashSet::<SeenKey>::new();
    let mut pq = BinaryHeap::<State>::new();
    let start_state = State {
        heat: 0,
        position: starting_point,
        direction: RIGHT,
        direction_counter: 1,
    };
    pq.push(start_state);

    let grid_x_len = grid.len() as isize;
    let grid_y_len = grid[0].len() as isize;

    while let Some(state) = pq.pop() {
        // reach the ending_point
        if state.position == ending_point && state.direction_counter >= MIN_STRAIGHT_STEPS {
            return state.heat;
        }

        if !seen.insert(SeenKey::from(state)) {
            continue;
        }

        let prev_direction = state.direction;
        // find the next point by direction
        for direction in DIRECTIONS {
            let (dx, dy) = direction;
            // can't reverse direction
            if prev_direction == (-dx, -dy) {
                continue;
            }

            // we can only go straight for MAX_STRAIGHT_STEPS
            let mut direction_counter = state.direction_counter;

            if prev_direction == direction {
                if direction_counter >= MAX_STRAIGHT_STEPS {
                    continue;
                }
                direction_counter += 1;
            } else {
                if direction_counter < MIN_STRAIGHT_STEPS {
                    continue;
                }
                direction_counter = 1;
            }

            let (x, y) = (
                state.position.0 as isize + dx,
                state.position.1 as isize + dy,
            );
            // check the bound
            if !(0..grid_x_len).contains(&x) || !(0..grid_y_len).contains(&y) {
                continue;
            }

            let (x, y) = (x as usize, y as usize);
            let point_heat = grid[x][y] as usize;
            let next = State {
                heat: state.heat + point_heat,
                position: (x, y),
                direction,
                direction_counter,
            };

            pq.push(next);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        94
    )]
    #[case(
        "111111111111
999999999991
999999999991
999999999991
999999999991",
        71
    )]
    fn test_process_data(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, process_data(input));
    }
}
