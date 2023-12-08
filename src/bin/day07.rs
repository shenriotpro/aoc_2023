use std::{fmt, fs, str::FromStr};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Card {
    name: char,
    value1: i64,
    value2: i64,
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: i64,
    value1: (i64, [i64; 5]),
    value2: (i64, [i64; 5]),
}

fn part1(input: &str) -> i64 {
    let mut hands = input
        .lines()
        .map(|line| line.parse::<Hand>().expect("Should be a valid hand"))
        .collect_vec();

    hands.sort_by_key(|h| h.value1);

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| ((i + 1) as i64) * hand.bid)
        .sum::<i64>()
}

fn part2(input: &str) -> i64 {
    let mut hands = input
        .lines()
        .map(|line| line.parse::<Hand>().expect("Should be a valid hand"))
        .collect_vec();

    hands.sort_by_key(|h| h.value2);

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| ((i + 1) as i64) * hand.bid)
        .sum::<i64>()
}

impl From<char> for Card {
    fn from(name: char) -> Self {
        let value1 = match name {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => name.to_digit(10).expect("Should be a valid card") as i64,
        };
        let value2 = match name {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'T' => 10,
            'J' => 1,
            _ => name.to_digit(10).expect("Should be a valid card") as i64,
        };
        Card {
            name,
            value1,
            value2,
        }
    }
}

fn cards_value1(cards: [Card; 5]) -> i64 {
    let mut sorted = cards;
    sorted.sort_by_key(|c| c.value1);
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

fn cards_value2(cards: [Card; 5]) -> i64 {
    let base_cards = cards.iter().filter(|c| c.name != 'J').collect_vec();
    let j_count = 5 - base_cards.len();
    let mut sorted = base_cards.clone();
    sorted.sort_by_key(|c| c.value2);
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

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards_fmt = self.cards.iter().map(|c| c.name).collect::<String>();

        write!(f, "{cards_fmt} {}", self.bid)
    }
}

#[derive(Debug)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_whitespace().collect_tuple().ok_or(ParseHandError)?;

        let cards = cards
            .chars()
            .map(Card::from)
            .collect_vec()
            .try_into()
            .map_err(|_| ParseHandError)?;
        let bid = bid.parse().map_err(|_| ParseHandError)?;

        let value1 = (
            cards_value1(cards),
            cards
                .iter()
                .map(|c| c.value1)
                .collect_vec()
                .try_into()
                .map_err(|_| ParseHandError)?,
        );
        let value2 = (
            cards_value2(cards),
            cards
                .iter()
                .map(|c| c.value2)
                .collect_vec()
                .try_into()
                .map_err(|_| ParseHandError)?,
        );

        Ok(Hand {
            cards,
            bid,
            value1,
            value2,
        })
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
