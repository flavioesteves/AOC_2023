use crate::utils::{read_file, split_text_lines};
use std::{i64, ops::Range};

#[derive(Debug, Clone, Default)]
struct Almanac {
    seeds: Vec<i64>,
    map: Vec<AlmanacMaps>,
}

#[derive(Debug, Clone, Default)]
struct Mapping {
    range: Range<i64>,
    delta: i64,
}

#[derive(Debug, Clone, Default)]
struct AlmanacMaps {
    maps: Vec<Mapping>,
}

impl AlmanacMaps {
    fn add_mapping(&mut self, source_start: i64, destination_start: i64, range_length: i64) {
        self.maps.push(Mapping {
            range: Range {
                start: destination_start,
                end: destination_start + range_length,
            },
            delta: source_start - destination_start,
        });
        self.maps.sort_by_key(|r| r.range.start);
    }

    fn run_mapping(&self, value: i64) -> i64 {
        for m in &self.maps {
            if m.range.contains(&value) {
                return value + m.delta;
            }
        }
        value
    }
    fn range_contains_seed(&self, seed: i64) -> i64 {
        for map in &self.maps {
            let seed_value = seed - map.delta;
            if map.range.contains(&seed_value) {
                return seed_value;
            }
        }
        seed
    }
}

pub fn run() {
    let load_data = read_file("day5.txt".to_string());
    let data_parsed = split_text_lines(load_data);
    let mapping = map_data(data_parsed);
    let result = calculate_location(mapping.clone());
    let result_2 = calculate_range_location(mapping);

    println!("Day 5 part 1 result: {}", result);

    //Part 2 currently consumes too much memory
    println!("Day 5 part 2 result: {}", result_2);
}

fn map_data(data_parsed: Vec<String>) -> Almanac {
    let s = data_parsed[0].split_once(": ").unwrap().1;
    let seeds: Vec<i64> = s.split(' ').map(|seed| seed.parse().unwrap()).collect();
    let mut almanac = Almanac::default();
    let mut almanac_maps = AlmanacMaps::default();

    for line in data_parsed[2..].iter() {
        if line.contains(":") {
            almanac.map.push(almanac_maps);
            almanac_maps = AlmanacMaps::default();
            continue;
        } else {
            if !line.is_empty() {
                let mapping_values: Vec<i64> =
                    line.split(' ').map(|v| v.parse().unwrap()).collect();
                almanac_maps.add_mapping(mapping_values[0], mapping_values[1], mapping_values[2]);
            }
        }
    }

    if !almanac_maps.maps.is_empty() {
        almanac.map.push(almanac_maps);
    }
    almanac.seeds = seeds;
    almanac
}

fn calculate_location(almanac: Almanac) -> i64 {
    let mut location = i64::MAX;

    for seed in almanac.seeds {
        let mut current = seed;
        for map in &almanac.map {
            current = map.run_mapping(current);
        }
        location = location.min(current);
    }

    location
}

fn calculate_range_location(almanac: Almanac) -> i64 {
    let seeds = range_of_seeds(almanac.seeds);
    let mut location = 1_i64;

    // Loop until find the result
    loop {
        let mut current = location;
        for map in almanac.map.iter().rev() {
            current = map.range_contains_seed(current);
        }
        for seed in &seeds {
            if seed.contains(&current) {
                return location;
            }
        }
        location += 1;
    }
}

fn range_of_seeds(seeds: Vec<i64>) -> Vec<Range<i64>> {
    let seeds_range = seeds
        .chunks(2)
        .map(|vec| Range {
            start: vec[0],
            end: vec[0] + vec[1],
        })
        .collect::<Vec<Range<i64>>>();
    seeds_range
}

// destination range start - source range start - range length
// 50 98 2:q
//

#[cfg(test)]
mod tests {
    use crate::{
        day_5::day_5::{calculate_location, calculate_range_location, map_data},
        utils::split_text_lines,
    };

    const MOCK_DATA: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_day5_part1() {
        let data_parsed = split_text_lines(MOCK_DATA.to_string());
        let map = map_data(data_parsed);
        let lowest_location_number: i64 = calculate_location(map.clone());
        let lowest_range_location: i64 = calculate_range_location(map);

        assert_eq!(lowest_location_number, 35);
        assert_eq!(lowest_range_location, 46);
    }
}
