use std::{collections::HashMap, fs, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NODE_RE: Regex = Regex::new(r"(?<name>\w+) = \((?<left>\w+), (?<right>\w+)\)")
        .expect("Should be a valid regex");
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

fn part1(input: &str) -> i64 {
    let mut lines = input.lines();
    let directions = lines
        .next()
        .expect("Should have directions")
        .chars()
        .collect_vec();
    lines.next();
    let mut nodes = HashMap::<String, Node>::new();
    for line in lines {
        let node = line.parse::<Node>().expect("Should be a valid node");
        nodes.insert(node.name.clone(), node);
    }

    let mut current: &str = "AAA";
    let mut res = 0i64;
    while current != "ZZZ" {
        let direction = directions[(res as usize) % directions.len()];
        let node = nodes.get(current).expect("Should be a valid node");
        current = match direction {
            'L' => &node.left,
            'R' => &node.right,
            _ => panic!("Should be a valid direction"),
        };
        res += 1;
    }

    res
}

fn part2(input: &str) -> i64 {
    let mut lines = input.lines();
    let directions = lines
        .next()
        .expect("Should have directions")
        .chars()
        .collect_vec();
    lines.next();
    let mut nodes = HashMap::<String, Node>::new();
    for line in lines {
        let node = line.parse::<Node>().expect("Should be a valid node");
        nodes.insert(node.name.clone(), node);
    }

    let mut res = 1i64;
    for start_node in nodes.values() {
        if !start_node.name.ends_with('A') {
            continue;
        }
        // println!("Trying {}", start_node.name);
        let mut cache = HashMap::<(String, usize), i64>::new();
        let mut current: &str = &start_node.name;
        let mut steps = 0i64;
        loop {
            let dir_offset = (steps as usize) % directions.len();
            let direction = directions[dir_offset];
            let cached = cache.get(&(current.to_string(), dir_offset));
            match cached {
                Some(cached) => {
                    // It looks like the size is always the same as the first Z.
                    // println!("cycle start {} size {}", cached, steps - cached);
                    // How come LCM is sufficient? Probably because of the above.
                    res = num::integer::lcm(res, steps - cached);
                    break;
                }
                None => {
                    cache.insert((current.to_string(), dir_offset), steps);
                }
            }
            let node = nodes.get(current).expect("Should be a valid node");
            current = match direction {
                'L' => &node.left,
                'R' => &node.right,
                _ => panic!("Should be a valid direction"),
            };
            steps += 1;
            // if current.ends_with('Z') {
            //     println!("{}: {}", steps, current);
            // }
        }
    }

    res
}

#[derive(Debug)]
struct ParseNodeError;

impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = NODE_RE.captures(s).ok_or(ParseNodeError)?;

        let name = caps
            .name("name")
            .ok_or(ParseNodeError)?
            .as_str()
            .parse::<String>()
            .map_err(|_| ParseNodeError)?;
        let left = caps
            .name("left")
            .ok_or(ParseNodeError)?
            .as_str()
            .parse::<String>()
            .map_err(|_| ParseNodeError)?;
        let right = caps
            .name("right")
            .ok_or(ParseNodeError)?
            .as_str()
            .parse::<String>()
            .map_err(|_| ParseNodeError)?;

        Ok(Node { name, left, right })
    }
}

fn main() {
    let file_path = "data/day08_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part1_alt() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(part2(input), 6);
    }
}
