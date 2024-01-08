use rust_sol::{parse_input, parse_one_race, ways_to_win};
use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let races = parse_input(&content);
    let combo_race = parse_one_race(&content);

    let answer_one: u64 = races.iter().map(|race| ways_to_win(race)).product();
    let answer_two = ways_to_win(&combo_race);

    println!("Answer 1: {answer_one}");
    println!("Answer 2: {answer_two}");
}
