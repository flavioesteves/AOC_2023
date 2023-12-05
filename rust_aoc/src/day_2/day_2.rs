use crate::utils::*;

pub enum Cubes {
    Red,
    Green,
    Blue,
}
impl Cubes {
    pub fn max(&self) -> i32 {
        match self {
            Cubes::Red => 12,
            Cubes::Green => 13,
            Cubes::Blue => 14,
        }
    }
}

pub fn calculate_games() {
    let read_document = read_file("day2.txt".to_string());
    let calculate_points = analyze_games(read_document);

    println!("Day 2 Part 1 result: {}", calculate_points);
}

fn analyze_games(document: String) -> i32 {
    let data_parsed: Vec<String> = split_text_lines(document);
    let mut total_points = 0;
    let mut game_id = 1;
    for game in data_parsed {
        let game = validate_games(game);
        if game {
            total_points += game_id;
        }

        game_id += 1;
    }
    total_points
}

fn validate_games(game: String) -> bool {
    let split_data: Vec<_> = game.split(":").collect();
    let _game_id = split_data[0];
    let games: Vec<_> = split_data[1].split(";").collect();

    for record in games {
        let game_record = calc(record.to_string());
        if !game_record {
            return false;
        };
    }

    true
}

fn calc(record: String) -> bool {
    let split_records: Vec<_> = record.split(",").collect();
    for record in split_records {
        let rec: Vec<_> = record.split(" ").collect();
        let color = Option::Some(rec[2]);
        let points: i32 = rec[1].parse().expect("Failed to convert to number");

        match color {
            Some("red") => {
                if points > Cubes::Red.max() {
                    return false;
                }
            }
            Some("green") => {
                if points > Cubes::Green.max() {
                    return false;
                }
            }
            Some("blue") => {
                if points > Cubes::Blue.max() {
                    return false;
                }
            }
            _ => println!("Error"),
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::day_2::day_2::analyze_games;

    #[test]
    fn test() {
        let mock = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();

        let a = analyze_games(mock);

        assert_eq!(8, a);
    }
}
