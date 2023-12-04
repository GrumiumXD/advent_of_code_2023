#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(":").unwrap();
            let (left, right) = numbers.split_once("|").unwrap();
            let wn = parse_numbers(left.trim());
            let mn = parse_numbers(right.trim());
            Card {
                winning_numbers: wn,
                my_numbers: mn,
            }
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> String {
    let cards = parse_cards(input);

    let points = cards
        .iter()
        .map(|c| {
            let amount = c
                .my_numbers
                .iter()
                .filter(|&n| c.winning_numbers.contains(n))
                .count();

            if amount == 0 {
                return 0;
            }

            1 << (amount - 1)
        })
        .collect::<Vec<_>>();

    points.iter().sum::<u32>().to_string()
}

pub fn part2(input: &str) -> String {
    let cards = parse_cards(input);

    let mut card_amount = vec![1; cards.len()];

    // build an iterator for the amount of winning points per card
    let wp_count = cards.iter().map(|c| {
        c.my_numbers
            .iter()
            .filter(|&n| c.winning_numbers.contains(n))
            .count()
    });

    // go over all cards and their winning points
    for (index, amount) in wp_count.enumerate() {
        // get the multiplier for the current card
        let multiplier = card_amount.get(index).unwrap().clone();

        // increase the amount of the following cards
        let upper = card_amount.len().min(index + 1 + amount);
        for cm in card_amount[index + 1..upper].iter_mut() {
            *cm += multiplier;
        }
    }

    card_amount.iter().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "30");
    }
}
