use std::{collections::HashSet, fs};

use aoc_2023::{grid_down, grid_left, grid_right, grid_up, parse_grid};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(input: &str) -> i64 {
    let grid = parse_grid(input);

    let position = (0, 0);
    let direction = Direction::Right;
    let mut energized = HashSet::new();
    let mut cache = HashSet::new();
    dfs(&grid, position, direction, &mut energized, &mut cache);

    energized.len() as i64
}

fn part2(input: &str) -> i64 {
    let grid = parse_grid(input);

    let left = (0..grid.len())
        .map(|i| energize(&grid, (i, 0), Direction::Right))
        .max()
        .expect("Should have a max");
    let right = (0..grid.len())
        .map(|i| energize(&grid, (i, grid[0].len() - 1), Direction::Left))
        .max()
        .expect("Should have a max");
    let top = (0..grid[0].len())
        .map(|j| energize(&grid, (0, j), Direction::Down))
        .max()
        .expect("Should have a max");
    let bottom = (0..grid[0].len())
        .map(|j| energize(&grid, (grid.len() - 1, j), Direction::Up))
        .max()
        .expect("Should have a max");

    *[left, right, top, bottom]
        .iter()
        .max()
        .expect("Should have a max")
}

fn dfs(
    grid: &Vec<Vec<char>>,
    position: (usize, usize),
    direction: Direction,
    energized: &mut HashSet<(usize, usize)>,
    cache: &mut HashSet<((usize, usize), Direction)>,
) {
    if cache.contains(&(position, direction)) {
        return;
    }

    cache.insert((position, direction));
    energized.insert(position);
    for (new_position, new_direction) in next(grid, position, direction) {
        dfs(grid, new_position, new_direction, energized, cache);
    }
}

fn next(
    grid: &Vec<Vec<char>>,
    position: (usize, usize),
    direction: Direction,
) -> Vec<((usize, usize), Direction)> {
    let c = grid[position.0][position.1];
    let neighbors = match (c, direction) {
        ('.', Direction::Down) => vec![(grid_down(grid, position), Direction::Down)],
        ('.', Direction::Up) => vec![(grid_up(grid, position), Direction::Up)],
        ('.', Direction::Left) => vec![(grid_left(grid, position), Direction::Left)],
        ('.', Direction::Right) => vec![(grid_right(grid, position), Direction::Right)],
        ('-', Direction::Down | Direction::Up) => vec![
            (grid_left(grid, position), Direction::Left),
            (grid_right(grid, position), Direction::Right),
        ],
        ('-', Direction::Left) => vec![(grid_left(grid, position), Direction::Left)],
        ('-', Direction::Right) => vec![(grid_right(grid, position), Direction::Right)],
        ('|', Direction::Down) => vec![(grid_down(grid, position), Direction::Down)],
        ('|', Direction::Up) => vec![(grid_up(grid, position), Direction::Up)],
        ('|', Direction::Left | Direction::Right) => vec![
            (grid_up(grid, position), Direction::Up),
            (grid_down(grid, position), Direction::Down),
        ],
        ('/', Direction::Down) => vec![(grid_left(grid, position), Direction::Left)],
        ('/', Direction::Up) => vec![(grid_right(grid, position), Direction::Right)],
        ('/', Direction::Left) => vec![(grid_down(grid, position), Direction::Down)],
        ('/', Direction::Right) => vec![(grid_up(grid, position), Direction::Up)],
        ('\\', Direction::Down) => vec![(grid_right(grid, position), Direction::Right)],
        ('\\', Direction::Up) => vec![(grid_left(grid, position), Direction::Left)],
        ('\\', Direction::Left) => vec![(grid_up(grid, position), Direction::Up)],
        ('\\', Direction::Right) => vec![(grid_down(grid, position), Direction::Down)],
        _ => panic!("Should be a valid cell"),
    };
    neighbors
        .iter()
        .filter(|(n, _)| n.is_some())
        .map(|(n, d)| (n.expect("Should be a valid neighbor").0, *d))
        .collect()
}

fn energize(grid: &Vec<Vec<char>>, position: (usize, usize), direction: Direction) -> i64 {
    let mut energized = HashSet::new();
    let mut cache = HashSet::new();
    dfs(grid, position, direction, &mut energized, &mut cache);
    energized.len() as i64
}

fn main() {
    let file_path = "data/day16_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        assert_eq!(part1(input), 46);
    }

    #[test]
    fn test_part2() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        assert_eq!(part2(input), 51);
    }
}
