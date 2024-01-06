#[derive(Debug)]
pub struct Mapping {
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

#[derive(Debug)]
pub struct MapTable {
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temp: Vec<Mapping>,
    temp_to_humitidy: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl MapTable {
    fn new(
        seed_to_soil: Vec<Mapping>,
        soil_to_fertilizer: Vec<Mapping>,
        fertilizer_to_water: Vec<Mapping>,
        water_to_light: Vec<Mapping>,
        light_to_temp: Vec<Mapping>,
        temp_to_humitidy: Vec<Mapping>,
        humidity_to_location: Vec<Mapping>,
    ) -> Self {
        MapTable {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humitidy,
            humidity_to_location,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Range { start, end }
    }
}

pub fn map_to_location(seed: u64, table: &MapTable) -> u64 {
    let soil = get_dest(seed, &table.seed_to_soil);
    let fertilizer = get_dest(soil, &table.soil_to_fertilizer);
    let water = get_dest(fertilizer, &table.fertilizer_to_water);
    let light = get_dest(water, &table.water_to_light);
    let temp = get_dest(light, &table.light_to_temp);
    let humidity = get_dest(temp, &table.temp_to_humitidy);
    get_dest(humidity, &table.humidity_to_location) // location
}

fn map_to_seed(location: u64, table: &MapTable) -> u64 {
    let humitity = get_src(location, &table.humidity_to_location);
    let temp = get_src(humitity, &table.temp_to_humitidy);
    let light = get_src(temp, &table.light_to_temp);
    let water = get_src(light, &table.water_to_light);
    let fertilizer = get_src(water, &table.fertilizer_to_water);
    let soil = get_src(fertilizer, &table.soil_to_fertilizer);
    get_src(soil, &table.seed_to_soil) // seed
}

fn get_src(dest: u64, source_maps: &[Mapping]) -> u64 {
    let search = source_maps.iter().find(|map| {
        dest >= map.dest_min && dest <= map.dest_min + (map.source_max - map.source_min)
    });

    match search {
        Some(map) => map.source_min + (dest - map.dest_min),
        None => dest,
    }
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

pub fn parse_map_table(sections: &mut std::str::Split<'_, &str>) -> MapTable {
    let seed_to_soil = parse_maps(&sections.next().unwrap());
    let soil_to_fertilizer = parse_maps(&sections.next().unwrap());
    let fertilizer_to_water = parse_maps(&sections.next().unwrap());
    let water_to_light = parse_maps(&sections.next().unwrap());
    let light_to_temp = parse_maps(&sections.next().unwrap());
    let temp_to_humitidy = parse_maps(&sections.next().unwrap());
    let humidity_to_location = parse_maps(&sections.next().unwrap());

    MapTable::new(
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humitidy,
        humidity_to_location,
    )
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

pub fn parse_seed_range(seeds: &str) -> Vec<Range> {
    let starts = seeds.split(" ").skip(1).step_by(2);

    seeds
        .split(" ")
        .skip(2)
        .step_by(2)
        .zip(starts)
        .map(|(len, start)| {
            let start: u64 = start.parse().unwrap();
            let end = start + len.parse::<u64>().unwrap();
            Range::new(start, end)
        })
        .collect()
}

pub fn parse_seeds(seeds: &str) -> Vec<u64> {
    seeds
        .split(" ")
        .skip(1)
        .map(|number| number.parse::<u64>().expect("Couldn't parse seed number."))
        .collect()
}

pub fn find_min_location(seed_range: &[Range], table: &MapTable) -> Option<u64> {
    // search through all possible locations until it finds one that matches a
    // seed (first found is minimum)
    for location in 0..u64::MAX {
        let seed = map_to_seed(location, &table);
        if valid_seed(seed, &seed_range) {
            return Some(location);
        }
    }
    return None;
}

fn valid_seed(seed: u64, seed_range: &[Range]) -> bool {
    if let Some(_) = seed_range
        .iter()
        .find(|range| seed >= range.start && seed < range.end)
    {
        return true;
    }
    false
}
