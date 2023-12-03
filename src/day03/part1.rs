use anyhow::Result;

pub fn process_data(input: &str) -> Result<u32> {
    let schematic = arryify_schematic(input);
    let mut sum = 0;

    let mut tmp_num = String::new();
    let mut is_tmp_num_part_num = false;
    for (i, lines) in schematic.iter().enumerate() {
        for (j, &elem) in lines.iter().enumerate() {
            if !elem.is_ascii_digit() {
                if is_tmp_num_part_num {
                    let num = tmp_num.parse::<u32>()?;
                    sum += num;
                }
                tmp_num.clear();
                is_tmp_num_part_num = false;
                continue;
            }

            tmp_num.push(elem);
            if is_tmp_num_part_num {
                continue;
            }
            is_tmp_num_part_num = is_part_num((i, j), &schematic);
        }
    }

    Ok(sum)
}

fn is_part_num((i, j): (usize, usize), schematic: &[Vec<char>]) -> bool {
    // previous
    if i > 0 {
        let previous_line = &schematic[i - 1];
        // up, up left, up right
        if let Some(&c) = previous_line.get(j) {
            if is_allowed_symbol(c) {
                return true;
            }
        }

        if j > 0 {
            let c = previous_line[j - 1];
            if is_allowed_symbol(c) {
                return true;
            }
        }

        if let Some(&c) = previous_line.get(j + 1) {
            if is_allowed_symbol(c) {
                return true;
            }
        }
    }

    // current
    let current_line = &schematic[i];
    //  left
    if j > 0 {
        let c = current_line[j - 1];
        if is_allowed_symbol(c) {
            return true;
        }
    }
    //  right
    if let Some(&c) = current_line.get(j + 1) {
        if is_allowed_symbol(c) {
            return true;
        }
    }

    // next
    // down, down left, down right
    if let Some(next_line) = schematic.get(i + 1) {
        if let Some(&c) = next_line.get(j) {
            if is_allowed_symbol(c) {
                return true;
            }
        }

        if j > 0 {
            let c = next_line[j - 1];
            if is_allowed_symbol(c) {
                return true;
            }
        }

        if let Some(&c) = next_line.get(j + 1) {
            if is_allowed_symbol(c) {
                return true;
            }
        }
    }

    false
}

fn is_allowed_symbol(c: char) -> bool {
    c.is_ascii_punctuation() && c != '.'
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
        assert_eq!(4361, process_data(schematic).unwrap());
    }
}
