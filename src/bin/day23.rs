use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

use aoc_2023::{grid_down, grid_left, grid_right, grid_up, parse_grid};

fn part1(input: &str) -> i64 {
    let grid = parse_grid(input);

    let start = (0usize, 1usize);
    let goal = (grid.len() - 1, grid[0].len() - 2);
    let mut seen = HashSet::new();

    backtrack(&grid, start, goal, &mut seen).expect("Should be a valid path")
}

fn backtrack(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    seen: &mut HashSet<(usize, usize)>,
) -> Option<i64> {
    if start == end {
        return Some(0);
    }
    let neighbors = get_neighbors(grid, start);
    if neighbors.is_empty() {
        return None;
    }
    let mut res = None;
    for (neighbor, dd) in neighbors {
        if !seen.contains(&neighbor) {
            seen.insert(neighbor);
            let d = backtrack(grid, neighbor, end, seen);
            if let Some(d) = d {
                res = match res {
                    None => Some(d + dd),
                    Some(res) => Some(res.max(d + dd)),
                };
            }
            seen.remove(&neighbor);
        }
    }
    res
}

fn get_neighbors(grid: &Vec<Vec<char>>, position: (usize, usize)) -> Vec<((usize, usize), i64)> {
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
        let (i, j) = *neighbor;
        match c {
            '>' => res.push(((i, j + 1), 2)),
            '<' => res.push(((i, j - 1), 2)),
            '^' => res.push(((i - 1, j), 2)),
            'v' => res.push(((i + 1, j), 2)),
            '.' => res.push((*neighbor, 1)),
            '#' => (),
            _ => panic!("Unexpected character"),
        }
    }
    res
}

fn get_next(
    grid: &Vec<Vec<char>>,
    prev: (usize, usize),
    position: (usize, usize),
) -> Option<(usize, usize)> {
    let neighbors = get_neighbors(grid, position);
    if neighbors.len() != 2 {
        return None;
    }
    if neighbors[0].0 == prev {
        Some(neighbors[1].0)
    } else {
        Some(neighbors[0].0)
    }
}

fn get_far_neighbor(
    grid: &Vec<Vec<char>>,
    position: (usize, usize),
    direction: (usize, usize),
) -> ((usize, usize), i64) {
    let mut prev = position;
    let mut cur = direction;
    let mut len = 1;
    while cur != position {
        let maybe_next = get_next(grid, prev, cur);
        if let Some(next) = maybe_next {
            prev = cur;
            cur = next;
            len += 1;
        } else {
            return (cur, len);
        }
    }
    (position, 0)
}

fn get_far_neighbors(
    grid: &Vec<Vec<char>>,
    position: (usize, usize),
) -> Vec<((usize, usize), i64)> {
    let mut res = vec![];
    for neighbor in get_neighbors(grid, position) {
        let far_neighbor = get_far_neighbor(grid, position, neighbor.0);
        if far_neighbor.0 != position {
            res.push(far_neighbor);
        }
    }
    res
}

type Neighbors = Vec<((usize, usize), i64)>;

fn backtrack2(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    seen: &mut HashSet<(usize, usize)>,
    cache: &mut HashMap<(usize, usize), Neighbors>,
) -> Option<i64> {
    if start == end {
        return Some(0);
    }
    let neighbors = if let Some(neighbors) = cache.get(&start) {
        neighbors.clone()
    } else {
        let neighbors = get_far_neighbors(grid, start);
        cache.insert(start, neighbors.clone());
        neighbors
    };
    if neighbors.is_empty() {
        return None;
    }
    let mut res = None;
    for (neighbor, dd) in neighbors {
        if !seen.contains(&neighbor) {
            seen.insert(neighbor);
            let d = backtrack2(grid, neighbor, end, seen, cache);
            if let Some(d) = d {
                res = match res {
                    None => Some(d + dd),
                    Some(res) => Some(res.max(d + dd)),
                };
            }
            seen.remove(&neighbor);
        }
    }
    res
}

fn part2(input: &str) -> i64 {
    let grid = parse_grid(&input.replace(['>', '<', '^', 'v'], "."));

    let start = (0usize, 1usize);
    let goal = (grid.len() - 1, grid[0].len() - 2);
    let mut seen = HashSet::new();
    let mut cache = HashMap::new();

    backtrack2(&grid, start, goal, &mut seen, &mut cache).expect("Should be a valid path")
}

fn main() {
    let file_path = "data/day23_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

        assert_eq!(part1(input), 94);
    }

    #[test]
    fn test_part2() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

        assert_eq!(part2(input), 154);
    }
}
