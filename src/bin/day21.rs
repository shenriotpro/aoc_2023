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

fn part2(input: &str, steps: Option<i64>) -> i64 {
    let steps = steps.unwrap_or(64);
    let grid = parse_grid(input);

    let start = grid_find(&grid, 'S').expect("Should have a start");
    let start = (start.0 as i64, start.1 as i64);
    let mut positions = HashSet::new();
    positions.insert(start);

    for _ in 0..steps {
        positions = positions
            .iter()
            .flat_map(|p| get_neighbors2(&grid, *p))
            .collect();
    }

    positions.len() as i64
}

fn get_neighbors2(grid: &Vec<Vec<char>>, position: (i64, i64)) -> Vec<(i64, i64)> {
    let (i, j) = position;
    let n = grid.len() as i64;
    let m = grid[0].len() as i64;
    let mut res = vec![];
    for (neighbor, c) in [
        (
            (i + 1, j),
            grid[(i + 1).rem_euclid(n) as usize][j.rem_euclid(m) as usize],
        ),
        (
            (i - 1, j),
            grid[(i - 1).rem_euclid(n) as usize][j.rem_euclid(m) as usize],
        ),
        (
            (i, j - 1),
            grid[i.rem_euclid(n) as usize][(j - 1).rem_euclid(m) as usize],
        ),
        (
            (i, j + 1),
            grid[i.rem_euclid(n) as usize][(j + 1).rem_euclid(m) as usize],
        ),
    ]
    .iter()
    {
        if *c != '#' {
            res.push(*neighbor);
        }
    }
    res
}

fn main() {
    let file_path = "data/day21_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input, Some(64)));
    let mut r = 0;
    let mut r2 = 0;
    for i in 64..1000 {
        let rr = part2(&input, Some(i));
        let rr2 = rr - r;
        println!("{} {} {}", i, rr - r, rr2 - r2);
        r = rr;
        r2 = rr2;
    }
    // println!("{}", part2(&input, Some(26501365)));
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

        assert_eq!(part2(input, Some(6)), 16);
        assert_eq!(part2(input, Some(10)), 50);
        assert_eq!(part2(input, Some(50)), 1594);
        assert_eq!(part2(input, Some(100)), 6536);
        // assert_eq!(part2(input, Some(500)), 167004);
        // assert_eq!(part2(input, Some(1000)), 668697);
        // assert_eq!(part2(input, Some(5000)), 16733044);
    }
}
