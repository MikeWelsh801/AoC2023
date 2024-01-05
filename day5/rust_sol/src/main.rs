use std::fs;

#[derive(Debug)]
struct Mapping {
    source_min: u64,
    source_max: u64,
    dest_min: u64,
}

impl Mapping {
    fn new(dest_min: u64, source_min: u64, range: u64) -> Self {
        Mapping {
            source_min,
            source_max: source_min + range,
            dest_min,
        }
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let mut sections = content.split("\r\n\r\n");

    let seeds = parse_seeds(&sections.next().expect("no seed list"));

    let seed_to_soil = parse_maps(&sections.next().unwrap());
    let soil_to_fertilizer = parse_maps(&sections.next().unwrap());
    let fertilizer_to_water = parse_maps(&sections.next().unwrap());
    let water_to_light = parse_maps(&sections.next().unwrap());
    let light_to_temp = parse_maps(&sections.next().unwrap());
    let temp_to_humitidy = parse_maps(&sections.next().unwrap());
    let humidity_to_location = parse_maps(&sections.next().unwrap());

    let answer_one = seeds
        .iter()
        .map(|seed| {
            let soil = get_dest(*seed, &seed_to_soil);
            let fertilizer = get_dest(soil, &soil_to_fertilizer);
            let water = get_dest(fertilizer, &fertilizer_to_water);
            let light = get_dest(water, &water_to_light);
            let temp = get_dest(light, &light_to_temp);
            let humidity = get_dest(temp, &temp_to_humitidy);
            get_dest(humidity, &humidity_to_location)
        })
        .min()
        .unwrap();

    println!("Answer 1: {answer_one}");
}

fn get_dest(seed: u64, source_maps: &[Mapping]) -> u64 {
    let search = source_maps
        .iter()
        .find(|map| seed >= map.source_min && seed <= map.source_max);

    match search {
        Some(map) => map.dest_min + (seed - map.source_min),
        None => seed,
    }
}

fn parse_maps(mapping: &str) -> Vec<Mapping> {
    mapping
        .lines()
        .skip(1)
        .into_iter()
        .map(|line| parse_map(line))
        .collect()
}

fn parse_map(line: &str) -> Mapping {
    let mut split = line.split(" ");
    let dest_min = split
        .next()
        .unwrap()
        .parse::<u64>()
        .expect("couldn't parse dest start.");
    let source_min = split
        .next()
        .unwrap()
        .parse::<u64>()
        .expect("couldn't parse source start.");
    let range = split
        .next()
        .unwrap()
        .parse::<u64>()
        .expect("couldn't parse range.");

    Mapping::new(dest_min, source_min, range)
}

fn parse_seeds(seeds: &str) -> Vec<u64> {
    seeds
        .split(" ")
        .skip(1)
        .map(|number| number.parse::<u64>().expect("Couldn't parse seed number."))
        .collect()
}
