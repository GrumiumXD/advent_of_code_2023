use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Pipe {
    WE,
    NS,
    NE,
    SE,
    SW,
    NW,
    Ground,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '-' => Pipe::WE,
            '|' => Pipe::NS,
            'L' => Pipe::NE,
            'F' => Pipe::SE,
            '7' => Pipe::SW,
            'J' => Pipe::NW,
            _ => Pipe::Ground,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Filled,
    Wall,
}

#[derive(Debug)]
struct TileMap {
    tiles: HashMap<(isize, isize), Tile>,
}

/// TileMap is a different representation of the
/// pipe map.
/// every pipe section is represented by a 3x3 section of tiles.
/// With that the "squeeze through pipes" can be simulated.
/// The idea is to use flood fill on the 'outer side' and then count
/// all center tiles from the 3x3 grid which were not filled.
/// those would represent the inner tiles from the original pipe
/// tiles.
impl TileMap {
    fn fill(&mut self, start: (isize, isize)) {
        let mut upcomming = vec![start];

        while let Some((x, y)) = upcomming.pop() {
            if let Some(Tile::Empty) = self.tiles.get(&(x, y)) {
                self.tiles.insert((x, y), Tile::Filled);
                upcomming.push((x + 1, y));
                upcomming.push((x, y + 1));
                upcomming.push((x - 1, y));
                upcomming.push((x, y - 1));
            }
        }
    }

    fn get_non_filled_center_count(&self) -> isize {
        self.tiles
            .iter()
            .filter(|&(k, v)| k.0 % 3 == 1 && k.1 % 3 == 1 && *v == Tile::Empty)
            .count() as isize
    }
}

#[derive(Debug)]
struct PipeMap {
    pipes: HashMap<(isize, isize), Pipe>,
    width: usize,
    height: usize,
    start: (isize, isize),
    loop_pipes: Vec<(isize, isize)>,
}

impl PipeMap {
    fn new(
        pipes: HashMap<(isize, isize), Pipe>,
        width: usize,
        height: usize,
        start: (isize, isize),
    ) -> Self {
        let mut pm = PipeMap {
            pipes,
            width,
            height,
            start,
            loop_pipes: Vec::new(),
        };

        pm.close_loop();

        pm
    }

    fn get_start_pos(&self) -> &(isize, isize) {
        &self.start
    }

    fn set_start_pipe(&mut self, pipe: Pipe) {
        self.pipes.insert(self.start, pipe);
    }

    fn get_length(&self) -> usize {
        self.loop_pipes.len()
    }

    fn walk(&self, pos: &(isize, isize), next_dir: Dir) -> Option<((isize, isize), Dir)> {
        match next_dir {
            Dir::N => {
                if pos.1 == 0 {
                    return None;
                }
                let next_pos = (pos.0, pos.1 - 1);
                match self.pipes.get(&next_pos) {
                    Some(Pipe::NS) => Some((next_pos, Dir::N)),
                    Some(Pipe::SE) => Some((next_pos, Dir::E)),
                    Some(Pipe::SW) => Some((next_pos, Dir::W)),
                    _ => None,
                }
            }
            Dir::E => {
                if pos.0 == (self.width - 1) as isize {
                    return None;
                }
                let next_pos = (pos.0 + 1, pos.1);
                match self.pipes.get(&next_pos) {
                    Some(Pipe::WE) => Some((next_pos, Dir::E)),
                    Some(Pipe::NW) => Some((next_pos, Dir::N)),
                    Some(Pipe::SW) => Some((next_pos, Dir::S)),
                    _ => None,
                }
            }
            Dir::S => {
                if pos.1 == (self.height - 1) as isize {
                    return None;
                }
                let next_pos = (pos.0, pos.1 + 1);
                match self.pipes.get(&next_pos) {
                    Some(Pipe::NS) => Some((next_pos, Dir::S)),
                    Some(Pipe::NE) => Some((next_pos, Dir::E)),
                    Some(Pipe::NW) => Some((next_pos, Dir::W)),
                    _ => None,
                }
            }
            Dir::W => {
                if pos.0 == 0 {
                    return None;
                }
                let next_pos = (pos.0 - 1, pos.1);
                match self.pipes.get(&next_pos) {
                    Some(Pipe::WE) => Some((next_pos, Dir::W)),
                    Some(Pipe::NE) => Some((next_pos, Dir::N)),
                    Some(Pipe::SE) => Some((next_pos, Dir::S)),
                    _ => None,
                }
            }
        }
    }

    /// close_loop tries all 6 possible section on the start tile
    /// to see which is closing the loop.
    /// it also walks the loop and collects all coordinates part of
    /// the loop
    fn close_loop(&mut self) {
        let possible_pipes = [Pipe::NS, Pipe::WE, Pipe::NE, Pipe::SE, Pipe::SW, Pipe::NW];

        let lengths = possible_pipes.iter().map(|p| {
            self.set_start_pipe(p.clone());
            let mut pos = *self.get_start_pos();

            let mut loop_pipes = Vec::new();
            let mut next_dir = match p {
                Pipe::NE | Pipe::NW | Pipe::NS => Dir::N,
                Pipe::SE | Pipe::SW => Dir::S,
                _ => Dir::E,
            };

            loop {
                let next = self.walk(&pos, next_dir);
                let Some((np, nd)) = next else {
                    return None;
                };
                pos = np;
                next_dir = nd;
                loop_pipes.push(pos);

                if self.get_start_pos() == &pos {
                    break;
                }
            }

            Some((p.clone(), loop_pipes))
        });

        let (pipe, loop_pipes) = lengths.flatten().next().expect("One path should exist");

        self.loop_pipes = loop_pipes;
        self.set_start_pipe(pipe);
    }

    // this converts the pipe map into a tile map
    // every pipe tile is represented by a 3x3 group of tiles.
    // This is so the "squeeze beetwen pipes" can be simulated
    fn create_tile_map(&self) -> TileMap {
        let mut tiles = HashMap::new();
        for (pos, p) in self.pipes.iter() {
            if self.loop_pipes.contains(pos) {
                match p {
                    Pipe::WE => {
                        tiles.insert((pos.0 * 3, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3, pos.1 * 3 + 1), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 1), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 1), Tile::Wall);
                        tiles.insert((pos.0 * 3, pos.1 * 3 + 2), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 2), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 2), Tile::Empty);
                    }
                    Pipe::NS => {
                        tiles.insert((pos.0 * 3, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3, pos.1 * 3 + 1), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 1), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 1), Tile::Empty);
                        tiles.insert((pos.0 * 3, pos.1 * 3 + 2), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 2), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 2), Tile::Empty);
                    }
                    Pipe::NE => {
                        tiles.insert((pos.0 * 3, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3, pos.1 * 3 + 1), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 1), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 1), Tile::Wall);
                        tiles.insert((pos.0 * 3, pos.1 * 3 + 2), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 2), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 2), Tile::Empty);
                    }
                    Pipe::SE => {
                        tiles.insert((pos.0 * 3, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3, pos.1 * 3 + 1), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 1), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 1), Tile::Wall);
                        tiles.insert((pos.0 * 3, pos.1 * 3 + 2), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 2), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 2), Tile::Empty);
                    }
                    Pipe::SW => {
                        tiles.insert((pos.0 * 3, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3, pos.1 * 3 + 1), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 1), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 1), Tile::Empty);
                        tiles.insert((pos.0 * 3, pos.1 * 3 + 2), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 2), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 2), Tile::Empty);
                    }
                    Pipe::NW => {
                        tiles.insert((pos.0 * 3, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3), Tile::Empty);
                        tiles.insert((pos.0 * 3, pos.1 * 3 + 1), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 1), Tile::Wall);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 1), Tile::Empty);
                        tiles.insert((pos.0 * 3, pos.1 * 3 + 2), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 2), Tile::Empty);
                        tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 2), Tile::Empty);
                    }
                    _ => (),
                }
            } else {
                tiles.insert((pos.0 * 3, pos.1 * 3), Tile::Empty);
                tiles.insert((pos.0 * 3 + 1, pos.1 * 3), Tile::Empty);
                tiles.insert((pos.0 * 3 + 2, pos.1 * 3), Tile::Empty);
                tiles.insert((pos.0 * 3, pos.1 * 3 + 1), Tile::Empty);
                tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 1), Tile::Empty);
                tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 1), Tile::Empty);
                tiles.insert((pos.0 * 3, pos.1 * 3 + 2), Tile::Empty);
                tiles.insert((pos.0 * 3 + 1, pos.1 * 3 + 2), Tile::Empty);
                tiles.insert((pos.0 * 3 + 2, pos.1 * 3 + 2), Tile::Empty);
            }
        }

        TileMap { tiles }
    }
}

fn parse_pipes(input: &str) -> PipeMap {
    let mut pipes = HashMap::new();

    let height = input.lines().count();
    let width = input
        .lines()
        .next()
        .expect("At least one line schould be in the input")
        .chars()
        .count();

    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            pipes.insert((x as isize, y as isize), c.into());

            if c == 'S' {
                start = (x as isize, y as isize);
            }
        }
    }

    PipeMap::new(pipes, width, height, start)
}

pub fn part1(input: &str) -> String {
    let pipe_map = parse_pipes(input);

    (pipe_map.get_length() / 2).to_string()
}

pub fn part2(input: &str) -> String {
    let pipe_map = parse_pipes(input);

    let mut tile_map = pipe_map.create_tile_map();

    tile_map.fill((0, 0));

    tile_map.get_non_filled_center_count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const INPUT2: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const INPUT3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const INPUT4: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const INPUT5: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn p1_exapmple1() {
        let result = part1(INPUT1);
        assert_eq!(result, "4");
    }

    #[test]
    fn p1_exapmple2() {
        let result = part1(INPUT2);
        assert_eq!(result, "8");
    }

    #[test]
    fn p2_example1() {
        let result = part2(INPUT3);
        assert_eq!(result, "4");
    }

    #[test]
    fn p2_example2() {
        let result = part2(INPUT4);
        assert_eq!(result, "8");
    }

    #[test]
    fn p2_example3() {
        let result = part2(INPUT5);
        assert_eq!(result, "10");
    }
}
