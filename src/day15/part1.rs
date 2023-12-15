pub fn process_data(input: &str) -> u64 {
    input.trim_end().split(',').map(hash_str).sum()
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
        assert_eq!(1320, process_data(input));
    }

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("pc-", 48)]
    fn test_hash_str(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(expected, hash_str(input));
    }

    #[test]
    fn test_calculate_current_value() {
        let mut v = calculate_current_value(0, 'H');
        assert_eq!(72, v);
        v = calculate_current_value(v, 'A');
        assert_eq!(265, v);
        v = calculate_current_value(v, 'S');
        assert_eq!(236, v);
        v = calculate_current_value(v, 'H');
        assert_eq!(244, v);
        v = custom_hash(v);
        assert_eq!(52, v);
    }
}
