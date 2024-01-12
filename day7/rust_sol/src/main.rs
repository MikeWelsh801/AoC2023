use std::fs;

use rust_sol::{parse_hands, get_total_winings};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input.");

    // wild cards not considered for part 1
    let mut hands = parse_hands(&contents, false);
    let mut wild_hands = parse_hands(&contents, true);

    // sort the two lists before calculating winnings
    hands.sort();
    wild_hands.sort();
    
    let answer_one = get_total_winings(&hands);
    let answer_two = get_total_winings(&wild_hands);

    println!("Answer 1: {answer_one}");
    println!("Answer 2: {answer_two}");
}
