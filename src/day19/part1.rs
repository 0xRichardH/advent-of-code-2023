use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{is_a, tag, take_until},
    character::complete::{self, alpha1},
    sequence::tuple,
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Condition {
    GreatThan(usize),
    LessThan(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Status {
    Accept,
    Reject,
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Rule {
    category: Option<Category>,
    condition: Option<Condition>,
    workflow: Option<String>,
    status: Option<Status>,
}

type Workflow = HashMap<String, Vec<Rule>>;
type Rating = HashMap<Category, usize>;
type Ratings = Vec<Rating>;

pub fn process_data(input: &str) -> usize {
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let workflows_input = input[0];
    let mut workflows = Workflow::new();
    workflows_input
        .lines()
        .map(parse_workflow_from)
        .for_each(|result| {
            if let Ok((key, rules)) = result {
                workflows.insert(key, rules);
            } else {
                eprintln!("failed to parse workflow: {}", result.unwrap_err());
            }
        });
    let ratings = parse_ratings_from(input[1]);

    let mut total = 0;

    for rating in ratings {
        let status = execute_rules("in", &workflows, &rating);
        if status.is_err() {
            eprintln!("failed to execute rules: {}", status.unwrap_err());
            continue;
        }

        if Status::Accept == status.unwrap() {
            rating.iter().for_each(|(_, v)| {
                total += v;
            })
        }
    }

    total
}

fn execute_rules(
    start_workflow_key: &str,
    workflows: &Workflow,
    rating: &Rating,
) -> Result<Status> {
    let mut q = VecDeque::new();
    q.push_back(start_workflow_key);

    while let Some(key) = q.pop_front() {
        let rules = workflows.get(key);
        if rules.is_none() {
            continue;
        }

        for rule in rules.unwrap() {
            if let Some(category) = rule.category {
                let value = rating[&category];
                if let Some(condition) = rule.condition {
                    if !condition.is_met(value) {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            if let Some(workflow) = &rule.workflow {
                q.push_back(workflow);
                break;
            }

            if let Some(status) = rule.status {
                return Ok(status);
            }
        }
    }

    Err(anyhow!("failed to execute rules"))
}

fn parse_workflow_from(input: &str) -> Result<(String, Vec<Rule>)> {
    let (_, (key, rules)) =
        parse_workflow(input).map_err(|e| anyhow!("failed to parse workflow: {}", e))?;
    let rules = rules
        .split(',')
        .map(Rule::try_from)
        .collect::<Result<Vec<Rule>>>()?;

    Ok((key.to_string(), rules))
}

fn parse_workflow(input: &str) -> IResult<&str, (&str, &str)> {
    let (rules, key) = take_until("{").parse(input.trim_end_matches('}'))?;
    let rules = rules.trim_start_matches('{');
    Ok(("", (key, rules)))
}

fn parse_ratings_from(ratings_input: &str) -> Ratings {
    ratings_input
        .lines()
        .map(|l| {
            l.trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .flat_map(|s| {
                    let parts = s.split('=').collect::<Vec<&str>>();
                    let key = Category::try_from(parts[0]);
                    let value = parts[1].parse::<usize>();

                    if key.is_err() {
                        eprintln!("Failed to parse {}", s);
                        return None;
                    }

                    if let Ok(value) = value {
                        Some((key.unwrap(), value))
                    } else {
                        eprintln!("Failed to parse {}", s);
                        None
                    }
                })
                .collect()
        })
        .collect()
}

impl TryFrom<&str> for Category {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input {
            "x" => Ok(Category::X),
            "m" => Ok(Category::M),
            "a" => Ok(Category::A),
            "s" => Ok(Category::S),
            _ => Err(anyhow!("Invalid condition")),
        }
    }
}

impl TryFrom<(&str, usize)> for Condition {
    type Error = anyhow::Error;

    fn try_from((condition, condition_value): (&str, usize)) -> Result<Self, Self::Error> {
        match condition {
            ">" => Ok(Condition::GreatThan(condition_value)),
            "<" => Ok(Condition::LessThan(condition_value)),
            _ => Err(anyhow!("Invalid condition")),
        }
    }
}

impl Condition {
    fn is_met(&self, value: usize) -> bool {
        match self {
            Condition::GreatThan(v) => value > *v,
            Condition::LessThan(v) => value < *v,
        }
    }
}

impl TryFrom<&str> for Rule {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if input == "A" {
            return Ok(Rule {
                status: Some(Status::Accept),
                ..Default::default()
            });
        }

        if input == "R" {
            return Ok(Rule {
                status: Some(Status::Reject),
                ..Default::default()
            });
        }

        if !input.contains(':') {
            return Ok(Rule {
                workflow: Some(input.to_string()),
                ..Default::default()
            });
        }

        parse_rule_from(input)
    }
}

fn parse_rule_from(input: &str) -> Result<Rule> {
    let (_, (category, condition, condition_value, action)) =
        parse_rule(input).map_err(|e| anyhow!("failed to pare rule: {}", e))?;

    let mut workflow = None;
    let mut status = None;
    match action {
        "R" => status = Some(Status::Reject),
        "A" => status = Some(Status::Accept),
        _ => {
            workflow = Some(action.to_string());
        }
    }

    let rule = Rule {
        category: category.try_into().ok(),
        condition: (condition, condition_value).try_into().ok(),
        workflow,
        status,
    };
    Ok(rule)
}

fn parse_rule(input: &str) -> IResult<&str, (&str, &str, usize, &str)> {
    let (input, (category, condition, condition_value, _, action)) =
        tuple((is_a("xmas"), is_a("><"), complete::u64, tag(":"), alpha1)).parse(input)?;

    Ok((
        input,
        (category, condition, condition_value as usize, action),
    ))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process_input() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(19114, process_data(input));
    }

    #[rstest]
    #[case("a<2006:qkq", Rule { category: Some(Category::A), condition: Some(Condition::LessThan(2006)), workflow: Some("qkq".to_string()), status: None })]
    #[case("m>2090:A", Rule { category: Some(Category::M), condition: Some(Condition::GreatThan(2090)), workflow: None, status: Some(Status::Accept) })]
    #[case("rfg", Rule { category: None, condition: None, workflow: Some("rfg".to_string()), status: None })]
    #[case("A", Rule { category: None, condition: None, workflow: None, status: Some(Status::Accept) })]
    #[case("R", Rule { category: None, condition: None, workflow: None, status: Some(Status::Reject) })]
    fn it_passes_rule(#[case] input: &str, #[case] expected: Rule) {
        assert_eq!(expected, input.try_into().unwrap());
    }
}
