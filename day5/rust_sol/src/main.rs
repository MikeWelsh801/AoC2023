use rust_sol::{
    map_to_location, map_to_seed, parse_map_table, parse_seed_range, parse_seeds, valid_seed,
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

    // search through all possible locations until it finds one that matches a
    // seed (first found is minimum)
    for location in 0..u64::MAX {
        let seed = map_to_seed(location, &table);
        if valid_seed(seed, &seed_range) {
            println!("Answer 2: {location}");
            break;
        }
    }
}
