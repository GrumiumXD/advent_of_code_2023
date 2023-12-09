#[derive(Debug, PartialEq)]
struct Round {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

#[derive(Debug)]
struct Game {
    pub rounds: Vec<Round>,
}

fn parse_round(input: &str) -> Round {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    input.split(", ").for_each(|cube| {
        let (count, col) = cube.split_once(' ').unwrap();
        let count = count.parse::<usize>().unwrap();
        match col {
            "red" => r = count,
            "green" => g = count,
            "blue" => b = count,
            _ => (),
        }
    });

    Round {
        red: r,
        green: g,
        blue: b,
    }
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|lines| {
            let (_, round_data) = lines.split_once(": ").unwrap();
            Game {
                rounds: round_data.split("; ").map(parse_round).collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> String {
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    let games = parse_games(input);

    let valid_rounds = games
        .iter()
        .enumerate()
        .filter_map(|(index, g)| {
            g.rounds.iter().try_fold(index + 1, |acc, round| {
                if round.red <= MAX_RED && round.green <= MAX_GREEN && round.blue <= MAX_BLUE {
                    return Some(acc);
                }

                None
            })
        })
        .collect::<Vec<_>>();

    valid_rounds.iter().sum::<usize>().to_string()
}

pub fn part2(input: &str) -> String {
    let games = parse_games(input);

    let powers = games
        .iter()
        .map(|g| {
            let maxs = g.rounds.iter().fold((0, 0, 0), |acc, r| {
                let re = if r.red >= acc.0 { r.red } else { acc.0 };
                let gr = if r.green >= acc.1 { r.green } else { acc.1 };
                let bl = if r.blue >= acc.2 { r.blue } else { acc.2 };

                (re, gr, bl)
            });
            maxs.0 * maxs.1 * maxs.2
        })
        .collect::<Vec<_>>();

    powers.iter().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "8");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "2286");
    }
}
