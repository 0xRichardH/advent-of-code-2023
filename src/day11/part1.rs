pub fn process_data(input: &str) -> u32 {
    let (points, empty_row_idx, empty_col_idx) = parse_data(input);

    points.iter().enumerate().fold(0, |acc, (idx, p)| {
        let sum_result = points.iter().skip(idx + 1).fold(0, |s, pp| {
            if p == pp {
                return s;
            }

            let (mut a, mut b) = p;
            let (mut c, mut d) = pp;

            a = get_new_row_or_col_idx(&empty_row_idx, a);
            b = get_new_row_or_col_idx(&empty_col_idx, b);
            c = get_new_row_or_col_idx(&empty_row_idx, c);
            d = get_new_row_or_col_idx(&empty_col_idx, d);

            s + (a as i32 - c as i32).abs() + (b as i32 - d as i32).abs()
        });
        acc + sum_result
    }) as u32
}

fn parse_data(input: &str) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let data = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect::<Vec<Vec<bool>>>();

    let mut empty_row_idx = data
        .iter()
        .enumerate()
        .filter(|(_idx, row)| row.iter().all(|b| !b))
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();
    empty_row_idx.sort();

    let mut empty_col_idx = (0..data[0].len())
        .filter(|idx| data.iter().all(|row| !row[*idx]))
        .collect::<Vec<usize>>();
    empty_col_idx.sort();

    let points = data
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, b)| **b)
                .map(|(j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();

    (points, empty_row_idx, empty_col_idx)
}

fn get_new_row_or_col_idx(empty_idxs: &[usize], idx: usize) -> usize {
    let empty_idxs_len = empty_idxs.len();
    let (min, max) = (empty_idxs[0], empty_idxs[empty_idxs_len - 1]);
    if idx < min {
        return idx;
    }

    if idx > max {
        return idx + empty_idxs_len;
    }

    for (i, n) in empty_idxs[0..empty_idxs_len - 1].iter().rev().enumerate() {
        if idx > *n {
            return idx + empty_idxs_len - 1 - i;
        }
    }

    idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(374, process_data(input));
    }
}
