use std::{fs, u32::MAX};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("couldn't read file");
    let total1: u32 = contents
        .lines()
        .map(|line| 10 * get_first_digit(line, false) + get_last_digit(line, false))
        .sum();

    let total2: u32 = contents
        .lines()
        .map(|line| 10 * get_first_digit(line, true) + get_last_digit(line, true))
        .sum();

    println!("Answer 1: {total1}");
    println!("Answer 2: {total2}");
}

fn get_first_digit(line: &str, include_words: bool) -> u32 {
    for (index, char) in line.chars().enumerate() {
        if include_words {
            if let Some(spelled_num) = find_spelled_digit(&line[..index]) {
                return spelled_num;
            }
        }
        if let Some(num) = char.to_digit(10) {
            return num;
        }
    }
    return MAX;
}

fn get_last_digit(line: &str, include_words: bool) -> u32 {
    for (index, char) in line.chars().rev().enumerate() {
        if include_words {
            if let Some(spelled_num) = find_spelled_digit(&line[line.len() - index..]) {
                return spelled_num;
            }
        }
        if let Some(num) = char.to_digit(10) {
            return num;
        }
    }
    MAX
}

fn find_spelled_digit(substring: &str) -> Option<u32> {
    if substring.contains("zero") {
        return Some(0);
    } else if substring.contains("one") {
        return Some(1);
    } else if substring.contains("two") {
        return Some(2);
    } else if substring.contains("three") {
        return Some(3);
    } else if substring.contains("four") {
        return Some(4);
    } else if substring.contains("five") {
        return Some(5);
    } else if substring.contains("six") {
        return Some(6);
    } else if substring.contains("seven") {
        return Some(7);
    } else if substring.contains("eight") {
        return Some(8);
    } else if substring.contains("nine") {
        return Some(9);
    }
    None
}
