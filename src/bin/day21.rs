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

fn part2(input: &str, steps: Option<i64>) -> i64 {
    let steps = steps.unwrap_or(26501365);
    let grid = parse_grid(input);

    let start = grid_find(&grid, 'S').expect("Should have a start");
    let start = (start.0 as i64, start.1 as i64);
    let mut dist = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    dist.insert(start, 0);
    let n = grid.len() as i64;
    let m = grid[0].len() as i64;
    let max = 6 * n.max(m);
    loop {
        let (position, d) = queue.pop_front().expect("Should have a position");
        // let (i, j) = position;
        // if i.rem_euclid(n) == 2 && j.rem_euclid(m) == 4 {
        //     println!("{:?} {}", position, d);
        // }
        if d > max {
            break;
        }
        for neighbor in get_neighbors2(&grid, position) {
            if dist.contains_key(&neighbor) {
                continue;
            }
            dist.insert(neighbor, d + 1);
            queue.push_back((neighbor, d + 1));
        }
    }

    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if dist.contains_key(&(i, j)) {
                let mut d = dist[&(i, j)];
                if d <= steps && d % 2 == steps % 2 {
                    res += 1;
                }
                let mut nj = j - m;
                let mut nd = dist[&(i, nj)];
                while nd <= steps && nd - d != m {
                    d = nd;
                    if d % 2 == steps % 2 {
                        res += 1;
                    }
                    nj -= m;
                    nd = dist[&(i, nj)];
                }
                while nd <= steps {
                    d = nd;
                    if d % 2 == steps % 2 {
                        res += 1;
                    }
                    nd += m;
                }

                let mut d = dist[&(i, j)];
                let mut nj = j + m;
                let mut nd = dist[&(i, nj)];
                while nd <= steps && nd - d != m {
                    d = nd;
                    if d % 2 == steps % 2 {
                        res += 1;
                    }
                    nj += m;
                    nd = dist[&(i, nj)];
                }
                while nd <= steps {
                    d = nd;
                    if d % 2 == steps % 2 {
                        res += 1;
                    }
                    nd += m;
                }
            }
        }
    }

    res
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

        assert_eq!(part2(input, Some(6)), 16);
        assert_eq!(part2(input, Some(10)), 50);
        assert_eq!(part2(input, Some(50)), 1594);
        // assert_eq!(part2(input, Some(100)), 6536);
        // assert_eq!(part2(input, Some(500)), 167004);
        // assert_eq!(part2(input, Some(1000)), 668697);
        // assert_eq!(part2(input, Some(5000)), 16733044);
    }
}
