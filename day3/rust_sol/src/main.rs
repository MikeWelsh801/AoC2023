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

    fn is_here(&self, row: usize, col: usize) -> bool {
        if self.line == row && col >= self.col_start && col <= self.col_end {
            return true;
        }
        false
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

    let answer_two: i32 = get_gear_locations(&grid)
        .into_iter()
        .map(|gear| numbers_touching_gear(&grid, gear.0, gear.1))
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| get_part_number(&parts, numbers[0]) * get_part_number(&parts, numbers[1]))
        .sum();

    println!("Answer 2: {answer_two}");
}

fn get_part_number(parts: &[Part], location: (i32, i32)) -> i32 {
    let (row, col) = (location.0 as usize, location.1 as usize);
    for part in parts.iter() {
        if part.is_here(row, col) {
            return part.number;
        }
    }
    0
}

fn numbers_touching_gear(grid: &[Vec<char>], row: i32, col: i32) -> Vec<(i32, i32)> {
    let mut touching = vec![];
    // look left
    if location_is_digit(&grid, row, col - 1) {
        touching.push((row, col - 1));
    }
    // look right
    if location_is_digit(&grid, row, col + 1) {
        touching.push((row, col + 1))
    }
    // look up
    if location_is_digit(&grid, row - 1, col) {
        touching.push((row - 1, col))
    } else {
        // might be two touching check corners
        // check up left
        if location_is_digit(&grid, row - 1, col - 1) {
            touching.push((row - 1, col - 1))
        }
        if location_is_digit(&grid, row - 1, col + 1) {
            touching.push((row - 1, col + 1))
        }
    }

    // look down
    if location_is_digit(&grid, row + 1, col) {
        touching.push((row + 1, col))
    } else {
        // could be two check corners
        // check down left
        if location_is_digit(&grid, row + 1, col - 1) {
            touching.push((row + 1, col - 1))
        }
        if location_is_digit(&grid, row + 1, col + 1) {
            touching.push((row + 1, col + 1))
        }
    }

    touching
}

fn location_is_digit(grid: &[Vec<char>], row: i32, col: i32) -> bool {
    let row_in_bounds = row >= 0 && (row as usize) < grid.len();
    let col_in_bounds = col >= 0 && (col as usize) < grid.len();

    if row_in_bounds && col_in_bounds && grid[row as usize][col as usize].is_digit(10) {
        return true;
    }
    false
}

fn get_gear_locations(grid: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut gears = vec![];

    (0..grid.len()).for_each(|i| {
        (0..grid[i].len()).for_each(|j| {
            if grid[i][j] == '*' {
                gears.push((i as i32, j as i32));
            }
        });
    });
    gears
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
        let mut end = 0;
        for (j, char) in line.chars().enumerate() {
            if char.is_numeric() && (j == 0 || j > end) {
                let (num, start, e) = parse_number(j, line);
                end = e;
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
