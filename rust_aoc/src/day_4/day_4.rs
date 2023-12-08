use std::collections::HashSet;

use crate::utils::*;

struct Card {
    winning_numbers: HashSet<i32>,
    chosen_numbers: HashSet<i32>,
}

impl Card {
    fn get_points(&self) -> i32 {
        let count = self
            .winning_numbers
            .intersection(&self.chosen_numbers)
            .count();

        if count == 0 {
            return 0;
        }
        let result: i32 = 1 << (count - 1);

        result
    }
}

pub fn run() {
    let cards = read_file("day4.txt".to_string());
    let analyze_cards = analyze_cards(cards.clone());
    let number_scratchcards = win_scratchcards(cards);

    println!("Day 4 part 1 result: {}", analyze_cards);
    println!("Day 4 part 2 result: {}", number_scratchcards);
}

// Part 1
fn analyze_cards(cards: String) -> i32 {
    let mut points: i32 = 0;
    let cards: Vec<String> = split_text_lines(cards);

    for card in cards {
        let card_matched = match_numbers(card);

        let card_points: i32 = card_matched.iter().map(Card::get_points).sum::<i32>();
        points += card_points;
    }

    points
}

// PArt 2
fn win_scratchcards(cards: String) -> i32 {
    let cards: Vec<String> = split_text_lines(cards);
    let mut scratchcards = vec![1usize; cards.len()];

    println!("card len {}", cards.len());
    for (index, card) in cards.iter().enumerate() {
        let scratchcard_numbers = match_numbers(card.to_string());
        let count = scratchcard_numbers
            .iter()
            .map(|card| {
                card.winning_numbers
                    .intersection(&card.chosen_numbers)
                    .count()
            })
            .sum::<usize>();

        println!("count {}", count);
        println!("index {:?}", index + 1..index + 1 + count);

        for i in index + 1..index + 1 + count {
            println!("For loop");
            scratchcards[i] += scratchcards[index];
        }
    }

    let result = scratchcards.iter().sum::<usize>() as i32;
    result
}

fn match_numbers(card: String) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    let (_, card_numbers) = card.split_once(": ").unwrap();
    let (win_num, choose_num) = card_numbers.split_once(" | ").unwrap();

    let winning_numbers = win_num
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<HashSet<_>>();

    let chosen_numbers = choose_num
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<HashSet<_>>();

    cards.push(Card {
        winning_numbers,
        chosen_numbers,
    });
    cards
}

#[cfg(test)]
mod tests {
    use crate::day_4::day_4::{analyze_cards, win_scratchcards};

    const MOCK: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_day4_part1() {
        let test_result = analyze_cards(MOCK.to_string());
        assert_eq!(13, test_result);
    }

    #[test]
    fn test_day4_part2() {
        let test_result = win_scratchcards(MOCK.to_string());

        assert_eq!(30, test_result);
    }
}
