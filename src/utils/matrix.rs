pub fn transpose<T: Copy + Send + Sync>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return Vec::new();
    }

    let len = v[0].len();
    (0..len)
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

pub fn display_grid<T>(grid: &Vec<Vec<T>>)
where
    T: std::fmt::Display,
{
    println!();
    for row in grid {
        println!("{}", row.iter().map(|n| n.to_string()).collect::<String>());
    }
    println!();
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![], vec![])]
    #[case(vec![vec![1, 2, 3], vec![4, 5, 6]], vec![vec![1, 4], vec![2, 5], vec![3, 6]])]
    #[case(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]])]
    fn test_transpose(#[case] input: Vec<Vec<u32>>, #[case] expected: Vec<Vec<u32>>) {
        assert_eq!(expected, transpose(input));
    }
}
