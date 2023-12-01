use aho_corasick::AhoCorasick;

pub fn part1(input: &str) -> String {
    let calibration_values = input
        .lines()
        .map(|l| {
            // get the digits from every line
            let mut digits = l.chars().filter_map(|c| c.to_digit(10));

            let first = digits.next().unwrap();
            let second = digits.next_back().unwrap_or(first);

            10 * first + second
        })
        .collect::<Vec<_>>();

    calibration_values.iter().sum::<u32>().to_string()
}

pub fn part2(input: &str) -> String {
    let s_pattern = &["two", "eight"];
    let s_replace = &["ttwoo", "eeightt"];
    let d_pattern = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let d_replace = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let ac_sanitize = AhoCorasick::new(s_pattern).unwrap();
    let ac_digitize = AhoCorasick::new(d_pattern).unwrap();

    let calibration_values = input
        .lines()
        .map(|l| {
            let sanitized = ac_sanitize.replace_all(l, s_replace);
            let digitized = ac_digitize.replace_all(&sanitized, d_replace);
            // get the digits from every line
            let mut digits = digitized.chars().filter_map(|c| c.to_digit(10));

            let first = digits.next().unwrap();
            let second = digits.next_back().unwrap_or(first);

            10 * first + second
        })
        .collect::<Vec<_>>();

    calibration_values.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn p1() {
        let result = part1(INPUT1);
        assert_eq!(result, "142");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT2);
        assert_eq!(result, "281");
    }
}
