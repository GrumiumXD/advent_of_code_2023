#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Mirror {
    Row(usize),
    Column(usize),
}

#[derive(Debug)]
struct Pattern {
    original: Vec<String>,
    transposed: Vec<String>,
}

impl Pattern {
    fn new(lines: Vec<String>) -> Self {
        let width = lines[0].len();

        let mut transposed = Vec::new();

        for i in 0..width {
            let line = lines.iter().map(|s| s.as_bytes()[i] as char).collect();
            transposed.push(line);
        }

        Pattern {
            original: lines,
            transposed,
        }
    }

    fn is_mirror_index(lines: &[String], index: usize) -> bool {
        lines[0..index]
            .iter()
            .rev()
            .zip(lines[index..].iter())
            .all(|(a, b)| a == b)
    }

    fn find_mirror_index(lines: &[String]) -> Option<usize> {
        (1..lines.len()).find(|&i| Self::is_mirror_index(lines, i))
    }

    fn find_mirror(&self) -> Mirror {
        if let Some(row) = Self::find_mirror_index(&self.original) {
            return Mirror::Row(row);
        }

        Mirror::Column(
            Self::find_mirror_index(&self.transposed).expect("must be a column at that point"),
        )
    }

    fn flip(orig: &mut [String], trans: &mut [String], x: usize, y: usize) {
        match orig[y].as_bytes()[x] as char {
            '#' => {
                orig[y].replace_range(x..x + 1, ".");
                trans[x].replace_range(y..y + 1, ".");
            }
            _ => {
                orig[y].replace_range(x..x + 1, "#");
                trans[x].replace_range(y..y + 1, "#");
            }
        }
    }

    fn find_smudge_mirror(&self) -> Mirror {
        let old_mirror = self.find_mirror();
        let mut original = self.original.clone();
        let mut transposed = self.transposed.clone();

        let width = original[0].len();
        let height = original.len();

        for y in 0..height {
            for x in 0..width {
                Self::flip(&mut original, &mut transposed, x, y);

                for row in 1..original.len() {
                    if Mirror::Row(row) != old_mirror && Self::is_mirror_index(&original, row) {
                        return Mirror::Row(row);
                    }
                }

                for col in 1..transposed.len() {
                    if Mirror::Column(col) != old_mirror && Self::is_mirror_index(&transposed, col)
                    {
                        return Mirror::Column(col);
                    }
                }
                Self::flip(&mut original, &mut transposed, x, y);
            }
        }

        unreachable!()
    }
}

fn parse_pattern(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|pattern| {
            let strs = pattern.lines().map(|s| s.to_owned()).collect::<Vec<_>>();
            Pattern::new(strs)
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> String {
    let pattern = parse_pattern(input);

    let mirror_values = pattern.iter().map(|p| match p.find_mirror() {
        Mirror::Row(x) => x * 100,
        Mirror::Column(x) => x,
    });

    mirror_values.sum::<usize>().to_string()
}

pub fn part2(input: &str) -> String {
    let pattern = parse_pattern(input);

    let mirror_values = pattern.iter().map(|p| match p.find_smudge_mirror() {
        Mirror::Row(x) => x * 100,
        Mirror::Column(x) => x,
    });

    mirror_values.sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "405");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "400");
    }
}
