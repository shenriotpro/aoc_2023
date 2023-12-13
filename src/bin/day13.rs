use std::fs;

use aoc_2023::parse_grid;
use itertools::Itertools;

fn part1(input: &str) -> i64 {
    // TODO: careful about the last (empty) line
    let sections = input.split("\n\n");
    let grids = sections.map(parse_grid).collect_vec();

    grids.iter().map(reflection_summary).sum()
}

fn reflection_summary(grid: &Vec<Vec<char>>) -> i64 {
    let mut res = 0;

    for (i, _) in grid.iter().enumerate() {
        if i == grid.len() - 1 {
            break;
        }
        let mut k = 0i64;
        let mut sym = true;
        let i = i as i64;
        while i - k >= 0 && i + k + 1 < grid.len() as i64 {
            if grid[(i - k) as usize] != grid[(i + k + 1) as usize] {
                sym = false;
                break;
            }
            k += 1;
        }
        if sym {
            res += 100 * (i + 1);
        }
    }

    for (j, _) in grid[0].iter().enumerate() {
        if j == grid[0].len() - 1 {
            break;
        }
        let mut k = 0i64;
        let mut sym = true;
        let j = j as i64;
        while j - k >= 0 && j + k + 1 < grid[0].len() as i64 {
            if (0..=grid.len() - 1)
                .any(|i| grid[i][(j - k) as usize] != grid[i][(j + k + 1) as usize])
            {
                sym = false;
                break;
            }
            k += 1;
        }
        if sym {
            res += j + 1;
        }
    }

    res
}

fn reflection_summary2(grid: &Vec<Vec<char>>) -> i64 {
    let mut res = 0;

    for (i, _) in grid.iter().enumerate() {
        if i == grid.len() - 1 {
            break;
        }
        let mut k = 0i64;
        let mut sym = 0;
        let i = i as i64;
        while i - k >= 0 && i + k + 1 < grid.len() as i64 {
            for (j, _) in grid[0].iter().enumerate() {
                if grid[(i - k) as usize][j] != grid[(i + k + 1) as usize][j] {
                    sym += 1;
                }
            }
            k += 1;
        }
        if sym == 1 {
            res += 100 * (i + 1);
        }
    }

    for (j, _) in grid[0].iter().enumerate() {
        if j == grid[0].len() - 1 {
            break;
        }
        let mut k = 0i64;
        let mut sym = 0;
        let j = j as i64;
        while j - k >= 0 && j + k + 1 < grid[0].len() as i64 {
            for (i, _) in grid.iter().enumerate() {
                if grid[i][(j - k) as usize] != grid[i][(j + k + 1) as usize] {
                    sym += 1;
                }
            }
            k += 1;
        }
        if sym == 1 {
            res += j + 1;
        }
    }

    res
}

fn part2(input: &str) -> i64 {
    // TODO: careful about the last (empty) line
    let sections = input.split("\n\n");
    let grids = sections.map(parse_grid).collect_vec();

    grids.iter().map(reflection_summary2).sum()
}

fn main() {
    let file_path = "data/day13_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(part1(input), 405);
    }

    #[test]
    fn test_part2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(part2(input), 400);
    }
}
