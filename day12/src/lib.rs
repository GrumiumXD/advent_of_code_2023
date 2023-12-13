/// needed some inspiration from reddit for this one
/// https://www.reddit.com/r/adventofcode/comments/18ge41g/2023_day_12_solutions/
use std::{collections::HashMap, iter::once};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Working,
    Broken,
    Unknown,
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => State::Working,
            '#' => State::Broken,
            _ => State::Unknown,
        }
    }
}

#[derive(Debug)]
struct SpringRow {
    springs: Vec<State>,
    groups: Vec<u8>,
}

impl SpringRow {
    fn new(springs: Vec<State>, groups: Vec<u8>) -> Self {
        SpringRow { springs, groups }
    }

    fn unfold(&mut self) {
        self.springs = self
            .springs
            .iter()
            .chain(once(&State::Unknown))
            .cycle()
            .cloned()
            .take(self.springs.len() * 5 + 4)
            .collect::<Vec<_>>();

        self.groups = self
            .groups
            .iter()
            .cycle()
            .cloned()
            .take(self.groups.len() * 5)
            .collect::<Vec<_>>();
    }

    fn get_arrangements(&self) -> u64 {
        let mut cache = HashMap::new();
        self.lookup_arrangements(&mut cache, 0, 0, 0)
    }

    fn lookup_arrangements(
        &self,
        cache: &mut HashMap<(usize, usize, u8), u64>,
        spring: usize, // index of the current spring
        group: usize,  // index of the current group
        length: u8,    // length of the current sequence of broken springs
    ) -> u64 {
        if spring >= self.springs.len() {
            // all groups were already traversed, so this is a valid arrangement
            if group >= self.groups.len() {
                return 1;
            }

            // we are on the last group and the current sequence has the exact length
            // this is also a valid arrangement
            if group == self.groups.len() - 1 && self.groups[group] == length {
                return 1;
            }

            // none of the 2 above cases
            // don't add an additional valid arrangement
            return 0;
        }

        match self.springs[spring] {
            State::Working => {
                // if the sequence has length 0, another working spring will not change this
                // so skip to the next spring
                if length == 0 {
                    return self.lookup_arrangements(cache, spring + 1, group, length);
                }

                // there are no groups left or the current sequence length does not match
                // the current group
                if group >= self.groups.len() || length != self.groups[group] {
                    return 0;
                }

                // a match was found, so proceed with the next group
                self.lookup_arrangements(cache, spring + 1, group + 1, 0)
            }
            State::Broken => {
                // there are no groups left or the current sequence length
                // (+ this additional broken spring) is longer than the current group
                if group >= self.groups.len() || length + 1 > self.groups[group] {
                    return 0;
                }

                // go to the next spring and increase the current length
                self.lookup_arrangements(cache, spring + 1, group, length + 1)
            }
            State::Unknown => {
                // check the cache
                if let Some(result) = cache.get(&(spring, group, length)) {
                    return *result;
                }

                // keep track of the possible arrangements
                let mut arrangements = 0;

                // if the current sequence has length 0 we can treat
                // this unknown as a working spring and keep going
                if length == 0 {
                    arrangements += self.lookup_arrangements(cache, spring + 1, group, length);
                }

                // another way this unknown could be treated as a working spring
                // is by completing the current group
                if group < self.groups.len() && length == self.groups[group] {
                    arrangements += self.lookup_arrangements(cache, spring + 1, group + 1, 0);
                }

                // if the current group is not yet filled, the unknown
                // could be treated as a broken spring
                if group < self.groups.len() && length < self.groups[group] {
                    arrangements += self.lookup_arrangements(cache, spring + 1, group, length + 1);
                }

                // there is no need to look at any "impossible" cases,
                // because they would just add "0" to the arrangements

                // memorize and return the value
                cache.insert((spring, group, length), arrangements);
                arrangements
            }
        }
    }
}

fn parse_springs(input: &str) -> Vec<SpringRow> {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').expect("line should hava space");
            let springs = springs.chars().map(char::into).collect::<Vec<_>>();
            let groups = groups
                .split(',')
                .map(|n| n.parse::<u8>().expect("should be a number"))
                .collect::<Vec<_>>();

            SpringRow::new(springs, groups)
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> String {
    let spring_rows = parse_springs(input);

    spring_rows
        .iter()
        .map(|sr| sr.get_arrangements())
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let mut spring_rows = parse_springs(input);

    for sr in spring_rows.iter_mut() {
        sr.unfold();
    }

    spring_rows
        .iter()
        .map(|sr| sr.get_arrangements())
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "525152");
    }
}
