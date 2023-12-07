use std::{fmt, fs, str::FromStr};

use cached::proc_macro::cached;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Card1 {
    name: char,
}

#[derive(PartialEq, Eq)]
struct Hand1 {
    cards: [Card1; 5],
    bid: i64,
}

fn part1(input: &str) -> i64 {
    let mut hands = input
        .lines()
        .map(|line| line.parse::<Hand1>().expect("Should be a valid hand"))
        .collect_vec();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| ((i + 1) as i64) * hand.bid)
        .sum::<i64>()
}

impl Card1 {
    fn value(&self) -> i64 {
        name_value1(self.name)
    }
}

#[cached]
fn name_value1(name: char) -> i64 {
    match name {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => name.to_digit(10).expect("Should be a valid card") as i64,
    }
}

impl PartialOrd for Card1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl Hand1 {
    fn value(&self) -> i64 {
        cards_value1(self.cards)
    }
}

#[cached]
fn cards_value1(cards: [Card1; 5]) -> i64 {
    let mut sorted = cards;
    sorted.sort();
    let groups = sorted.iter().group_by(|c| *c);
    let groups_sizes = groups
        .into_iter()
        .map(|(_, group)| group.collect_vec().len())
        .sorted()
        .collect_vec();

    if groups_sizes.contains(&5) {
        7
    } else if groups_sizes.contains(&4) {
        6
    } else if groups_sizes.contains(&3) && groups_sizes.contains(&2) {
        5
    } else if groups_sizes.contains(&3) {
        4
    } else if groups_sizes.ends_with(&[2, 2]) {
        3
    } else if groups_sizes.contains(&2) {
        2
    } else {
        1
    }
}

impl PartialOrd for Hand1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.value(), self.cards).cmp(&(other.value(), other.cards))
    }
}

impl fmt::Debug for Hand1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards_fmt = self.cards.iter().map(|c| c.name).collect::<String>();

        write!(f, "{cards_fmt} {}", self.bid)
    }
}

#[derive(Debug)]
struct ParseHand1Error;

impl FromStr for Hand1 {
    type Err = ParseHand1Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s
            .split_whitespace()
            .collect_tuple()
            .ok_or(ParseHand1Error)?;

        let cards = cards
            .chars()
            .map(|c| Card1 { name: c })
            .collect_vec()
            .try_into()
            .map_err(|_| ParseHand1Error)?;
        let bid = bid.parse().map_err(|_| ParseHand1Error)?;

        Ok(Hand1 { cards, bid })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Card2 {
    name: char,
}

#[derive(PartialEq, Eq)]
struct Hand2 {
    cards: [Card2; 5],
    bid: i64,
}

fn part2(input: &str) -> i64 {
    let mut hands = input
        .lines()
        .map(|line| line.parse::<Hand2>().expect("Should be a valid hand"))
        .collect_vec();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| ((i + 1) as i64) * hand.bid)
        .sum::<i64>()
}

impl Card2 {
    fn value(&self) -> i64 {
        name_value2(self.name)
    }
}

#[cached]
fn name_value2(name: char) -> i64 {
    match name {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        'J' => 1,
        _ => name.to_digit(10).expect("Should be a valid card") as i64,
    }
}

impl PartialOrd for Card2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl Hand2 {
    fn value(&self) -> i64 {
        cards_value2(self.cards)
    }
}

#[cached]
fn cards_value2(cards: [Card2; 5]) -> i64 {
    if cards.contains(&Card2 { name: 'J' }) {
        return cards_value2_j(cards);
    }
    let mut sorted = cards;
    sorted.sort();
    let groups = sorted.iter().group_by(|c| *c);
    let groups_sizes = groups
        .into_iter()
        .map(|(_, group)| group.collect_vec().len())
        .sorted()
        .collect_vec();

    if groups_sizes.contains(&5) {
        7
    } else if groups_sizes.contains(&4) {
        6
    } else if groups_sizes.contains(&3) && groups_sizes.contains(&2) {
        5
    } else if groups_sizes.contains(&3) {
        4
    } else if groups_sizes.ends_with(&[2, 2]) {
        3
    } else if groups_sizes.contains(&2) {
        2
    } else {
        1
    }
}

fn cards_value2_j(cards: [Card2; 5]) -> i64 {
    let base_cards = cards.iter().filter(|c| c.name != 'J').collect_vec();
    let j_count = 5 - base_cards.len();
    let mut sorted = base_cards.clone();
    sorted.sort();
    let groups = sorted.iter().group_by(|c| *c);
    let groups_sizes = groups
        .into_iter()
        .map(|(_, group)| group.collect_vec().len())
        .sorted()
        .collect_vec();

    if j_count == 5 || groups_sizes.contains(&(5 - j_count)) {
        7
    } else if groups_sizes.contains(&(4 - j_count)) {
        6
    } else if groups_sizes.iter().rev().take(2).sum::<usize>() >= 5 - j_count {
        5
    } else if groups_sizes.contains(&(3 - j_count)) {
        4
    } else if groups_sizes.iter().rev().take(2).sum::<usize>() >= 4 - j_count {
        3
    } else if groups_sizes.contains(&(2 - j_count)) {
        2
    } else {
        1
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.value(), self.cards).cmp(&(other.value(), other.cards))
    }
}

impl fmt::Debug for Hand2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards_fmt = self.cards.iter().map(|c| c.name).collect::<String>();

        write!(f, "{cards_fmt} {}", self.bid)
    }
}

#[derive(Debug)]
struct ParseHand2Error;

impl FromStr for Hand2 {
    type Err = ParseHand2Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s
            .split_whitespace()
            .collect_tuple()
            .ok_or(ParseHand2Error)?;

        let cards = cards
            .chars()
            .map(|c| Card2 { name: c })
            .collect_vec()
            .try_into()
            .map_err(|_| ParseHand2Error)?;
        let bid = bid.parse().map_err(|_| ParseHand2Error)?;

        Ok(Hand2 { cards, bid })
    }
}

fn main() {
    let file_path = "data/day07_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(part1(input), 6440);
    }

    #[test]
    fn test_part2() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        assert_eq!(part2(input), 5905);
    }
}
