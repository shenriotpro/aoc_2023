use std::{collections::HashSet, fs};

use aoc_2023::{grid_down, grid_find, grid_left, grid_right, grid_up, parse_grid};

fn part1(input: &str, steps: Option<i64>) -> i64 {
    let steps = steps.unwrap_or(64);
    let grid = parse_grid(input);

    let start = grid_find(&grid, 'S').expect("Should have a start");
    let mut positions = HashSet::new();
    positions.insert(start);

    for _ in 0..steps {
        positions = positions
            .iter()
            .flat_map(|p| get_neighbors(&grid, *p))
            .collect();
    }

    positions.len() as i64
}

fn get_neighbors(grid: &Vec<Vec<char>>, position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for (neighbor, c) in [
        grid_down(grid, position),
        grid_up(grid, position),
        grid_left(grid, position),
        grid_right(grid, position),
    ]
    .iter()
    .flatten()
    {
        if *c != '#' {
            res.push(*neighbor);
        }
    }
    res
}

fn part2(input: &str) -> String {
    let mut res = String::new();
    for line in input.lines() {}
    res
}

fn main() {
    let file_path = "data/day21_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input, Some(64)));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        assert_eq!(part1(input, Some(6)), 16);
    }

    #[test]
    fn test_part2() {
        let input = "
";

        assert_eq!(part2(input), "");
    }
}
