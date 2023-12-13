use argh::FromArgs;
use std::error;

#[derive(FromArgs, PartialEq, Debug)]
/// day and part options
struct Options {
    /// what day to run
    #[argh(positional)]
    day: usize,

    /// optional selection for what part to calculate
    #[argh(option, short = 'p')]
    part: Option<u8>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let opts: Options = argh::from_env();

    let parts = match opts.part {
        Some(1) => (true, false),
        Some(2) => (false, true),
        _ => (true, true),
    };

    let inputs = [
        include_str!("../../inputs/day01.txt"),
        include_str!("../../inputs/day02.txt"),
        include_str!("../../inputs/day03.txt"),
        include_str!("../../inputs/day04.txt"),
        include_str!("../../inputs/day05.txt"),
        include_str!("../../inputs/day06.txt"),
        include_str!("../../inputs/day07.txt"),
        include_str!("../../inputs/day08.txt"),
        include_str!("../../inputs/day09.txt"),
        include_str!("../../inputs/day10.txt"),
        include_str!("../../inputs/day11.txt"),
        include_str!("../../inputs/day12.txt"),
    ];

    if opts.day < 1 && opts.day > inputs.len() {
        return Err("Invalid day selected!".into());
    }

    println!("Selected day: {}", &opts.day);
    let input = *inputs.get(opts.day - 1).unwrap();

    let results = match opts.day {
        1 => (
            parts.0.then(|| day01::part1(input)),
            parts.1.then(|| day01::part2(input)),
        ),
        2 => (
            parts.0.then(|| day02::part1(input)),
            parts.1.then(|| day02::part2(input)),
        ),
        3 => (
            parts.0.then(|| day03::part1(input)),
            parts.1.then(|| day03::part2(input)),
        ),
        4 => (
            parts.0.then(|| day04::part1(input)),
            parts.1.then(|| day04::part2(input)),
        ),
        5 => (
            parts.0.then(|| day05::part1(input)),
            parts.1.then(|| day05::part2(input)),
        ),
        6 => (
            parts.0.then(|| day06::part1(input)),
            parts.1.then(|| day06::part2(input)),
        ),
        7 => (
            parts.0.then(|| day07::part1(input)),
            parts.1.then(|| day07::part2(input)),
        ),
        8 => (
            parts.0.then(|| day08::part1(input)),
            parts.1.then(|| day08::part2(input)),
        ),
        9 => (
            parts.0.then(|| day09::part1(input)),
            parts.1.then(|| day09::part2(input)),
        ),
        10 => (
            parts.0.then(|| day10::part1(input)),
            parts.1.then(|| day10::part2(input)),
        ),
        11 => (
            parts.0.then(|| day11::part1(input)),
            parts.1.then(|| day11::part2(input, 1000000)),
        ),
        12 => (
            parts.0.then(|| day12::part1(input)),
            parts.1.then(|| day12::part2(input)),
        ),

        _ => (None, None),
    };

    if let Some(r) = results.0 {
        println!("Part 1: {}", r)
    }
    if let Some(r) = results.1 {
        println!("Part 2: {}", r)
    }

    Ok(())
}
