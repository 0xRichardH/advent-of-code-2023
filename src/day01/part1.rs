use anyhow::Context;

pub fn calibrate(input: &str) -> anyhow::Result<u32> {
    let result = input
        .lines()
        .flat_map(|s| parse_numbers_from_str(s).ok())
        .sum::<u32>();

    Ok(result)
}

fn parse_numbers_from_str(input: &str) -> anyhow::Result<u32> {
    let numbers = input
        .chars()
        .flat_map(|s| s.to_digit(10))
        .collect::<Vec<u32>>();

    let num_str = match numbers.len() {
        1 => {
            let num = numbers[0].to_string();
            format!("{}{}", num, num)
        }
        2.. => {
            let num1 = numbers[0];
            let num2 = numbers[numbers.len() - 1];
            format!("{}{}", num1, num2)
        }
        _ => String::new(),
    };
    let num = num_str
        .parse::<u32>()
        .context("parse number from string failed")?;

    Ok(num)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        142
    )]
    fn it_shoud_calibrate_it_from_str(
        #[case] input: &str,
        #[case] expected: u32,
    ) -> anyhow::Result<()> {
        assert_eq!(expected, calibrate(input)?);
        Ok(())
    }

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn it_should_parse_numbers_from_str(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, parse_numbers_from_str(input).unwrap());
    }
}
