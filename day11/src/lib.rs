use itertools::Itertools;

fn parse_galaxies(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '#').then_some((x, y)))
        })
        .collect::<Vec<_>>()
}

fn expand(galaxies: &mut Vec<(usize, usize)>, expansion: usize) {
    let max_x = galaxies
        .iter()
        .map(|(x, _)| *x)
        .max()
        .expect("there should be a maximum x");
    let max_y = galaxies
        .iter()
        .map(|(_, y)| *y)
        .max()
        .expect("there should be a maximum y");

    let empty_cols = (0..max_x)
        .into_iter()
        .filter(|x| !galaxies.iter().any(|(gx, _)| *gx == *x))
        .collect::<Vec<_>>();
    let empty_rows = (0..max_y)
        .into_iter()
        .filter(|y| !galaxies.iter().any(|(_, gy)| *gy == *y))
        .collect::<Vec<_>>();

    // move all galaxies "right" of an empty column
    // further by the expansion-1
    // this essentially "replaces" all empty columns
    // with `expansion` times empty columns
    for c in empty_cols.iter().rev() {
        for g in galaxies.iter_mut() {
            if g.0 > *c {
                *g = (g.0 + (expansion - 1), g.1);
            }
        }
    }
    // move all galaxies "below" of an empty row
    // further by the expansion-1
    // this essentially "replaces" all empty rows
    // with `expansion` times rows columns
    for r in empty_rows.iter().rev() {
        for g in galaxies.iter_mut() {
            if g.1 > *r {
                *g = (g.0, g.1 + (expansion - 1));
            }
        }
    }
}

pub fn part1(input: &str) -> String {
    let mut galaxies = parse_galaxies(input);
    expand(&mut galaxies, 2);

    let distances = galaxies
        .iter()
        .combinations(2)
        .map(|pair| pair[0].0.abs_diff(pair[1].0) + pair[0].1.abs_diff(pair[1].1));

    distances.sum::<usize>().to_string()
}

pub fn part2(input: &str, expansion: usize) -> String {
    let mut galaxies = parse_galaxies(input);
    expand(&mut galaxies, expansion);

    let distances = galaxies
        .iter()
        .combinations(2)
        .map(|pair| pair[0].0.abs_diff(pair[1].0) + pair[0].1.abs_diff(pair[1].1));

    distances.sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "374");
    }

    #[test]
    fn p2_exp10() {
        let result = part2(INPUT, 10);
        assert_eq!(result, "1030");
    }

    #[test]
    fn p2_exp100() {
        let result = part2(INPUT, 100);
        assert_eq!(result, "8410");
    }
}
