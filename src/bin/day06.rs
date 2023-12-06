use std::{fs, iter::zip};

use aoc_2023::split_parse;
use itertools::Itertools;

fn part1(input: &str) -> i64 {
    let mut lines = input.lines();
    let times = split_parse(lines.next().expect("There should be a first line"));
    let distances = split_parse(lines.next().expect("There should be a second line"));

    zip(times, distances)
        .map(|(t, d)| {
            (0..=t)
                .filter(|push_t| simulate_push(*push_t, t) > d)
                .count() as i64
        })
        .product()
}

fn simulate_push(push_time: i64, total_time: i64) -> i64 {
    push_time * (total_time - push_time)
}

fn part2(input: &str) -> i64 {
    let mut lines = input.lines();
    let times = split_parse::<i64>(lines.next().expect("There should be a first line"));
    let distances = split_parse::<i64>(lines.next().expect("There should be a second line"));

    let time = times
        .iter()
        .map(|i| i.to_string())
        .join("")
        .parse::<i64>()
        .expect("Should be a valid integer");
    let distance = distances
        .iter()
        .map(|i| i.to_string())
        .join("")
        .parse::<i64>()
        .expect("Should be a valid integer");

    (0..=time)
        .filter(|push_t| simulate_push(*push_t, time) > distance)
        .count() as i64
}

fn main() {
    let file_path = "data/day06_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(part1(input), 288);
    }

    #[test]
    fn test_part2() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        assert_eq!(part2(input), 71503);
    }
}
