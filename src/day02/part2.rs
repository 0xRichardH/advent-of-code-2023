use regex::Regex;

#[derive(Debug)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn process_data(input: &str) -> anyhow::Result<u32> {
    // Game (\d+):(.+?);(.+?);(.+)
    let re = Regex::new(r"Game \d+:(?<rounds>.+)")?;

    let num = input
        .lines()
        .flat_map(|s| {
            let caps = re.captures(s)?;
            let rounds_in_str = &caps["rounds"];
            let game = parse_game(rounds_in_str).ok()?;
            let power = game.red * game.green * game.blue;
            Some(power)
        })
        .sum();

    Ok(num)
}

fn parse_game(rounds_in_str: &str) -> anyhow::Result<Game> {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for round in rounds_in_str.split(';') {
        for item in round.split(", ") {
            let parts: Vec<&str> = item.split_whitespace().collect();
            let count = parts[0].parse::<u32>()?;
            match parts[1] {
                "red" => {
                    red = red.max(count);
                }
                "blue" => blue = blue.max(count),
                "green" => {
                    green = green.max(count);
                }
                _ => (),
            }
        }
    }

    Ok(Game { red, green, blue })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_process_the_data() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, process_data(input).unwrap());
    }
}
