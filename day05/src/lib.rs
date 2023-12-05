use std::{collections::HashMap, ops::Range};

#[derive(Debug)]
struct RangeMap {
    ranges: Vec<(usize, Range<usize>)>,
}

impl RangeMap {
    // map an index to a target
    fn get_target(&self, index: usize) -> usize {
        for (target, range) in self.ranges.iter() {
            if range.contains(&index) {
                return *target + (index - range.start);
            }
        }

        index
    }

    // "reverse mapping" get the index from the target
    fn get_index(&self, target: usize) -> usize {
        for (t, range) in self.ranges.iter() {
            let reversed = *t..*t + range.len();
            if reversed.contains(&target) {
                return range.start + (target - *t);
            }
        }

        target
    }
}

fn parse_ranges(input: &str) -> RangeMap {
    let mut ranges = Vec::new();

    for line in input.lines() {
        let (target, range) = line.split_once(" ").unwrap();
        let (start, length) = range.split_once(" ").unwrap();

        let target = target.parse::<usize>().unwrap();
        let start = start.parse::<usize>().unwrap();
        let length = length.parse::<usize>().unwrap();

        ranges.push((target, start..start + length));
    }

    RangeMap { ranges }
}

fn parse_seeds_and_maps(input: &str) -> (Vec<usize>, HashMap<String, RangeMap>) {
    let mut maps = HashMap::new();
    let mut seeds = Vec::new();

    // split the sections at empty lines
    input.split("\n\n").for_each(|section| {
        if section.starts_with("seeds:") {
            seeds = section
                .strip_prefix("seeds: ")
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
        } else {
            let (key, ranges) = section.split_once(" map:\n").unwrap();
            maps.insert(key.to_string(), parse_ranges(ranges));
        }
    });

    (seeds, maps)
}

pub fn part1(input: &str) -> String {
    let (seeds, maps) = parse_seeds_and_maps(input);

    // map all seeds to their locations
    let locations = seeds.iter().map(|seed| {
        let soil = maps.get("seed-to-soil").unwrap().get_target(*seed);
        let fert = maps.get("soil-to-fertilizer").unwrap().get_target(soil);
        let water = maps.get("fertilizer-to-water").unwrap().get_target(fert);
        let light = maps.get("water-to-light").unwrap().get_target(water);
        let temp = maps.get("light-to-temperature").unwrap().get_target(light);
        let humid = maps
            .get("temperature-to-humidity")
            .unwrap()
            .get_target(temp);

        maps.get("humidity-to-location").unwrap().get_target(humid)
    });

    locations.min().unwrap().to_string()
}

pub fn part2(input: &str) -> String {
    let (seeds, maps) = parse_seeds_and_maps(input);

    // convert the seed numbers to a list of ranges
    let seed_ranges = seeds
        .as_slice()
        .chunks(2)
        .map(|c| c[0]..c[0] + c[1])
        .collect::<Vec<_>>();

    // construct an iterator starting form 0 which uses "reverse mapping"
    // to find the required seed for that location
    let possible_seeds = (0..).map(|location| {
        let humid = maps
            .get("humidity-to-location")
            .unwrap()
            .get_index(location);
        let temp = maps
            .get("temperature-to-humidity")
            .unwrap()
            .get_index(humid);
        let light = maps.get("light-to-temperature").unwrap().get_index(temp);
        let water = maps.get("water-to-light").unwrap().get_index(light);
        let fert = maps.get("fertilizer-to-water").unwrap().get_index(water);
        let soil = maps.get("soil-to-fertilizer").unwrap().get_index(fert);

        maps.get("seed-to-soil").unwrap().get_index(soil)
    });

    // look for the first item inside the iterator that is inside
    // one of the seed ranges
    let (minimal_location, _) = possible_seeds
        .enumerate()
        .find(|&(_, seed)| seed_ranges.iter().any(|r| r.contains(&seed)))
        .unwrap();

    minimal_location.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
seeds: 79 14 55 13

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
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "35");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "46");
    }
}
