use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct MapNode {
    left: String,
    right: String,
}

#[derive(Debug)]
pub struct Map {
    instructions: Vec<Direction>,
    pub graph: HashMap<String, MapNode>,
}

fn gcd(a: usize, b: usize) -> usize {
    let remainder = a % b;
    if remainder == 0 {
        return b;
    } else {
        return gcd(b, remainder);
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    return a * b / gcd(a, b);
}

pub fn get_node_count(start: &str, map: &Map) -> usize {
    let mut count = 0;
    let mut current = start.to_string();

    while current != "ZZZ" {
        let node = map.graph.get(&current).unwrap();
        let next = match map.instructions[count % map.instructions.len()] {
            Direction::Left => &node.left,
            Direction::Right => &node.right,
        };
        current = next.to_string();
        count += 1;
    }
    return count;
}

pub fn get_node_count_zend(start: &str, map: &Map) -> usize {
    let mut count = 0;
    let mut current = start.to_string();

    while !current.ends_with("Z") {
        let node = map.graph.get(&current).unwrap();
        let next = match map.instructions[count % map.instructions.len()] {
            Direction::Left => &node.left,
            Direction::Right => &node.right,
        };
        current = next.to_string();
        count += 1;
    }
    return count;
}

pub fn parse_map(contents: &str) -> Map {
    let lines: Vec<&str> = contents.lines().collect();
    let directions = lines[0]
        .chars()
        .map(|char| {
            return match char {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => Direction::Left,
            };
        })
        .collect();
    let mut graph = HashMap::new();

    lines.iter().skip(2).for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let left = parts[2].replace("(", "").replace(",", "");
        let right = parts[3].replace(")", "");
        let node = MapNode { left, right };
        let key = parts[0];
        graph.insert(key.to_string(), node);
    });

    Map {
        instructions: directions,
        graph,
    }
}
