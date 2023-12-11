use std::fs;

use aoc_2023::parse_grid;
use itertools::Itertools;

fn part1(input: &str) -> i64 {
    let grid = parse_grid(input);

    let empty_columns = (0..grid[0].len())
        .filter(|j| (0..grid.len()).map(|i| grid[i][*j]).all(|c| c != '#'))
        .collect_vec();
    let empty_lines = (0..grid.len())
        .filter(|i| (0..grid[0].len()).map(|j| grid[*i][j]).all(|c| c != '#'))
        .collect_vec();

    let grid = duplicate_columns(&grid, &empty_columns);
    let grid = duplicate_lines(&grid, &empty_lines);

    let stars = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|(i, j)| grid[*i][*j] == '#')
        .collect_vec();

    stars
        .iter()
        .combinations(2)
        .map(|v| {
            let (s1, s2) = v.iter().collect_tuple().expect("Should be valid pairs");
            distance(s1, s2)
        })
        .sum()
}

fn distance(s1: &(usize, usize), s2: &(usize, usize)) -> i64 {
    let (i1, j1) = s1;
    let (i2, j2) = s2;
    (*i1 as i64 - *i2 as i64).abs() + (*j1 as i64 - *j2 as i64).abs()
}

fn duplicate_columns(grid: &Vec<Vec<char>>, empty_columns: &[usize]) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![];
    for i in 0..grid.len() {
        res.push(vec![]);
        for j in 0..grid[0].len() {
            res[i].push(grid[i][j]);
            if empty_columns.contains(&j) {
                res[i].push(grid[i][j]);
            }
        }
    }
    res
}

fn duplicate_lines(grid: &[Vec<char>], empty_lines: &[usize]) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![];
    for (i, row) in grid.iter().enumerate() {
        res.push(row.clone());
        if empty_lines.contains(&i) {
            res.push(row.clone());
        }
    }
    res
}

fn part2(input: &str, expansion: Option<i64>) -> i64 {
    let expansion = expansion.unwrap_or(1000000);

    let grid = parse_grid(input);

    let stars = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|(i, j)| grid[*i][*j] == '#')
        .collect_vec();

    let dist = stars
        .iter()
        .combinations(2)
        .map(|v| {
            let (s1, s2) = v.iter().collect_tuple().expect("Should be valid pairs");
            distance(s1, s2)
        })
        .sum::<i64>();

    let dist_2 = part1(input);

    dist + (expansion - 1) * (dist_2 - dist)
}

fn main() {
    let file_path = "data/day11_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input, None));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(part1(input), 374);
    }

    #[test]
    fn test_part2_10() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(part2(input, Some(10)), 1030);
    }

    #[test]
    fn test_part2_100() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(part2(input, Some(100)), 8410);
    }
}
