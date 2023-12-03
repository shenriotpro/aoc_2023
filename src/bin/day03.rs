use std::fs;

use itertools::Itertools;

fn part1(input: &str) -> i64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    grid.iter()
        .enumerate()
        .map(|(i, line)| line_sum(i, line, &grid))
        .sum()
}

fn line_sum(i: usize, line: &[char], grid: &Vec<Vec<char>>) -> i64 {
    let mut res = 0;
    let mut cur_part = 0;
    let mut cur_valid = false;
    for (j, c) in line.iter().enumerate() {
        if c.is_ascii_digit() {
            cur_part *= 10;
            cur_part += c.to_digit(10).expect("Should be a valid digit") as i64;
            if !cur_valid
                && neighbors(i as i32, j as i32, grid)
                    .iter()
                    .any(|c| *c != '.' && !c.is_ascii_digit())
            {
                cur_valid = true;
            }
        } else {
            if cur_valid {
                res += cur_part;
            }
            cur_part = 0;
            cur_valid = false;
        }
    }
    // Let's not forget the last part.
    if cur_valid {
        res += cur_part;
    }
    res
}

fn neighbors<T: Copy>(i: i32, j: i32, grid: &Vec<Vec<T>>) -> Vec<T> {
    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    deltas
        .iter()
        .filter(|(di, dj)| {
            i + di >= 0
                && i + di < (grid.len() as i32)
                && j + dj >= 0
                && j + dj < (grid[0].len() as i32)
        })
        .map(|delta| {
            let (di, dj) = delta;
            grid[(i + di) as usize][(j + dj) as usize]
        })
        .collect::<Vec<_>>()
}

fn part2(input: &str) -> i64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (numbers, numbers_grid) = find_numbers(&grid);
    grid.iter()
        .enumerate()
        .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, c)| (i, j, *c)))
        .filter(|(_, _, c)| *c == '*')
        .map(|(i, j, _)| {
            neighbors(i as i32, j as i32, &numbers_grid)
                .iter()
                .filter_map(|n| *n)
                .unique()
                .collect_vec()
        })
        .filter(|neis| neis.len() == 2)
        .map(|neis| numbers[neis[0]] * numbers[neis[1]])
        .sum()
}

fn find_numbers(grid: &Vec<Vec<char>>) -> (Vec<i64>, Vec<Vec<Option<usize>>>) {
    let mut numbers = vec![];
    let mut numbers_grid = vec![];
    let mut in_number = false;
    for line in grid {
        numbers_grid.push(vec![]);
        for c in line {
            if c.is_ascii_digit() {
                let d = c.to_digit(10).expect("Should be a valid digit") as i64;
                if in_number {
                    let number = numbers.last_mut().expect("Should have a number");
                    *number *= 10;
                    *number += d;
                } else {
                    in_number = true;
                    numbers.push(d);
                }
                numbers_grid
                    .last_mut()
                    .expect("Should have a numbers line")
                    .push(Some(numbers.len() - 1));
            } else {
                in_number = false;
                numbers_grid
                    .last_mut()
                    .expect("Should have a numbers line")
                    .push(None);
            }
        }
        in_number = false;
    }
    (numbers, numbers_grid)
}

fn main() {
    let file_path = "data/day03_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part2(input), 467835);
    }
}
