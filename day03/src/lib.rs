use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Cell {
    Digit(u32),
    Symbol(char),
    Empty,
}

/// check for symbols in the moore neighborhood
fn check_for_symbol(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    let hmax = (grid[0].len() - 1).min(x + 1);
    let hmin = if x > 0 { x - 1 } else { 0 };

    let vmax = (grid.len() - 1).min(y + 1);
    let vmin = if y > 0 { y - 1 } else { 0 };

    for row in grid[vmin..=vmax].iter() {
        for c in row[hmin..=hmax].iter() {
            if let Cell::Symbol(_) = c {
                return true;
            }
        }
    }

    false
}

/// check for gears in the moore neighborhood and return their coordinates
fn check_for_gear(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> Option<(usize, usize)> {
    let hmax = (grid[0].len() - 1).min(x + 1);
    let hmin = if x > 0 { x - 1 } else { 0 };

    let vmax = (grid.len() - 1).min(y + 1);
    let vmin = if y > 0 { y - 1 } else { 0 };

    for (ypos, row) in grid[vmin..=vmax].iter().enumerate() {
        for (xpos, c) in row[hmin..=hmax].iter().enumerate() {
            if let Cell::Symbol('*') = c {
                return Some((xpos + hmin, ypos + vmin));
            }
        }
    }

    None
}

fn parse_grid(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0'..='9' => Cell::Digit(c.to_digit(10).unwrap()),
                    '.' => Cell::Empty,
                    _ => Cell::Symbol(c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> String {
    let grid = parse_grid(input);

    let mut part_numbers = Vec::new();
    let mut valid = false;
    let mut current_number = 0;

    // find all numbers next to a symbol
    for (y, line) in grid.iter().clone().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if let Cell::Digit(n) = c {
                current_number = 10 * current_number + n;
                if !valid {
                    valid = check_for_symbol(&grid, x, y);
                }
            } else {
                if current_number > 0 && valid {
                    part_numbers.push(current_number);
                }
                valid = false;
                current_number = 0;
            }
        }
        if current_number > 0 && valid {
            part_numbers.push(current_number);
        }
        valid = false;
        current_number = 0;
    }

    part_numbers.iter().sum::<u32>().to_string()
}

pub fn part2(input: &str) -> String {
    let grid = parse_grid(input);

    let mut part_numbers_with_gears = Vec::new();
    let mut gear = None;
    let mut current_number = 0;

    // find all numbers next to a gear
    for (y, line) in grid.iter().clone().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if let Cell::Digit(n) = c {
                current_number = 10 * current_number + n;
                if gear.is_none() {
                    gear = check_for_gear(&grid, x, y);
                }
            } else {
                if let Some(g) = gear {
                    if current_number > 0 {
                        part_numbers_with_gears.push((current_number, g));
                    }
                }
                gear = None;
                current_number = 0;
            }
        }
        if let Some(g) = gear {
            if current_number > 0 {
                part_numbers_with_gears.push((current_number, g));
            }
        }
        gear = None;
        current_number = 0;
    }

    // build a hashmap of all gears with their numbers
    let mut gear_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for pn in part_numbers_with_gears {
        let key = gear_numbers.get_mut(&pn.1);
        if let Some(g) = key {
            g.push(pn.0);
        } else {
            gear_numbers.insert(pn.1, vec![pn.0]);
        }
    }

    // filter out all gears with not exactly 2 numbers and calculate the ratio
    let ratios = gear_numbers
        .values()
        .filter(|&numbers| (numbers.len() == 2))
        .map(|numbers| numbers[0] * numbers[1])
        .collect::<Vec<u32>>();

    ratios.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "4361");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "467835");
    }
}
