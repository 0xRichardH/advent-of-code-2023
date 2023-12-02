use regex::Regex;

#[derive(Debug)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn process_data(input: &str) -> anyhow::Result<u32> {
    let loaded_game = Game {
        red: 12,
        green: 13,
        blue: 14,
    };
    // Game (\d+):(.+?);(.+?);(.+)
    let re = Regex::new(r"Game (?<num>\d+):(?<rounds>.+)")?;

    let num = input
        .lines()
        .flat_map(|s| {
            let caps = re.captures(s)?;
            let game_num: &str = &caps["num"];
            let rounds_in_str = &caps["rounds"];
            let round_games = parse_game(rounds_in_str).ok()?;

            for game in round_games {
                if game.red > loaded_game.red {
                    return None;
                }
                if game.green > loaded_game.green {
                    return None;
                }
                if game.blue > loaded_game.blue {
                    return None;
                }
            }

            game_num.parse::<u32>().ok()
        })
        .sum();

    Ok(num)
}

fn parse_game(rounds_in_str: &str) -> anyhow::Result<Vec<Game>> {
    let mut games = Vec::new();
    for round in rounds_in_str.split(';') {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for item in round.split(", ") {
            let parts: Vec<&str> = item.split_whitespace().collect();
            let count = parts[0].parse::<u32>()?;
            match parts[1] {
                "red" => {
                    red = count;
                }
                "blue" => {
                    blue = count;
                }
                "green" => {
                    green = count;
                }
                _ => (),
            }
            games.push(Game { red, blue, green });
        }
    }

    Ok(games)
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
        assert_eq!(8, process_data(input).unwrap());
    }
}
