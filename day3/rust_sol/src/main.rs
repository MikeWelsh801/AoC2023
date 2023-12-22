use std::{char, fs};

#[derive(Debug)]
struct Part {
    number: i32,
    col_start: usize,
    col_end: usize,
    line: usize,
}

impl Part {
    fn new(number: i32, col_start: usize, col_end: usize, line: usize) -> Self {
        Part {
            number,
            col_start,
            col_end,
            line,
        }
    }

    fn is_valid(&self, grid: &[Vec<char>]) -> bool {
        // check up and down for every digit
        for j in self.col_start..=self.col_end {
            // look up
            if self.line > 0 && check_symbol(grid[self.line - 1][j]) {
                return true;
            }
            // look down
            if self.line < grid.len() - 1 && check_symbol(grid[self.line + 1][j]) {
                return true;
            }
        }
        // look right (only applies to last digit)
        if self.col_end < grid[self.line].len() - 1
            && check_symbol(grid[self.line][self.col_end + 1])
        {
            return true;
        }
        // look left (only applies to first digit)
        if self.col_start > 0 && check_symbol(grid[self.line][self.col_start - 1]) {
            return true;
        }

        /* Check the corners */

        // up right
        if self.col_end < grid[self.line].len() - 1
            && self.line > 0
            && check_symbol(grid[self.line - 1][self.col_end + 1])
        {
            return true;
        }
        // down right
        if self.col_end < grid[self.line].len() - 1
            && self.line < grid.len() - 1
            && check_symbol(grid[self.line + 1][self.col_end + 1])
        {
            return true;
        }
        // up left
        if self.col_start > 0
            && self.line > 0
            && check_symbol(grid[self.line - 1][self.col_start - 1])
        {
            return true;
        }
        // down left
        if self.col_start > 0
            && self.line < grid.len() - 1
            && check_symbol(grid[self.line + 1][self.col_start - 1])
        {
            return true;
        }

        false
    }
}

fn check_symbol(character: char) -> bool {
    match character {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => false,
        _ => true,
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("couldn't read input file");
    let parts = build_number_list(&contents);
    let grid = build_grid(&contents);
    let answer_one: i32 = parts
        .iter()
        .filter(|part| part.is_valid(&grid))
        .map(|part| part.number)
        .sum();

    println!("Answer 1: {answer_one}");

}

fn build_grid(contents: &str) -> Vec<Vec<char>> {
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn build_number_list(contents: &str) -> Vec<Part> {
    let mut parts = vec![];
    for (i, line) in contents.lines().enumerate() {
        let (mut num, mut start, mut end) = (0, 0, 0);
        for (j, char) in line.chars().enumerate() {
            if char.is_numeric() && (j == 0 || j > end) {
                (num, start, end) = parse_number(j, line);
                parts.push(Part::new(num, start, end, i));
            }
        }
    }
    parts
}

fn parse_number(index: usize, line: &str) -> (i32, usize, usize) {
    let (start, mut end) = (index, index);
    while let Ok(_) = line[end..=end].parse::<i32>() {
        end += 1;
        if end == line.len() {
            break;
        }
    }
    let number = line[start..end].parse::<i32>().unwrap();

    (number, start, end - 1)
}
