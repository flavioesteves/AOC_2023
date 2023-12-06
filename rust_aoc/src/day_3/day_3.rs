use crate::utils::*;

pub fn gear_ratios() {
    let _schematic = read_file("day3.txt".to_string());
}

fn get_part_numbers(schematic: String) -> i32 {
    let lines: Vec<String> = split_text_lines(schematic);

    let mut current_number: String = String::from("");

    for (row, line) in lines.iter().enumerate() {
        let r = line.chars().enumerate();

        for (point, c) in r {
            if c.is_numeric() {
                current_number.push(c);
                println!("Point: {}, Value:{}, Row: {}", point, c, row);
                let c_n: i32 = current_number.clone().parse().unwrap();
                println!("Current_number:{:?}", c_n);
            } else {
                current_number = "".to_string();
            }
            let x: Vec<_> = line.chars().collect();

            if x[point + 1] != '.' {
                println!("Point {}", x[point]);
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::day_3::day_3::get_part_numbers;

    #[test]
    fn test_day3_part1() {
        let mock = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let _ = get_part_numbers(mock.to_string());

        assert_eq!(0, 1)
    }
}
