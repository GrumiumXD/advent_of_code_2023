fn parse_time_and_distances(input: &str) -> Vec<(i64, i64)> {
    let data = input
        .lines()
        .map(|l| {
            let (_, numbers) = l.split_once(":").unwrap();
            numbers
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut time_and_distances = Vec::new();

    for i in 0..data[0].len() {
        time_and_distances.push((data[0][i], data[1][i]));
    }

    time_and_distances
}

pub fn part1(input: &str) -> String {
    let time_and_distances = parse_time_and_distances(input);

    let solutions = time_and_distances
        .iter()
        .map(|&(time, dist)| (0..=time).filter(|t| t * (time - t) > dist).count())
        .collect::<Vec<_>>();

    solutions.iter().product::<usize>().to_string()
}

pub fn part2(input: &str) -> String {
    let no_spaces = input.replace(" ", "");
    let time_and_distances = parse_time_and_distances(&no_spaces);

    let race = time_and_distances[0];
    let solution = (0..=race.0).filter(|t| t * (race.0 - t) > race.1).count();

    solution.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "288");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "71503");
    }
}
