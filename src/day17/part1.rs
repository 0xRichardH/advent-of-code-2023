use std::collections::BinaryHeap;

const MAX_STRAIGHT_STEPS: usize = 3;
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

pub fn process_data(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect::<Vec<Vec<u32>>>();

    let starting_point = (0, 0); // top-left
    let ending_point = (grid.len() - 1, grid[0].len() - 1); // bottom-right
                                                            //
    find_shortest_path(&grid, starting_point, ending_point)
}

fn find_shortest_path(
    grid: &[Vec<u32>],
    starting_point: (usize, usize),
    ending_point: (usize, usize),
) -> usize {
    let mut distance = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    let mut pq = BinaryHeap::<State>::new();
    pq.push(State {
        heat: 0,
        position: starting_point,
        direction: RIGHT,
        direction_counter: 1,
    });
    distance[starting_point.0][starting_point.1] = 0;

    let grid_x_len = grid.len() as isize;
    let grid_y_len = grid[0].len() as isize;

    while let Some(State {
        heat,
        position,
        direction_counter,
        direction,
    }) = pq.pop()
    {
        // reach the ending_point
        if position == ending_point {
            return heat;
        }

        // skip the most heaty point
        if heat > distance[position.0][position.1] {
            continue;
        }

        let prev_direction = direction;
        // find the next point by direction
        for (dx, dy) in DIRECTIONS {
            let direction = (dx, dy);
            // can't reverse direction
            match prev_direction {
                UP if direction == DOWN => continue,
                DOWN if direction == UP => continue,
                LEFT if direction == RIGHT => continue,
                RIGHT if direction == LEFT => continue,
                _ => (),
            }

            let mut direction_counter = direction_counter;
            // we can only go straight for MAX_STRAIGHT_STEPS
            if prev_direction == direction {
                if direction_counter > MAX_STRAIGHT_STEPS {
                    continue;
                }
                direction_counter += 1;
            } else {
                direction_counter = 1;
            }

            let (x, y) = (position.0 as isize + dx, position.1 as isize + dy);
            // check the bound
            if x < 0 || x >= grid_x_len || y < 0 || y >= grid_y_len {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            let point_heat = grid[x][y] as usize;
            let next = State {
                heat: heat + point_heat,
                position: (x, y),
                direction,
                direction_counter,
            };

            if next.heat < distance[x][y] {
                // enqueue the least heat point
                pq.push(next);
                distance[next.position.0][next.position.1] = next.heat;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        let input = "2413432311323
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
4322674655533";
        assert_eq!(102, process_data(input));
    }
}
