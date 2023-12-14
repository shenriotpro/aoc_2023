use std::{collections::HashMap, fs};

use aoc_2023::parse_grid;

fn part1(input: &str) -> i64 {
    let grid = parse_grid(input);

    let new_grid = move_north(&grid);
    compute_load(&new_grid)
}

fn part2(input: &str) -> i64 {
    let grid = parse_grid(input);

    let mut cache = HashMap::new();
    cache.insert(grid.clone(), 0);
    let mut new_grid = cycle(&grid);
    let mut i = 1i64;
    while !cache.contains_key(&new_grid) {
        cache.insert(new_grid.clone(), i);
        new_grid = cycle(&new_grid);
        i += 1;
    }

    let cycle_start = cache.get(&new_grid).expect("Should be in cache");
    let cycle_len = i - cycle_start;
    let remaining = (1000000000 - cycle_start) % cycle_len;

    for _ in 0..remaining {
        new_grid = cycle(&new_grid);
    }

    compute_load(&new_grid)
}

fn compute_load(grid: &Vec<Vec<char>>) -> i64 {
    let mut res = 0;
    for (j, _) in grid[0].iter().enumerate() {
        for (i, _) in grid.iter().enumerate() {
            match grid[i][j] {
                'O' => {
                    res += (grid.len() - i) as i64;
                }
                '#' => {}
                '.' => {}
                _ => panic!("Should be a valid cell"),
            }
        }
    }
    res
}

fn move_north(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_grid = grid.to_owned();

    for (j, _) in grid[0].iter().enumerate() {
        let mut destination = 0;
        for (i, _) in grid.iter().enumerate() {
            match grid[i][j] {
                'O' => {
                    if destination != i {
                        new_grid[destination][j] = 'O';
                        new_grid[i][j] = '.';
                    }
                    destination += 1;
                }
                '#' => {
                    new_grid[i][j] = '#';
                    destination = i + 1;
                }
                '.' => {
                    new_grid[i][j] = '.';
                }
                _ => panic!("Should be a valid cell"),
            }
        }
    }

    new_grid
}

fn cycle(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_grid = grid.to_owned();

    for (j, _) in grid[0].iter().enumerate() {
        let mut destination = 0;
        for (i, _) in grid.iter().enumerate() {
            match grid[i][j] {
                'O' => {
                    if destination != i {
                        new_grid[destination][j] = 'O';
                        new_grid[i][j] = '.';
                    }
                    destination += 1;
                }
                '#' => {
                    new_grid[i][j] = '#';
                    destination = i + 1;
                }
                '.' => {
                    new_grid[i][j] = '.';
                }
                _ => panic!("Should be a valid cell"),
            }
        }
    }

    let grid = new_grid.clone();
    for (i, _) in grid.iter().enumerate() {
        let mut destination = 0;
        for (j, _) in grid[0].iter().enumerate() {
            match grid[i][j] {
                'O' => {
                    if destination != j {
                        new_grid[i][destination] = 'O';
                        new_grid[i][j] = '.';
                    }
                    destination += 1;
                }
                '#' => {
                    new_grid[i][j] = '#';
                    destination = j + 1;
                }
                '.' => {
                    new_grid[i][j] = '.';
                }
                _ => panic!("Should be a valid cell"),
            }
        }
    }

    let grid = new_grid.clone();
    for (j, _) in grid[0].iter().enumerate() {
        let mut destination = grid.len() - 1;
        for (i, _) in grid.iter().enumerate().rev() {
            match grid[i][j] {
                'O' => {
                    if destination != i {
                        new_grid[destination][j] = 'O';
                        new_grid[i][j] = '.';
                    }
                    destination -= 1;
                }
                '#' => {
                    new_grid[i][j] = '#';
                    if i > 0 {
                        destination = i - 1;
                    }
                }
                '.' => {
                    new_grid[i][j] = '.';
                }
                _ => panic!("Should be a valid cell"),
            }
        }
    }

    let grid = new_grid.clone();
    for (i, _) in grid.iter().enumerate() {
        let mut destination = grid[0].len() - 1;
        for (j, _) in grid[0].iter().enumerate().rev() {
            match grid[i][j] {
                'O' => {
                    if destination != j {
                        new_grid[i][destination] = 'O';
                        new_grid[i][j] = '.';
                    }
                    destination -= 1;
                }
                '#' => {
                    new_grid[i][j] = '#';
                    if j > 0 {
                        destination = j - 1;
                    }
                }
                '.' => {
                    new_grid[i][j] = '.';
                }
                _ => panic!("Should be a valid cell"),
            }
        }
    }

    new_grid
}

fn main() {
    let file_path = "data/day14_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(part1(input), 136);
    }

    #[test]
    fn test_part2() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(part2(input), 64);
    }
}
