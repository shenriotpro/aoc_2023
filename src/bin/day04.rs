use std::{
    collections::{HashMap, HashSet},
    fs,
};

use regex::Regex;

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(parse_game)
        .map(|(winning, mine)| find_matches(&winning, &mine).len())
        .map(count_points)
        .sum()
}

fn parse_game(line: &str) -> (Vec<i64>, Vec<i64>) {
    let re = Regex::new(r"\d+").expect("Should be a valid regex");
    let (_, game) = line.split_once(": ").expect("Should be a valid game");
    let (winning, mine) = game.split_once(" | ").expect("Should be a valid game");
    let winning = re
        .find_iter(winning)
        .map(|m| m.as_str().parse().expect("Should be a valid integer"))
        .collect();
    let mine = re
        .find_iter(mine)
        .map(|m| m.as_str().parse().expect("Should be a valid integer"))
        .collect();
    (winning, mine)
}

fn find_matches(winning: &[i64], mine: &[i64]) -> HashSet<i64> {
    // https://stackoverflow.com/questions/62949404/cannot-infer-type-for-type-parameter-s-when-using-hashsetfrom-iter
    let winning = HashSet::<_>::from_iter(winning.iter().cloned());
    let mine = HashSet::from_iter(mine.iter().cloned());
    winning.intersection(&mine).copied().collect()
}

fn count_points(matching: usize) -> i64 {
    match matching {
        0 => 0,
        _ => 2i64.pow(matching as u32 - 1),
    }
}

fn part2(input: &str) -> i64 {
    let mut count_by_id = HashMap::<usize, i64>::new();
    for (i, line) in input.lines().enumerate() {
        let id = i + 1;
        let (winning, mine) = parse_game(line);
        let nb_matches = find_matches(&winning, &mine).len();
        let my_count = *count_by_id.entry(id).or_insert(1);
        for next_id in id + 1..id + 1 + nb_matches {
            count_by_id
                .entry(next_id)
                .and_modify(|counter| *counter += my_count)
                .or_insert(1 + my_count);
        }
    }
    count_by_id.values().sum()
}

fn main() {
    let file_path = "data/day04_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part2(input), 30);
    }
}
