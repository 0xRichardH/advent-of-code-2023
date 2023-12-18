use anyhow::{anyhow, Context};
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self, hex_digit1, space1},
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};

const UP: (isize, isize) = (-1, 0);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);
const RIGHT: (isize, isize) = (0, 1);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug)]
struct Dig {
    direction: Direction,
    meters: usize,
}

type Position = (isize, isize);

pub fn process_data(input: &str) -> usize {
    let dig_plan = input
        .lines()
        .flat_map(|l| Dig::try_from(l).ok())
        .collect::<Vec<_>>();

    let (points, boundary_points_count) = get_points(&dig_plan);

    let area = calculate_area(&points);
    let interior = calculate_interior(area, boundary_points_count);

    boundary_points_count + interior
}

fn get_points(dig_plan: &[Dig]) -> (Vec<Position>, usize) {
    let mut boundary_points_count = 0;
    let mut current_point = (0, 0);
    let mut points = Vec::new();

    for d in dig_plan {
        boundary_points_count += d.meters;
        let next_point = d.next_point(current_point);
        if let Some(next_point) = next_point {
            points.push(next_point);
            current_point = next_point;
        }
    }

    (points, boundary_points_count)
}

// Shoelace formula
// https://en.wikipedia.org/wiki/Shoelace_formula
fn calculate_area(points: &[Position]) -> usize {
    let mut xy = 0;
    let mut yx = 0;
    for pp in points.windows(2) {
        let (x1, y1) = pp[0];
        let (x2, y2) = pp[1];
        xy += x1 * y2;
        yx += y1 * x2;
    }

    (xy - yx).unsigned_abs() / 2
}

// Pick's theorem
// https://en.wikipedia.org/wiki/Pick%27s_theorem
fn calculate_interior(area: usize, boundary_points_count: usize) -> usize {
    area - (boundary_points_count / 2) + 1
}

impl TryFrom<&str> for Dig {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let (_, hex_digit) =
            parse_single_plan(input).map_err(|e| anyhow!("failed to parse plan: {}", e))?;

        let hex_digit_chars = hex_digit.chars().collect::<Vec<_>>();

        let direction_result = hex_digit_chars.last();
        let meters_str = hex_digit_chars[0..hex_digit_chars.len() - 1]
            .iter()
            .collect::<String>();
        let meters = usize::from_str_radix(&meters_str, 16).context("parser meters failed")?;

        if direction_result.is_none() {
            return Err(anyhow!("failed to parse direction"));
        }

        let dig = Dig {
            direction: Direction::from(direction_result.unwrap()),
            meters,
        };

        Ok(dig)
    }
}

impl Dig {
    fn next_point(&self, current: Position) -> Option<Position> {
        let (x, y) = current;
        let (dx, dy) = self.direction.position();
        let (i, j) = (x + dx * self.meters as isize, y + dy * self.meters as isize);

        Some((i, j))
    }
}

impl From<&char> for Direction {
    fn from(dir: &char) -> Self {
        match dir {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => Direction::None,
        }
    }
}

impl Direction {
    fn position(&self) -> Position {
        match self {
            Direction::Up => UP,
            Direction::Down => DOWN,
            Direction::Left => LEFT,
            Direction::Right => RIGHT,
            Direction::None => (0, 0),
        }
    }
}

fn parse_single_plan(input: &str) -> IResult<&str, &str> {
    let (input, (_, hex_digit)) = preceded(
        tuple((is_a("UDLR"), space1, complete::u32, space1)),
        delimited(tag("("), tuple((tag("#"), hex_digit1)), tag(")")),
    )
    .parse(input)?;

    Ok((input, hex_digit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(952408144115, process_data(input));
    }
}
