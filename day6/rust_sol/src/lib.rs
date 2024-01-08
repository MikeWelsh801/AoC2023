use std::str::Lines;

#[derive(Debug)]
pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn new(time: u64, distance: u64) -> Self {
        Race { time, distance }
    }

    pub fn ways_to_win(&self) -> u64 {
        (0..=self.time)
            .filter(|hold_time| {
                let time_to_run = self.time - hold_time;
                hold_time * time_to_run > self.distance // hold time is also speed
            })
            .count() as u64
    }
}

pub fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = parse_line(&mut lines);
    let distance = parse_line(&mut lines);

    times
        .iter()
        .zip(distance)
        .map(|(time, distance)| Race::new(*time, distance))
        .collect()
}

fn parse_line(lines: &mut Lines<'_>) -> Vec<u64> {
    lines
        .next()
        .expect("no next line in input")
        .split_whitespace()
        .skip(1)
        .map(|ele| ele.parse().expect("encountered non-number in input"))
        .collect()
}

pub fn parse_one_race(content: &str) -> Race {
    let mut lines = content.lines();
    let time = parse_line_single(&mut lines);
    let distance = parse_line_single(&mut lines);

    Race::new(
        time.parse().expect("Couldn't parse value."),
        distance.parse().expect("Coundn't parse value."),
    )
}

fn parse_line_single(lines: &mut Lines<'_>) -> String {
    lines
        .next()
        .expect("no next line in input")
        .split_whitespace()
        .skip(1)
        .collect()
}
