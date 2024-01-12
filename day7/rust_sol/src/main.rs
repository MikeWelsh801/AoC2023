use std::fs;

use rust_sol::parse_hands;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input.");
    let mut hands = parse_hands(&contents, false);
    hands.sort();

    let answwer_one: u32 = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) as u32 * hand.bid)
        .sum();

    println!("Answer 1: {answwer_one}");
}
