use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

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

fn get_neighbors2(grid: &Vec<Vec<char>>, position: (i64, i64)) -> Vec<(i64, i64)> {
    let mut res = vec![];
    let (i, j) = position;
    let n = grid.len() as i64;
    let m = grid[0].len() as i64;
    for (neighbor, c) in [(i + 1, j), (i - 1, j), (i, j - 1), (i, j + 1)]
        .iter()
        .map(|(ni, nj)| {
            (
                (*ni, *nj),
                grid[ni.rem_euclid(n) as usize][nj.rem_euclid(m) as usize],
            )
        })
    {
        if c != '#' {
            res.push(neighbor);
        }
    }
    res
}

fn part2_small(input: &str, steps: i64) -> i64 {
    let grid = parse_grid(input);

    let start = grid_find(&grid, 'S').expect("Should have a start");
    let start = (start.0 as i64, start.1 as i64);
    let mut seen = HashSet::new();
    seen.reserve((steps * steps) as usize);
    let mut queue = VecDeque::new();
    queue.reserve((steps * steps) as usize);
    queue.push_back((start, 0));
    seen.insert(start);
    let mut res = 0;
    while !queue.is_empty() {
        let (position, d) = queue.pop_front().expect("Should have a position");
        if d % 2 == steps % 2 {
            res += 1;
        }
        if d > steps {
            break;
        }
        if d == steps {
            continue;
        }
        for neighbor in get_neighbors2(&grid, position) {
            if seen.contains(&neighbor) {
                continue;
            }
            seen.insert(neighbor);
            queue.push_back((neighbor, d + 1));
        }
    }

    res
}

fn part2(input: &str, steps: Option<i64>) -> i64 {
    let steps = steps.unwrap_or(26501365);
    // Unfortunately, we have to choose between slow and specialized.
    if steps <= 5000 {
        return part2_small(input, steps);
    }
    let grid = parse_grid(input);

    let start = grid_find(&grid, 'S').expect("Should have a start");
    let mut dist = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    dist.insert(start, 0);
    while !queue.is_empty() {
        let (position, d) = queue.pop_front().expect("Should have a position");
        for neighbor in get_neighbors(&grid, position) {
            if dist.contains_key(&neighbor) {
                continue;
            }
            dist.insert(neighbor, d + 1);
            queue.push_back((neighbor, d + 1));
        }
    }

    // https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    let odd = dist.values().filter(|d| *d % 2 == 1).count() as i64;
    let even = dist.values().filter(|d| *d % 2 == 0).count() as i64;
    let max = grid.len() / 2;
    let odd_corners = dist.values().filter(|d| *d % 2 == 1 && **d > max).count() as i64;
    let even_corners = dist.values().filter(|d| *d % 2 == 0 && **d > max).count() as i64;
    let n = steps / (grid.len() as i64);

    (n + 1) * (n + 1) * odd + n * n * even - (n + 1) * odd_corners + n * even_corners
}

fn main() {
    let file_path = "data/day21_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input, None));
    println!("{}", part2(&input, None));
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

        // TODO: improve perf?
        assert_eq!(part2(input, Some(6)), 16);
        assert_eq!(part2(input, Some(10)), 50);
        assert_eq!(part2(input, Some(50)), 1594);
        assert_eq!(part2(input, Some(100)), 6536);
        assert_eq!(part2(input, Some(500)), 167004);
        assert_eq!(part2(input, Some(1000)), 668697);
        assert_eq!(part2(input, Some(5000)), 16733044);
    }
}
