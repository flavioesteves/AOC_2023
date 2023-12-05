use crate::utils::*;

pub fn retrieve_calibration_values() {
    let read_document = read_file("day1.txt".to_string());
    let values = calculate_values(read_document, false);
    let sum_value = sum_values(values);

    println!("Day 1 Part 1 result: {}", sum_value);
}

pub fn retrieve_calibration_values_part2() {
    let read_document = read_file("day1.txt".to_string());
    let values = calculate_values(read_document, true);
    let sum_value = sum_values(values);

    println!("Day 1 Part 2 result: {}", sum_value);
}

fn calculate_values(document: String, is_part2: bool) -> Vec<i32> {
    let data_parsed: Vec<String> = split_text_lines(document);
    let mut calculate_values = Vec::new();

    for mut line in data_parsed {
        let mut only_numerics = Vec::new();

        if is_part2 {
            line = replace_string_values_with_numbers(line);
        }

        for c in line.chars() {
            if c.is_numeric() {
                only_numerics.push(c)
            }
        }

        let first_value = only_numerics[0];
        let last_value = only_numerics[only_numerics.len() - 1];
        let concatenated_string = format!("{}{}", first_value, last_value);

        if let Ok(result) = concatenated_string.parse::<i32>() {
            calculate_values.push(result);
        } else {
            println!("Failed to convert")
        }
    }
    calculate_values
}
fn sum_values(values: Vec<i32>) -> i32 {
    let sum = values.iter().sum();
    sum
}

fn replace_string_values_with_numbers(line: String) -> String {
    let line = line.replace("one", "one1one");
    let line = line.replace("two", "two2two");
    let line = line.replace("three", "three3three");
    let line = line.replace("four", "four4four");
    let line = line.replace("five", "five5five");
    let line = line.replace("six", "six6six");
    let line = line.replace("seven", "seven7seven");
    let line = line.replace("eight", "eight8eight");
    let line = line.replace("nine", "nine9nine");

    line
}

#[cfg(test)]
mod tests {
    use crate::day_1::day_1::{calculate_values, sum_values};

    #[test]
    fn test_sample() {
        let mock = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let values = calculate_values(mock.to_string(), true);
        let sum_value = sum_values(values);

        assert_eq!(281, sum_value);
    }
}
