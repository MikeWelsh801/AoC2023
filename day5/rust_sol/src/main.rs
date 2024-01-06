use rust_sol::{
    find_min_location, map_to_location, parse_map_table, parse_seed_range, parse_seeds,
};
use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let mut sections = content.split("\r\n\r\n");
    let seed_section = sections.next().expect("no seed list");

    let seeds = parse_seeds(&seed_section);
    let seed_range = parse_seed_range(&seed_section);

    let table = parse_map_table(&mut sections);

    let answer_one = seeds
        .iter()
        .map(|seed| map_to_location(*seed, &table))
        .min()
        .unwrap();

    println!("Answer 1: {answer_one}");

    match find_min_location(&seed_range, &table) {
        Some(answer_two) => println!("Answer 2: {answer_two}"),
        None => println!("No valid location found!"),
    }
}
