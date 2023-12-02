use std::cmp::max;
use std::collections::HashMap;
use std::fs;

use regex::Regex;

fn get_id(line: &str) -> i32 {
    let re = Regex::new(r"Game (\d+): ").expect("Should be a valid regex");
    let caps = re.captures(line).expect("Should find a game id");
    caps.get(1)
        .expect("Should find an id")
        .as_str()
        .parse::<i32>()
        .expect("Should be a valid number")
}

fn get_draws(line: &str) -> Vec<Vec<(i32, String)>> {
    let mut res = vec![];
    let (_, game) = line.split_once(": ").expect("Should be a valid game");
    for draw in game.split("; ") {
        res.push(vec![]);
        let re = Regex::new(r"(?<n>\d+) (?<color>red|green|blue)").unwrap();
        for group in re.captures_iter(draw) {
            let n = group["n"].parse::<i32>().unwrap();
            let color = group["color"].to_string().clone();
            res.last_mut()
                .expect("Result should not be empty")
                .push((n, color))
        }
    }
    res
}

fn part1(input: &str) -> i32 {
    let max_possible = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut res = 0;
    for line in input.lines() {
        let mut possible = true;
        'outer: for draw in get_draws(line) {
            for (n, color) in draw {
                if n > *max_possible
                    .get(&color.as_str())
                    .expect("Should be a valid color")
                {
                    possible = false;
                    break 'outer;
                }
            }
        }
        if possible {
            res += get_id(line);
        }
    }
    res
}

fn part2(input: &str) -> i64 {
    // Note that the result may be fairly large.
    let mut res = 0;
    for line in input.lines() {
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
        res += min_possible.values().product::<i64>();
    }
    res
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
