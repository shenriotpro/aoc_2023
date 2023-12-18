use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use itertools::Itertools;

fn part1(input: &str) -> i64 {
    let (mut i, mut j) = (0, 0);
    let mut trench = HashSet::new();
    trench.insert((i, j));

    let mut res = 0i64;
    let mut res2 = 0i64;
    for line in input.lines() {
        let (dir, steps, _) = line
            .split_whitespace()
            .collect_tuple()
            .expect("Should be a valid triplet");
        let steps = steps.parse::<i64>().expect("Should be a valid number");
        let (oi, oj) = (i, j);
        for _ in 0..steps {
            match dir {
                "R" => j += 1,
                "L" => j -= 1,
                "U" => i -= 1,
                "D" => i += 1,
                _ => panic!("Should be a valid direction"),
            }
            // TODO: check we don't have many loops
            if !trench.contains(&(i, j)) {
                trench.insert((i, j));
                res += 1;
            }
        }
        res2 += oi * j - oj * i;
    }
    println!("{}", 2 + res / 2 + res2.abs() / 2);
    let mut queue = VecDeque::new();
    // TODO: lucky guess
    queue.push_back((1, 1));
    let mut visited = HashSet::new();
    visited.insert((i, j));
    while !queue.is_empty() {
        let (i, j) = queue.pop_front().expect("Should not be empty");
        res += 1;
        if !visited.contains(&(i + 1, j)) && !trench.contains(&(i + 1, j)) {
            queue.push_back((i + 1, j));
            visited.insert((i + 1, j));
        }
        if !visited.contains(&(i, j + 1)) && !trench.contains(&(i, j + 1)) {
            queue.push_back((i, j + 1));
            visited.insert((i, j + 1));
        }
        if !visited.contains(&(i - 1, j)) && !trench.contains(&(i - 1, j)) {
            queue.push_back((i - 1, j));
            visited.insert((i - 1, j));
        }
        if !visited.contains(&(i, j - 1)) && !trench.contains(&(i, j - 1)) {
            queue.push_back((i, j - 1));
            visited.insert((i, j - 1));
        }
    }

    res
}

fn part2(input: &str) -> i64 {
    let (mut i, mut j) = (0, 0);

    let mut res = 0i64;
    let mut res2 = 0i64;
    for line in input.lines() {
        let (_, _, hex) = line
            .split_whitespace()
            .collect_tuple()
            .expect("Should be a valid triplet");
        let steps = i64::from_str_radix(&hex[2..7], 16).expect("Should be a valid hex number");
        let dir = match hex.chars().nth(7) {
            Some('0') => "R",
            Some('1') => "D",
            Some('2') => "L",
            Some('3') => "U",
            _ => panic!("Should be a valid direction"),
        };
        let (oi, oj) = (i, j);
        match dir {
            "R" => j += steps,
            "L" => j -= steps,
            "U" => i -= steps,
            "D" => i += steps,
            _ => panic!("Should be a valid direction"),
        }
        res += steps;
        res2 += oi * j - oj * i;
    }
    1 + res / 2 + res2.abs() / 2
}

fn main() {
    let file_path = "data/day18_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        assert_eq!(part1(input), 62);
    }

    #[test]
    fn test_part2() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        assert_eq!(part2(input), 952408144115);
    }
}
