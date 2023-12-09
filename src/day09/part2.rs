use anyhow::Result;

pub fn process_data(input: &str) -> Result<i32> {
    let result = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .flat_map(|n| n.parse())
                .collect::<Vec<i32>>()
        })
        .fold(0, |acc, n| acc + extrapolate(&n));

    Ok(result)
}

fn extrapolate(histories: &[i32]) -> i32 {
    let mut diffs: Vec<Vec<i32>> = vec![histories.to_vec()];
    calculate_diffs(histories, &mut diffs);

    diffs.iter().rev().flat_map(|nums| nums.last()).sum()
}

fn calculate_diffs(histories: &[i32], diffs: &mut Vec<Vec<i32>>) {
    let diff = histories
        .iter()
        .zip(&histories[1..])
        .map(|(a, b)| b - a)
        .collect::<Vec<i32>>();
    diffs.push(diff.clone());

    if diff.iter().any(|&n| n != 0) {
        calculate_diffs(&diff, diffs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(114, process_data(input).unwrap());
    }
}
