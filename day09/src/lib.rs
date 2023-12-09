struct HistorySteps {
    values: Vec<i64>,
    init: bool,
    done: bool,
    forward: bool,
}

impl HistorySteps {
    fn new(values: Vec<i64>, forward: bool) -> Self {
        HistorySteps {
            values,
            init: true,
            done: false,
            forward,
        }
    }
}

impl Iterator for HistorySteps {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.init {
            self.init = false;
            if self.forward {
                return Some(*self.values.last().unwrap());
            } else {
                return Some(*self.values.first().unwrap());
            }
        }
        if self.done {
            return None;
        }

        self.values = self
            .values
            .iter()
            .zip(self.values.iter().skip(1))
            .map(|(&a, &b)| b - a)
            .collect::<Vec<_>>();
        if self.values.iter().all(|i| *i == 0) {
            self.done = true;
        }

        if self.forward {
            return Some(*self.values.last().unwrap());
        }

        Some(*self.values.first().unwrap())
    }
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<i64>().expect("should only be a number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> String {
    let oasis = parse_input(input);

    let extrapolations = oasis.into_iter().map(|v| {
        let last_numbers = HistorySteps::new(v, true).collect::<Vec<_>>();
        last_numbers
            .into_iter()
            .rev()
            .reduce(|acc, el| acc + el)
            .unwrap()
    });

    extrapolations.sum::<i64>().to_string()
}

pub fn part2(input: &str) -> String {
    let oasis = parse_input(input);

    let extrapolations = oasis.into_iter().map(|v| {
        let first_numbers = HistorySteps::new(v, false).collect::<Vec<_>>();
        first_numbers
            .into_iter()
            .rev()
            .reduce(|acc, el| el - acc)
            .unwrap()
    });

    extrapolations.sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "114");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "2");
    }
}
