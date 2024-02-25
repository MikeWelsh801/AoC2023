use std::fs;

use rust_sol::{get_node_count, get_node_count_zend, lcm, parse_map};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't open file.");
    let map = parse_map(&contents);
    let count = get_node_count("AAA", &map);
    println!("Answer 1: {}", count);

    let total_steps: usize = map
        .graph
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|key| get_node_count_zend(key, &map))
        .reduce(|a, b| lcm(a, b))
        .unwrap();
    println!("Answer 2: {}", total_steps);
}
