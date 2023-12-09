use num_integer::Integer;
use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};

#[derive(Debug, Clone)]
enum Dir {
    Left,
    Right,
}

struct MapIterator<'a> {
    position: &'a str,
    steps: Box<dyn Iterator<Item = Dir>>,
    maps: &'a HashMap<String, (String, String)>,
}

impl<'a> MapIterator<'a> {
    fn new(start: &'a str, steps: &[Dir], maps: &'a HashMap<String, (String, String)>) -> Self {
        MapIterator {
            position: start,
            // allowed, because this seems to be a known issue
            // https://rust-lang.github.io/rust-clippy/master/index.html#/unnecessary_to_owned
            #[allow(clippy::unnecessary_to_owned)]
            steps: Box::new(steps.to_owned().into_iter().cycle()),
            maps,
        }
    }
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let s = self.steps.next().unwrap();
        match s {
            Dir::Left => self.position = &self.maps.get(self.position).unwrap().0,
            Dir::Right => self.position = &self.maps.get(self.position).unwrap().1,
        }

        Some(self.position)
    }
}

fn parse_input(input: &str) -> (Vec<Dir>, HashMap<String, (String, String)>) {
    let (steps, maps) = input.split_once("\n\n").unwrap();
    let i_parser = parser!(line({
        'L' => Dir::Left,
        'R' => Dir::Right,
    }+));

    let steps = i_parser.parse(steps).unwrap();

    let m_parser = parser!(lines(string(alnum+) " = (" string(alnum+) ", " string(alnum+) ")"));
    let maps = m_parser.parse(maps).unwrap();

    let maps = maps
        .into_iter()
        .map(|(s, l, r)| (s, (l, r)))
        .collect::<HashMap<_, _>>();

    (steps, maps)
}

pub fn part1(input: &str) -> String {
    let (steps, maps) = parse_input(input);

    let (count, _) = MapIterator::new("AAA", &steps, &maps)
        .enumerate()
        .find(|&(_, location)| location == "ZZZ")
        .unwrap();

    (count + 1).to_string()
}

pub fn part2(input: &str) -> String {
    let (steps, maps) = parse_input(input);

    // let mut count: usize = 0;
    let locations = maps
        .keys()
        .filter_map(|k| k.ends_with('A').then_some(k.as_str()))
        .collect::<Vec<_>>();

    // while testing with the actual input I noticed all "paths" have a fixed cycle length
    // so figuring out the first time a path hits its goal is enough to figure out
    // at what point in time all paths are on their respective goal
    let cycles = locations
        .iter()
        .map(|&start| {
            let (count, _) = MapIterator::new(start, &steps, &maps)
                .enumerate()
                .find(|&(_, loc)| loc.ends_with('Z'))
                .unwrap();
            count + 1
        })
        .collect::<Vec<_>>();

    // calculate the lowest common multiple of all cycles
    let lcm = cycles.into_iter().reduce(|acc, el| acc.lcm(&el)).unwrap();

    lcm.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "6");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT2);
        assert_eq!(result, "6");
    }
}
