use std::collections::HashMap;

use anyhow::{anyhow, Result};
use nom::{
    character::complete::{self, alpha1, alphanumeric1, multispace0, multispace1, space0},
    multi::many1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult, Parser,
};
use num::Integer;

type NavigateMap<'a> = HashMap<&'a str, Vec<&'a str>>;

pub fn process_data(input: &str) -> Result<usize> {
    let (input, guide) = parse_guide(input).map_err(|e| anyhow!("Failed to parse guide: {}", e))?;
    let (_, (map, starts)) =
        parse_navigate_map(input).map_err(|e| anyhow!("Failed to parse map: {}", e))?;

    let mut steps_arr = Vec::with_capacity(starts.len());
    starts.into_iter().for_each(|start| {
        let mut current = start;
        let mut counter = 0;
        for direction in guide.chars().cycle() {
            let current_directions = &map[current];
            current = match direction {
                'L' => current_directions[0],
                'R' => current_directions[1],
                _ => current,
            };
            counter += 1;
            if is_end(current) {
                break;
            }
        }
        steps_arr.push(counter);
    });

    let steps = steps_arr.iter().fold(1, |acc, n| acc.lcm(n));

    Ok(steps)
}

fn parse_guide(input: &str) -> IResult<&str, &str> {
    terminated(alpha1, multispace1).parse(input)
}

fn parse_navigate_map(input: &str) -> IResult<&str, (NavigateMap, Vec<&str>)> {
    let (input, nodes) = many1(terminated(
        tuple((
            alphanumeric1,
            delimited(space0, complete::char('='), space0),
            parse_directions,
        ))
        .map(|(node, _, nodes)| (node, nodes)),
        multispace0,
    ))
    .parse(input)?;

    let mut starts = Vec::new();
    let maps: NavigateMap = nodes.into_iter().fold(HashMap::new(), |mut h, (k, v)| {
        if is_start(k) {
            starts.push(k);
        }
        h.insert(k, v);
        h
    });

    Ok((input, (maps, starts)))
}

fn parse_directions(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        complete::char('('),
        separated_pair(
            alphanumeric1,
            delimited(space0, complete::char(','), space0),
            alphanumeric1,
        ),
        complete::char(')'),
    )
    .parse(input)
    .map(|(input, (left, right))| (input, vec![left, right]))
}

fn is_start(node: &str) -> bool {
    node.to_string().ends_with('A')
}

fn is_end(node: &str) -> bool {
    node.to_string().ends_with('Z')
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        6
    )]
    pub fn test_process_data(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, process_data(input).unwrap());
    }
}
