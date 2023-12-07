use crate::utils::*;

//TODO revisit
//TODO check a better approach to this exercise
//This a mess, but works :)

pub fn gear_ratios() {
    let schematic = read_file("day3.txt".to_string());
    let result = get_part_numbers(schematic);

    println!("Day 3 Part 1 result: {}", result);
}
fn get_part_numbers(schematic: String) -> i32 {
    let lines: Vec<String> = split_text_lines(schematic);
    let mut points: i32 = 0;

    for (row, line) in lines.iter().enumerate() {
        let mut current_number: String = String::from("");
        let r = line.chars().enumerate();
        let mut is_valid: bool = false;
        for (point, c) in r {
            let mut current_points: i32 = 0;
            if c.is_numeric() {
                if !is_valid {
                    is_valid = number_is_valid(row, point, line.to_string(), lines.clone());
                }
                current_number.push(c);

                // If is the last number on the line we add here the points
                if is_valid && point == line.len() - 1 {
                    current_points += current_number.parse::<i32>().unwrap();
                }
            } else {
                if is_valid {
                    current_points += current_number.parse::<i32>().unwrap();
                }

                is_valid = false;
                current_number = "".to_string();
            }
            points += current_points;
        }
    }
    points
}

fn number_is_valid(row: usize, point: usize, line: String, lines: Vec<String>) -> bool {
    let left_index = if point.saturating_sub(1) <= 0 {
        0
    } else {
        point - 1
    };
    let right_index = if point + 1 >= line.len() {
        line.len().saturating_sub(1)
    } else {
        point + 1
    };

    let top_index = if row.saturating_sub(1) <= 0 {
        0
    } else {
        row - 1
    };

    let bottom_index = if row + 1 >= lines.len() {
        lines.len().saturating_sub(1)
    } else {
        row + 1
    };

    // left
    let left_side = line.chars().nth(left_index).is_some_and(is_symbol);
    // right
    let right_side = line.chars().nth(right_index).is_some_and(is_symbol);

    // top
    let top_side = lines[top_index].chars().nth(point).is_some_and(is_symbol);
    let top_left = lines[top_index]
        .chars()
        .nth(left_index)
        .is_some_and(is_symbol);
    let top_right = lines[top_index]
        .chars()
        .nth(right_index)
        .is_some_and(is_symbol);

    // bottom
    let bottom_side = lines[bottom_index]
        .chars()
        .nth(point)
        .is_some_and(is_symbol);
    let bottom_left = lines[bottom_index]
        .chars()
        .nth(left_index)
        .is_some_and(is_symbol);
    let bottom_right = lines[bottom_index]
        .chars()
        .nth(right_index)
        .is_some_and(is_symbol);

    left_side
        || right_side
        || top_side
        || top_left
        || top_right
        || bottom_side
        || bottom_left
        || bottom_right
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

#[cfg(test)]
mod tests {
    use crate::day_3::day_3::{gear_ratios, get_part_numbers};
    use crate::utils::read_file;
    #[test]
    fn test_day3_part1() {
        let mock = "..........
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let schematic = read_file("day3.txt".to_string());
        let result = get_part_numbers(schematic);

        let r = get_part_numbers(mock.to_string());
        let f = gear_ratios();
        //546312
        assert_eq!(r, 8722)
        //assert_eq!(result, 546312)
    }
}
