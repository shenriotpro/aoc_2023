use std::fs;

use aoc_2023::split_parse;
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct Hail {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Hail {
    fn from(line: &str) -> Hail {
        let ints = split_parse(&line.replace(',', ""));
        let (x, y, z, vx, vy, vz) = (ints[0], ints[1], ints[2], ints[3], ints[4], ints[5]);
        Hail {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        }
    }
}

fn part1(input: &str, bounds: Option<(i64, i64)>) -> i64 {
    let bounds = bounds.unwrap_or((200000000000000, 400000000000000));
    let hails = input.lines().map(Hail::from).collect::<Vec<_>>();

    hails
        .iter()
        .combinations(2)
        .filter(|c| intersect(*c[0], *c[1], bounds))
        .count() as i64
}

fn intersect(h1: Hail, h2: Hail, bounds: (i64, i64)) -> bool {
    // Exact check for parallels.
    if h1.vx * h2.vy == h1.vy * h2.vx {
        return false;
    }
    let (a1, b1) = compute_ab(h1);
    let (a2, b2) = compute_ab(h2);
    let x = (b2 - b1) / (a1 - a2);
    if (x < (h1.x as f64) && h1.vx > 0) || (x > (h1.x as f64) && h1.vx < 0) {
        return false;
    }
    if (x < (h2.x as f64) && h2.vx > 0) || (x > (h2.x as f64) && h2.vx < 0) {
        return false;
    }
    let y = a1 * x + b1;
    let min = bounds.0 as f64;
    let max = bounds.1 as f64;
    let r = min <= x && x <= max && min <= y && y <= max;
    // if r {
    //     println!("{:?} {:?}", h1, h2);
    // }
    r
}

fn compute_ab(h: Hail) -> (f64, f64) {
    // TODO: check for 0s
    let a = (h.vy as f64) / (h.vx as f64);
    let b = (h.y as f64) - a * (h.x as f64);
    (a, b)
}

fn part2(input: &str) -> i64 {
    0
}

fn main() {
    let file_path = "data/day24_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input, None));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

        assert_eq!(part1(input, Some((7, 27))), 2);
    }

    #[test]
    fn test_part2() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

        assert_eq!(part2(input), 47);
    }
}
