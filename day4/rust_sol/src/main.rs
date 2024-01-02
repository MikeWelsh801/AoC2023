use std::fs;

#[derive(Debug)]
struct Table {
    cards: Vec<Card>,
}

impl Table {
    fn new() -> Self {
        Table { cards: vec![] }
    }

    fn add_card(&mut self, card: &str) {
        self.cards.push(Card::new(card));
    }

    /// driver for recursive copy function
    fn get_copies(&self, start_index: usize) -> usize {
        // always count at least this card
        if self.cards[start_index - 1].matches == 0 {
            return 1;
        }
        1 + self.copies_recursive(start_index)
    }

    fn copies_recursive(&self, index: usize) -> usize {
        // base case: out of bounds or no matches
        if index > self.cards.len() || self.cards[index - 1].matches == 0 {
            return 0;
        }

        // add this cards matches (1 copy per match) with all of the other matched
        // coppies
        let matches = self.cards[index - 1].matches;
        let copies: usize = (1..=matches)
            .map(|offset| self.copies_recursive(index + offset))
            .sum();

        matches + copies
    }
}

#[derive(Debug)]
struct Card {
    card_num: usize,
    matches: usize,
}

impl Card {
    fn new(card_info: &str) -> Self {
        let matches =
            get_number_matches(&get_winning_nums(&card_info), &get_my_numbers(&card_info));
        let card_num = card_info
            .split(" ")
            .skip_while(|ele| !ele.contains(':'))
            .next()
            .unwrap()
            .replace(":", "")
            .parse::<usize>()
            .unwrap();

        Card { card_num, matches }
    }
}

fn main() {
    // parse input and build table
    let mut table = Table::new();

    fs::read_to_string("input.txt")
        .expect("Couldn't read file.")
        .lines()
        .for_each(|line| table.add_card(&line));

    let answer_one: usize = table
        .cards
        .iter()
        .map(|card| match card.matches {
            0 => card.matches,
            _ => 1 << (card.matches - 1),
        })
        .sum();

    let answer_two: usize = table
        .cards
        .iter()
        .map(|card| table.get_copies(card.card_num))
        .sum();

    println!("Answer 1: {answer_one}");
    println!("Answer 2: {answer_two}");
}

fn get_number_matches(winning_numbers: &[i32], my_numbers: &[i32]) -> usize {
    my_numbers
        .iter()
        .filter(|num| winning_numbers.contains(num))
        .count()
}

fn get_my_numbers(line: &str) -> Vec<i32> {
    line.split(" ")
        .skip_while(|ele| *ele != "|")
        .skip(1)
        .filter(|ele| !ele.is_empty())
        .map(|num| num.parse().unwrap())
        .collect()
}

fn get_winning_nums(line: &str) -> Vec<i32> {
    line.split(" ")
        .skip_while(|ele| !ele.contains(':'))
        .skip(1)
        .take_while(|ele| *ele != "|")
        .filter(|ele| !ele.is_empty())
        .map(|num| num.parse().unwrap())
        .collect()
}
