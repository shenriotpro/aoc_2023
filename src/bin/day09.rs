use std::fs;

use aoc_2023::split_parse;
use itertools::Itertools;

fn part1(input: &str) -> i64 {
    let values = input.lines().map(split_parse::<i64>).collect_vec();
    values.iter().map(|v| find_next(v)).sum()
}

fn find_next(v: &[i64]) -> i64 {
    if v.iter().all(|&x| x == 0) {
        0
    } else {
        let diffs = v.windows(2).map(|w| w[1] - w[0]).collect_vec();
        v.last().expect("Should have a value") + find_next(&diffs)
    }
}

fn part2(input: &str) -> i64 {
    let values = input.lines().map(split_parse::<i64>).collect_vec();
    values.iter().map(|v| find_previous(v)).sum()
}

fn find_previous(v: &[i64]) -> i64 {
    if v.iter().all(|&x| x == 0) {
        0
    } else {
        let diffs = v.windows(2).map(|w| w[1] - w[0]).collect_vec();
        v.first().expect("Should have a value") - find_previous(&diffs)
    }
}

fn main() {
    let file_path = "data/day09_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(part1(input), 114);
    }

    #[test]
    fn test_part2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(part2(input), 2);
    }
}
