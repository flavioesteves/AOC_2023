use crate::utils::*;

pub fn calculate_power_sets() {
    let read_document = read_file("day2.txt".to_string());
    let n_games = analyze_games(read_document);

    println!("Day 2 Part 2 result: {}", n_games);
}

fn analyze_games(document: String) -> i32 {
    let games: Vec<String> = split_text_lines(document);
    let mut total_power: i32 = 0;
    for game in games {
        total_power += calculate_fewest_cubes_to_play(game);
    }
    total_power
}

fn calculate_fewest_cubes_to_play(game: String) -> i32 {
    let parsed_data: Vec<_> = game.split(":").collect();
    let _game_id = parsed_data[0];
    let rounds: Vec<_> = parsed_data[1].split(";").collect();
    let mut red_cubes: i32 = 0;
    let mut green_cubes: i32 = 0;
    let mut blue_cubes: i32 = 0;

    for round in rounds {
        let round_power = calculate_total_power(round.to_string());
        if round_power.red_cubes > red_cubes {
            red_cubes = round_power.red_cubes;
        }
        if round_power.green_cubes > green_cubes {
            green_cubes = round_power.green_cubes;
        }
        if round_power.blue_cubes > blue_cubes {
            blue_cubes = round_power.blue_cubes;
        }
    }
    let total_power: i32 = red_cubes * green_cubes * blue_cubes;
    total_power
}

struct RoundPower {
    red_cubes: i32,
    green_cubes: i32,
    blue_cubes: i32,
}

impl RoundPower {
    fn new(red_cubes: i32, green_cubes: i32, blue_cubes: i32) -> Self {
        Self {
            red_cubes,
            green_cubes,
            blue_cubes,
        }
    }
}

fn calculate_total_power(round: String) -> RoundPower {
    let mut red_cubes: i32 = 0;
    let mut green_cubes: i32 = 0;
    let mut blue_cubes: i32 = 0;

    let cubes: Vec<_> = round.split(",").collect();

    for cube in cubes {
        let parsed_data: Vec<_> = cube.split(" ").collect();
        let cube_color = Option::Some(parsed_data[2]);
        let n_cubes: i32 = parsed_data[1].parse().expect("Failed to convert to i32");

        match cube_color {
            Some("red") => {
                if n_cubes > red_cubes {
                    red_cubes = n_cubes;
                }
            }
            Some("green") => {
                if n_cubes > green_cubes {
                    green_cubes = n_cubes
                }
            }
            Some("blue") => {
                if n_cubes > blue_cubes {
                    blue_cubes = n_cubes;
                }
            }
            _ => println!("Error on calculate_total_power"),
        }
    }

    RoundPower::new(red_cubes, green_cubes, blue_cubes)
}

#[cfg(test)]
mod tests {
    use crate::day_2::day_2_p2::analyze_games;

    #[test]
    fn test() {
        let mock = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();

        let a = analyze_games(mock);

        assert_eq!(2286, a);
    }
}
