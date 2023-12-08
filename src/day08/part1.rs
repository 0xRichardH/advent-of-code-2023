use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, Result};
use nom::{
    character::complete::{self, alpha1, multispace0, multispace1, space0},
    multi::many1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult, Parser,
};

type NavigateMap<'a> = HashMap<&'a str, Vec<&'a str>>;

const CURRENT: &str = "AAA";
const DESTINATION: &str = "ZZZ";

pub fn process_data(input: &str) -> Result<usize> {
    let (input, guide) = parse_guide(input).map_err(|e| anyhow!("Failed to parse guide: {}", e))?;
    let (_, map) = parse_navigate_map(input).map_err(|e| anyhow!("Failed to parse map: {}", e))?;

    let mut guide = guide.chars().collect::<VecDeque<_>>();
    let mut steps = 0;

    let mut current = CURRENT;
    while let Some(direction) = guide.pop_front() {
        let current_directions = &map[current];
        current = match direction {
            'L' => current_directions[0],
            'R' => current_directions[1],
            _ => current,
        };
        steps += 1;
        if current == DESTINATION {
            break;
        }
        guide.push_back(direction);
    }

    Ok(steps)
}

fn parse_guide(input: &str) -> IResult<&str, &str> {
    terminated(alpha1, multispace1).parse(input)
}

fn parse_navigate_map(input: &str) -> IResult<&str, NavigateMap> {
    let (input, nodes) = many1(terminated(
        tuple((
            alpha1,
            delimited(space0, complete::char('='), space0),
            parse_directions,
        ))
        .map(|(node, _, nodes)| (node, nodes)),
        multispace0,
    ))
    .parse(input)?;

    let maps: NavigateMap = nodes.into_iter().fold(HashMap::new(), |mut h, (k, v)| {
        h.insert(k, v);
        h
    });

    Ok((input, maps))
}

fn parse_directions(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        complete::char('('),
        separated_pair(
            alpha1,
            delimited(space0, complete::char(','), space0),
            alpha1,
        ),
        complete::char(')'),
    )
    .parse(input)
    .map(|(input, (left, right))| (input, vec![left, right]))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        2
    )]
    #[case(
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        6
    )]
    pub fn test_process_data(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, process_data(input).unwrap());
    }
}
