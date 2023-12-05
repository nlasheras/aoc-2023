use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

#[derive(Clone)]
pub struct MapRange {
    pub destination_start : u64,
    pub source_start : u64,
    pub len : u64
}

impl MapRange {
    fn from(input: &str) -> MapRange {
        let mut parts = input.split_whitespace();
        MapRange { 
            destination_start : parts.next().unwrap().parse::<u64>().unwrap(),
            source_start: parts.next().unwrap().parse::<u64>().unwrap(),
            len : parts.next().unwrap().parse::<u64>().unwrap() 
        }
    }
}

#[derive(Clone)]
pub struct Map {
    pub ranges : Vec<MapRange>
}

impl Map {
    fn from(input: &str) -> Map {
        Map { ranges :  input.lines().skip(1).map(|s| MapRange::from(s)).collect() }
    }

    pub fn convert(&self, input: u64) -> u64 {
        for r in &self.ranges {
            if input >= r.source_start && input < r.source_start + r.len {
                return r.destination_start + input - r.source_start
            }
        }
        input
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> (Vec<u64>, Vec<Map>) {
    let mut lines = input.lines();
    let seed_line = lines.next().unwrap();

    let seeds = seed_line.split(": ").skip(1).next().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let maps = lines.skip(1).join("\n").split("\n\n")
        .map(|s| Map::from(s))
        .collect();

    (seeds, maps)
}

#[aoc(day5, part1)]
pub fn lowest_location(input: &(Vec<u64>, Vec<Map>)) -> u64 {
    input.0.iter().map(|seed| {
        let mut n = *seed;
        for map in &input.1 {
            n = map.convert(n);
        }
        n
    }).min().unwrap()
}

#[aoc(day5, part2)]
pub fn lowest_location_ranges(input: &(Vec<u64>, Vec<Map>)) -> u64 {
    let mut new_ranges = Vec::new();
    for i in 0..input.0.len()/2 {
        let start = input.0[2*i];
        let count = input.0[2*i+1];
        for j in 0..count {
            new_ranges.push(start+j)
        }
    }
    lowest_location(&(new_ranges, input.1.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY05_EXAMPLE: &str = "seeds: 79 14 55 13

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
        let input = parse_input(DAY05_EXAMPLE);
        assert_eq!(lowest_location(&input), 35)
    }

    #[test]
    fn test_day5_part2() {
        let input = parse_input(DAY05_EXAMPLE);
        assert_eq!(lowest_location_ranges(&input), 46)
    }
}
