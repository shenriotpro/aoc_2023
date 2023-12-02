use std::cmp::max;
use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ID_RE: Regex = Regex::new(r"Game (\d+): ").expect("Should be a valid regex");
    static ref DRAWS_RE: Regex =
        Regex::new(r"(?<n>\d+) (?<color>red|green|blue)").expect("Should be a valid regex");
}

fn get_id(line: &str) -> i32 {
    let caps = ID_RE.captures(line).expect("Should find a game id");
    caps.get(1)
        .expect("Should find an id")
        .as_str()
        .parse::<i32>()
        .expect("Should be a valid number")
}

fn get_draws(line: &str) -> Vec<Vec<(i32, String)>> {
    let (_, game) = line.split_once(": ").expect("Should be a valid game");
    game.split("; ")
        .map(|draw| {
            DRAWS_RE
                .captures_iter(draw)
                .map(|group| {
                    let n = group["n"].parse::<i32>().expect("Should be a valid number");
                    let color = group["color"].to_string().clone();
                    (n, color)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> i32 {
    let max_possible = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    input
        .lines()
        .filter(|line| {
            get_draws(line).iter().all(|draw| {
                draw.iter().all(|(n, color)| {
                    n <= max_possible
                        .get(&color.as_str())
                        .expect("Should be a valid color")
                })
            })
        })
        .map(get_id)
        .sum()
}

fn part2(input: &str) -> i64 {
    // Note that the result may be fairly large.
    input
        .lines()
        .map(|line| {
            let mut min_possible = HashMap::from([
                ("red".to_string(), 0i64),
                ("green".to_string(), 0i64),
                ("blue".to_string(), 0i64),
            ]);
            for draw in get_draws(line) {
                for (n, color) in draw {
                    min_possible
                        .entry(color)
                        .and_modify(|e| *e = max(*e, n.into()));
                }
            }
            min_possible.values().product::<i64>()
        })
        .sum()
}

fn main() {
    let file_path = "data/day02_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part2(input), 2286);
    }
}
