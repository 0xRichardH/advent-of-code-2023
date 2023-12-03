use std::collections::HashMap;

use anyhow::Result;

pub fn process_data(input: &str) -> Result<u32> {
    let schematic = arryify_schematic(input);
    let mut result: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let mut tmp_num = String::new();
    let mut symobl_position_of_tmp_num = None;
    for (i, lines) in schematic.iter().enumerate() {
        for (j, &elem) in lines.iter().enumerate() {
            if !elem.is_ascii_digit() {
                if let Some(position) = symobl_position_of_tmp_num {
                    let num = tmp_num.parse::<u32>()?;
                    result
                        .entry(position)
                        .and_modify(|e| e.push(num))
                        .or_insert(vec![num]);
                }
                tmp_num.clear();
                symobl_position_of_tmp_num = None;
                continue;
            }

            tmp_num.push(elem);
            if symobl_position_of_tmp_num.is_some() {
                continue;
            }
            symobl_position_of_tmp_num = get_symbol_position_of_part_num((i, j), &schematic);
        }
    }

    let sum = result
        .iter()
        .filter(|(_, ele)| ele.len() > 1)
        .fold(0, |acc, (_, ele)| {
            let num = ele.iter().product::<u32>();
            acc + num
        });

    Ok(sum)
}

fn get_symbol_position_of_part_num(
    (i, j): (usize, usize),
    schematic: &[Vec<char>],
) -> Option<(usize, usize)> {
    // previous
    if i > 0 {
        let previous_line = &schematic[i - 1];
        // up, up left, up right
        if let Some(&c) = previous_line.get(j) {
            if is_allowed_symbol(c) {
                return Some((i - 1, j));
            }
        }

        if j > 0 {
            let c = previous_line[j - 1];
            if is_allowed_symbol(c) {
                return Some((i - 1, j - 1));
            }
        }

        if let Some(&c) = previous_line.get(j + 1) {
            if is_allowed_symbol(c) {
                return Some((i - 1, j + 1));
            }
        }
    }

    // current
    let current_line = &schematic[i];
    //  left
    if j > 0 {
        let c = current_line[j - 1];
        if is_allowed_symbol(c) {
            return Some((i, j - 1));
        }
    }
    //  right
    if let Some(&c) = current_line.get(j + 1) {
        if is_allowed_symbol(c) {
            return Some((i, j + 1));
        }
    }

    // next
    // down, down left, down right
    if let Some(next_line) = schematic.get(i + 1) {
        if let Some(&c) = next_line.get(j) {
            if is_allowed_symbol(c) {
                return Some((i + 1, j));
            }
        }

        if j > 0 {
            let c = next_line[j - 1];
            if is_allowed_symbol(c) {
                return Some((i + 1, j - 1));
            }
        }

        if let Some(&c) = next_line.get(j + 1) {
            if is_allowed_symbol(c) {
                return Some((i + 1, j + 1));
            }
        }
    }

    None
}

fn is_allowed_symbol(c: char) -> bool {
    c == '*'
}

fn arryify_schematic(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_process_data() {
        let schematic = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(467835, process_data(schematic).unwrap());
    }
}
