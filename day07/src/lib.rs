use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Joker,
            'Q' => Card::Queen,
            'K' => Card::King,
            _ => Card::Ace,
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Type {
    HighCard([Card; 5]),
    OnePair([Card; 5]),
    TwoPair([Card; 5]),
    ThreeOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    FourOfAKind([Card; 5]),
    FiveOfAKind([Card; 5]),
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        let cards: [Card; 5] = value
            .chars()
            .map(|c| c.into())
            .collect::<Vec<_>>()
            .try_into()
            .expect("should be length 5");

        let unique = cards.iter().collect::<HashSet<_>>();
        let mut occurrences = unique
            .iter()
            .map(|&c| cards.iter().filter(|&card| *card == *c).count())
            .collect::<Vec<_>>();

        occurrences.sort_by(|a, b| b.cmp(a));
        match (occurrences.first(), occurrences.get(1)) {
            (Some(5), _) => Type::FiveOfAKind(cards),
            (Some(4), _) => Type::FourOfAKind(cards),
            (Some(3), Some(2)) => Type::FullHouse(cards),
            (Some(3), Some(1)) => Type::ThreeOfAKind(cards),
            (Some(2), Some(2)) => Type::TwoPair(cards),
            (Some(2), Some(1)) => Type::OnePair(cards),
            _ => Type::HighCard(cards),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Type,
    bet: usize,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum JCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for JCard {
    fn from(value: char) -> Self {
        match value {
            '2' => JCard::Two,
            '3' => JCard::Three,
            '4' => JCard::Four,
            '5' => JCard::Five,
            '6' => JCard::Six,
            '7' => JCard::Seven,
            '8' => JCard::Eight,
            '9' => JCard::Nine,
            'T' => JCard::Ten,
            'J' => JCard::Joker,
            'Q' => JCard::Queen,
            'K' => JCard::King,
            _ => JCard::Ace,
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum JType {
    HighCard([JCard; 5]),
    OnePair([JCard; 5]),
    TwoPair([JCard; 5]),
    ThreeOfAKind([JCard; 5]),
    FullHouse([JCard; 5]),
    FourOfAKind([JCard; 5]),
    FiveOfAKind([JCard; 5]),
}

impl From<&str> for JType {
    fn from(value: &str) -> Self {
        let cards: [JCard; 5] = value
            .chars()
            .map(|c| c.into())
            .collect::<Vec<_>>()
            .try_into()
            .expect("should be length 5");

        let unique = cards.iter().collect::<HashSet<_>>();
        let mut occurrences = unique
            .iter()
            .map(|&c| cards.iter().filter(|&card| *card == *c).count())
            .collect::<Vec<_>>();

        let joker_amount = cards.iter().filter(|&card| *card == JCard::Joker).count();

        occurrences.sort_by(|a, b| b.cmp(a));
        match (occurrences.first(), occurrences.get(1), joker_amount) {
            (Some(5), _, _) => JType::FiveOfAKind(cards),
            (Some(4), _, 4) => JType::FiveOfAKind(cards),
            (Some(4), _, 1) => JType::FiveOfAKind(cards),
            (Some(4), _, 0) => JType::FourOfAKind(cards),
            (Some(3), Some(2), 3) => JType::FiveOfAKind(cards),
            (Some(3), Some(2), 2) => JType::FiveOfAKind(cards),
            (Some(3), Some(2), 0) => JType::FullHouse(cards),
            (Some(3), Some(1), 3) => JType::FourOfAKind(cards),
            (Some(3), Some(1), 1) => JType::FourOfAKind(cards),
            (Some(3), Some(1), 0) => JType::ThreeOfAKind(cards),
            (Some(2), Some(2), 2) => JType::FourOfAKind(cards),
            (Some(2), Some(2), 1) => JType::FullHouse(cards),
            (Some(2), Some(2), 0) => JType::TwoPair(cards),
            (Some(2), Some(1), 2) => JType::ThreeOfAKind(cards),
            (Some(2), Some(1), 1) => JType::ThreeOfAKind(cards),
            (Some(2), Some(1), 0) => JType::OnePair(cards),
            (Some(1), _, 1) => JType::OnePair(cards),
            _ => JType::HighCard(cards),
        }
    }
}

#[derive(Debug)]
struct JHand {
    cards: JType,
    bet: usize,
}

pub fn part1(input: &str) -> String {
    let mut hands = input
        .lines()
        .map(|line| {
            let (h, b) = line.split_once(' ').unwrap();

            Hand {
                cards: h.into(),
                bet: b.parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| a.cards.cmp(&b.cards));

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bet);

    winnings.sum::<usize>().to_string()
}

pub fn part2(input: &str) -> String {
    let mut hands = input
        .lines()
        .map(|line| {
            let (h, b) = line.split_once(' ').unwrap();

            JHand {
                cards: h.into(),
                bet: b.parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| a.cards.cmp(&b.cards));

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bet);

    winnings.sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "6440");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "5905");
    }
}
