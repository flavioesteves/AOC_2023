use rust_aoc::utils::{read_file, split_text_lines};

fn main() {
    let y = read_file("day1.txt".to_string());
    let x = split_text_lines(y);
    println!("Hello, world!");
    println!("Splitted {:?}", x);
}
