use crate::utils::*;

pub struct Part1 {}

impl Part1 {
    pub fn retrieve_calibration_values() {
        let read_document = read_file("day1.txt".to_string());
        let values = Self::calculate_values(read_document);
        let sum_value = sum_values(values);

        println!("Day 1 Part 1 result: {}", sum_value);
    }

    fn calculate_values(document: String) -> Vec<i32> {
        let data_parsed: Vec<String> = split_text_lines(document);
        let mut calculate_values = Vec::new();

        for line in data_parsed {
            let mut only_numerics = Vec::new();
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
}

fn sum_values(values: Vec<i32>) -> i32 {
    let sum = values.iter().sum();
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        let test_example = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("hello", test_example);
    }
}
