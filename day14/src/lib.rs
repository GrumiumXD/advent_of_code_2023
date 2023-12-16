use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{fmt, ops::Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Rock,
    Cube,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'O' => Tile::Rock,
            '#' => Tile::Cube,
            _ => Tile::Empty,
        }
    }
}

#[derive(Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Hash)]
struct Grid {
    tiles: Vec<Tile>,
    width: usize,
}

impl Grid {
    fn new(tiles: Vec<Tile>, width: usize) -> Self {
        Grid { tiles, width }
    }

    fn get_height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get(&self, pos: (usize, usize)) -> Option<Tile> {
        self.tiles.get(pos.1 * self.width + pos.0).copied()
    }

    fn set(&mut self, pos: (usize, usize), tile: Tile) {
        if let Some(t) = self.tiles.get_mut(pos.1 * self.width + pos.0) {
            *t = tile;
        }
    }

    fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    fn tilt(&mut self, dir: Dir) {
        let height = self.get_height();
        let width = self.get_width();
        match dir {
            Dir::N => {
                for x in 0..width {
                    let rocks = self.rock_count(x, true);
                    for (range, count) in rocks {
                        for r in range.start..range.start + count {
                            self.set((x, r), Tile::Rock);
                        }
                        for e in range.start + count..range.end {
                            self.set((x, e), Tile::Empty);
                        }
                    }
                }
            }
            Dir::E => {
                for y in 0..height {
                    let rocks = self.rock_count(y, false);
                    for (range, count) in rocks {
                        let diff = range.len() - count;
                        for r in range.start..range.start + diff {
                            self.set((r, y), Tile::Empty);
                        }
                        for e in range.start + diff..range.end {
                            self.set((e, y), Tile::Rock);
                        }
                    }
                }
            }
            Dir::S => {
                for x in 0..width {
                    let rocks = self.rock_count(x, true);
                    for (range, count) in rocks {
                        let diff = range.len() - count;
                        for r in range.start..range.start + diff {
                            self.set((x, r), Tile::Empty);
                        }
                        for e in range.start + diff..range.end {
                            self.set((x, e), Tile::Rock);
                        }
                    }
                }
            }
            Dir::W => {
                for y in 0..height {
                    let rocks = self.rock_count(y, false);
                    for (range, count) in rocks {
                        for r in range.start..range.start + count {
                            self.set((r, y), Tile::Rock);
                        }
                        for e in range.start + count..range.end {
                            self.set((e, y), Tile::Empty);
                        }
                    }
                }
            }
        }
    }

    fn iter(&self) -> impl Iterator<Item = ((usize, usize), &Tile)> {
        self.tiles
            .iter()
            .enumerate()
            .map(|(index, t)| ((index % self.width, index / self.width), t))
    }

    fn rock_count(&self, index: usize, vertical: bool) -> Vec<(Range<usize>, usize)> {
        let mut counts = Vec::new();

        let mut count = 0;
        let mut start = 0;

        self.iter()
            .filter(|&(p, _)| (vertical && p.0 == index) || (!vertical && p.1 == index))
            .for_each(|(p, t)| match t {
                Tile::Rock => count += 1,
                Tile::Cube => {
                    let end = if vertical { p.1 } else { p.0 };
                    counts.push((start..end, count));
                    start = end + 1;
                    count = 0;
                }
                Tile::Empty => (),
            });

        let end = if vertical {
            self.get_height()
        } else {
            self.get_width()
        };
        counts.push((start..end, count));

        counts
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                match self.get((x, y)) {
                    Some(Tile::Rock) => write!(f, "O")?,
                    Some(Tile::Empty) => write!(f, ".")?,
                    Some(Tile::Cube) => write!(f, "#")?,
                    None => unreachable!(),
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut width = 0;
    let tiles = input
        .lines()
        .flat_map(|line| {
            if width == 0 {
                width = line.len();
            }
            line.chars().map(char::into)
        })
        .collect::<Vec<_>>();

    Grid::new(tiles, width)
}

pub fn part1(input: &str) -> String {
    let mut grid = parse_grid(input);

    let height = grid.get_height();
    // println!("{}", &grid);

    grid.tilt(Dir::N);

    // println!("{}", &grid);

    let loads = grid
        .iter()
        .filter_map(|(p, t)| (*t == Tile::Rock).then_some(height - p.1));

    loads.sum::<usize>().to_string()
}

pub fn part2(input: &str) -> String {
    let mut grid = parse_grid(input);

    let height = grid.get_height();
    let mut hashes = Vec::new();
    hashes.push(grid.calculate_hash());

    let mut step = 0;
    let mut cycle_found = false;

    while step < 1000000000 {
        grid.tilt(Dir::N);
        grid.tilt(Dir::W);
        grid.tilt(Dir::S);
        grid.tilt(Dir::E);

        if !cycle_found {
            let new_hash = grid.calculate_hash();

            if let Some(index) = hashes
                .iter()
                .enumerate()
                .find_map(|(i, hash)| (*hash == new_hash).then_some(i))
            {
                cycle_found = true;
                let diff = step - (index - 1);
                while step < 1000000000 {
                    step += diff;
                }
                step -= diff;
            } else {
                hashes.push(new_hash);
            }
        }

        step += 1;
    }

    let loads = grid
        .iter()
        .filter_map(|(p, t)| (*t == Tile::Rock).then_some(height - p.1));

    (loads.sum::<usize>()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "136");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "64");
    }
}
