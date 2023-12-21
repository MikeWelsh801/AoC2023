use std::fs;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn new(id: u32, sets: Vec<Set>) -> Self {
        Game { id, sets }
    }
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Set { red, green, blue }
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("couldn't read input");
    let games = parse_input(&content);
    let bag = Set::new(12, 13, 14);

    let answer: u32 = games
        .iter()
        .filter(|game| is_game_possible(game, &bag))
        .map(|game| game.id)
        .sum();

    println!("Answer 1: {answer}");
}

fn is_game_possible(game: &Game, bag: &Set) -> bool {
    for set in game.sets.iter() {
        if set.red > bag.red || set.blue > bag.blue || set.green > bag.green {
            return false;
        }
    }
    true
}

fn parse_input(content: &str) -> Vec<Game> {
    content.lines().map(|line| parse_game(&line)).collect()
}

fn parse_game(line: &str) -> Game {
    let split = line.split(' ');
    let id = split.skip(1).next().unwrap().replace(":", "");
    let game = format!("Game {}: ", id);

    let set_list: Vec<Set> = line
        .replace(&game, "")
        .split(";")
        .map(|set| parse_set(&set.trim()))
        .collect();

    Game::new(id.parse().unwrap(), set_list)
}

fn parse_set(set: &str) -> Set {
    let (mut red, mut green, mut blue) = (0, 0, 0);

    set.split(",")
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|color| {
            let mut split = color.trim().split(" ");
            let (number, color) = (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap(),
            );

            match color {
                "red" => red = number,
                "green" => green = number,
                "blue" => blue = number,
                _ => panic!("unexpected token in colors"),
            };
        });

    Set::new(red, green, blue)
}
