use std::collections::HashMap;

enum Operation {
    Dash(String),
    Equal(String),
}

impl Operation {
    fn label(&self) -> String {
        match self {
            Operation::Dash(label) => label.to_string(),
            Operation::Equal(label) => label.to_string(),
        }
    }
}

pub fn process_data(input: &str) -> u64 {
    let mut label_map = HashMap::<String, usize>::new();
    let mut boxes: Vec<Vec<(String, u8)>> = vec![Vec::<(String, u8)>::new(); 256];

    for character in input.trim_end().split(',') {
        let (operation, focal_length) = parse_character(character);
        let label = operation.label();

        let box_idx = if let Some(box_idx) = label_map.get(&label) {
            *box_idx
        } else {
            let box_idx = hash_str(label.as_str()) as usize;
            label_map.insert(label.clone(), box_idx);
            box_idx
        };

        let contents = boxes.get_mut(box_idx);
        if contents.is_none() {
            continue;
        }
        let contents = contents.unwrap();

        match operation {
            Operation::Dash(_) => {
                if contents.is_empty() {
                    continue;
                }
                if let Some(idx) = contents.iter().position(|v| v.0 == label) {
                    contents.remove(idx);
                }
            }

            Operation::Equal(_) => {
                let content = (label.clone(), focal_length.unwrap_or(0));
                if contents.is_empty() {
                    boxes[box_idx] = vec![content];
                    continue;
                }
                if let Some(idx) = contents.iter().position(|v| v.0 == label) {
                    contents[idx] = content;
                } else {
                    contents.push(content);
                }
            }
        }
    }

    boxes.iter().enumerate().fold(0, |power, (idx, contents)| {
        let box_power = (idx + 1) as u64;
        contents
            .iter()
            .enumerate()
            .fold(0, |c_power, (i, (_, focal_length))| {
                c_power + box_power * (i as u64 + 1) * (*focal_length as u64)
            })
            + power
    })
}

fn parse_character(input: &str) -> (Operation, Option<u8>) {
    if input.ends_with('-') {
        let label = input.trim_end_matches('-');
        return (Operation::Dash(label.to_string()), None);
    }

    let opts = input.split('=').collect::<Vec<&str>>();
    let label = opts[0];
    let focal_length = opts[1];
    (
        Operation::Equal(label.to_string()),
        focal_length.parse().ok(),
    )
}

fn hash_str(input: &str) -> u64 {
    let v = input.chars().fold(0, calculate_current_value);
    custom_hash(v)
}

fn calculate_current_value(current_value: u64, character: char) -> u64 {
    custom_hash(current_value) + character as u64
}

fn custom_hash(current_value: u64) -> u64 {
    current_value * 17 % 256
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process_data() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(145, process_data(input));
    }

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn", 0)]
    #[case("cm", 0)]
    #[case("qp", 1)]
    #[case("pc", 3)]
    #[case("ot", 3)]
    #[case("ab", 3)]
    fn test_hash_str(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(expected, hash_str(input));
    }
}
