use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use itertools::Itertools;

fn part1(input: &str) -> i64 {
    let grid = parse_grid(input);

    let start = grid_find(&grid, 'S').expect("Should have a start");
    let mut queue = VecDeque::<(usize, usize)>::new();
    let mut dist = HashMap::<(usize, usize), i64>::new();
    dist.insert(start, 0);
    if let Some(((ni, nj), c)) = grid_down(&grid, start) {
        if ['|', 'L', 'J'].contains(&c) {
            queue.push_back((ni, nj));
            dist.insert((ni, nj), 1);
        }
    }
    if let Some(((ni, nj), c)) = grid_up(&grid, start) {
        if ['|', '7', 'F'].contains(&c) {
            queue.push_back((ni, nj));
            dist.insert((ni, nj), 1);
        }
    }
    if let Some(((ni, nj), c)) = grid_left(&grid, start) {
        if ['-', 'L', 'F'].contains(&c) {
            queue.push_back((ni, nj));
            dist.insert((ni, nj), 1);
        }
    }
    if let Some(((ni, nj), c)) = grid_right(&grid, start) {
        if ['-', 'J', '7'].contains(&c) {
            queue.push_back((ni, nj));
            dist.insert((ni, nj), 1);
        }
    }

    while !queue.is_empty() {
        let (i, j) = queue.pop_front().expect("Queue should not be empty");
        let d = *dist.get(&(i, j)).expect("Should have a distance");
        let neighbors = match grid[i][j] {
            '|' => vec![grid_down(&grid, (i, j)), grid_up(&grid, (i, j))],
            '-' => vec![grid_right(&grid, (i, j)), grid_left(&grid, (i, j))],
            'L' => vec![grid_up(&grid, (i, j)), grid_right(&grid, (i, j))],
            'J' => vec![grid_left(&grid, (i, j)), grid_up(&grid, (i, j))],
            '7' => vec![grid_down(&grid, (i, j)), grid_left(&grid, (i, j))],
            'F' => vec![grid_right(&grid, (i, j)), grid_down(&grid, (i, j))],
            _ => panic!("Should be a valid pipe"),
        };
        let neighbors = neighbors
            .iter()
            .filter_map(|e| *e)
            .filter(|((ni, nj), _)| !dist.contains_key(&(*ni, *nj)))
            .collect_vec();
        for ((ni, nj), _) in neighbors {
            queue.push_back((ni, nj));
            dist.insert((ni, nj), d + 1);
        }
    }

    *dist.values().max().expect("Should have nodes")
}

fn part2(input: &str) -> i64 {
    let mut grid = parse_grid(input);

    let start = grid_find(&grid, 'S').expect("Should have a start");
    let mut queue = VecDeque::<(usize, usize)>::new();
    let mut dist = HashMap::<(usize, usize), i64>::new();
    dist.insert(start, 0);
    let mut start_neighbors = vec![];
    if let Some(((ni, nj), c)) = grid_down(&grid, start) {
        if ['|', 'L', 'J'].contains(&c) {
            queue.push_back((ni, nj));
            dist.insert((ni, nj), 1);
            start_neighbors.push('d');
        }
    }
    if let Some(((ni, nj), c)) = grid_up(&grid, start) {
        if ['|', '7', 'F'].contains(&c) {
            queue.push_back((ni, nj));
            dist.insert((ni, nj), 1);
            start_neighbors.push('u');
        }
    }
    if let Some(((ni, nj), c)) = grid_left(&grid, start) {
        if ['-', 'L', 'F'].contains(&c) {
            queue.push_back((ni, nj));
            dist.insert((ni, nj), 1);
            start_neighbors.push('l');
        }
    }
    if let Some(((ni, nj), c)) = grid_right(&grid, start) {
        if ['-', 'J', '7'].contains(&c) {
            queue.push_back((ni, nj));
            dist.insert((ni, nj), 1);
            start_neighbors.push('r');
        }
    }

    start_neighbors.sort();
    let behind_s = match start_neighbors
        .iter()
        .collect_tuple()
        .expect("Should have 2 neighbors")
    {
        ('d', 'l') => '7',
        ('d', 'r') => 'F',
        ('d', 'u') => '|',
        ('l', 'r') => '-',
        ('l', 'u') => 'J',
        ('r', 'u') => 'L',
        _ => panic!("Should be a valid start"),
    };
    grid[start.0][start.1] = behind_s;

    while !queue.is_empty() {
        let (i, j) = queue.pop_front().expect("Queue should not be empty");
        let d = *dist.get(&(i, j)).expect("Should have a distance");
        let neighbors = match grid[i][j] {
            '|' => vec![grid_down(&grid, (i, j)), grid_up(&grid, (i, j))],
            '-' => vec![grid_right(&grid, (i, j)), grid_left(&grid, (i, j))],
            'L' => vec![grid_up(&grid, (i, j)), grid_right(&grid, (i, j))],
            'J' => vec![grid_left(&grid, (i, j)), grid_up(&grid, (i, j))],
            '7' => vec![grid_down(&grid, (i, j)), grid_left(&grid, (i, j))],
            'F' => vec![grid_right(&grid, (i, j)), grid_down(&grid, (i, j))],
            _ => panic!("Should be a valid pipe"),
        };
        let neighbors = neighbors
            .iter()
            .filter_map(|e| *e)
            .filter(|((ni, nj), _)| !dist.contains_key(&(*ni, *nj)))
            .collect_vec();
        for ((ni, nj), _) in neighbors {
            queue.push_back((ni, nj));
            dist.insert((ni, nj), d + 1);
        }
    }

    let mut res = 0;
    let mut left_crossings = HashMap::<(usize, usize), (i64, char)>::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let new_crossings = match j {
                0 => (0, '|'),
                _ => {
                    let ((li, lj), lc) =
                        grid_left(&grid, (i, j)).expect("Should be a valid neighbor");
                    let (prev_crossings, prev_open) = *left_crossings
                        .get(&(li, lj))
                        .expect("Should be a valid neighbor");
                    if !dist.contains_key(&(li, lj)) {
                        (prev_crossings, prev_open)
                    } else {
                        update_crossings(prev_crossings, prev_open, lc)
                    }
                }
            };
            left_crossings.insert((i, j), new_crossings);
            if !dist.contains_key(&(i, j)) && new_crossings.0 % 2 == 1 {
                res += 1;
            }
        }
    }

    res
}

fn update_crossings(prev_crossings: i64, prev_open: char, lc: char) -> (i64, char) {
    match (prev_open, lc) {
        (_, '|') => (prev_crossings + 1, '|'),
        ('L', '7') => (prev_crossings + 1, '|'),
        ('F', 'J') => (prev_crossings + 1, '|'),
        (_, 'L') => (prev_crossings, 'L'),
        (_, 'F') => (prev_crossings, 'F'),
        _ => (prev_crossings, prev_open),
    }
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn grid_find<T: PartialEq>(grid: &Vec<Vec<T>>, hay: T) -> Option<(usize, usize)> {
    (0..grid.len())
        .flat_map(|i| (0..grid[0].len()).map(move |j| (i, j)))
        .find(|(i, j)| grid[*i][*j] == hay)
}

fn grid_down<T: PartialEq + Copy>(
    grid: &Vec<Vec<T>>,
    start: (usize, usize),
) -> Option<((usize, usize), T)> {
    let (i, j) = start;
    let i = i as i64;
    let j = j as i64;
    let n = grid.len() as i64;
    let m = grid[0].len() as i64;
    let ni = i + 1;
    let nj = j;
    if ni >= 0 && ni < n && nj >= 0 && nj < m {
        Some(((ni as usize, nj as usize), grid[ni as usize][nj as usize]))
    } else {
        None
    }
}

fn grid_up<T: PartialEq + Copy>(
    grid: &Vec<Vec<T>>,
    start: (usize, usize),
) -> Option<((usize, usize), T)> {
    let (i, j) = start;
    let i = i as i64;
    let j = j as i64;
    let n = grid.len() as i64;
    let m = grid[0].len() as i64;
    let ni = i - 1;
    let nj = j;
    if ni >= 0 && ni < n && nj >= 0 && nj < m {
        Some(((ni as usize, nj as usize), grid[ni as usize][nj as usize]))
    } else {
        None
    }
}

fn grid_left<T: PartialEq + Copy>(
    grid: &Vec<Vec<T>>,
    start: (usize, usize),
) -> Option<((usize, usize), T)> {
    let (i, j) = start;
    let i = i as i64;
    let j = j as i64;
    let n = grid.len() as i64;
    let m = grid[0].len() as i64;
    let ni = i;
    let nj = j - 1;
    if ni >= 0 && ni < n && nj >= 0 && nj < m {
        Some(((ni as usize, nj as usize), grid[ni as usize][nj as usize]))
    } else {
        None
    }
}

fn grid_right<T: PartialEq + Copy>(
    grid: &Vec<Vec<T>>,
    start: (usize, usize),
) -> Option<((usize, usize), T)> {
    let (i, j) = start;
    let i = i as i64;
    let j = j as i64;
    let n = grid.len() as i64;
    let m = grid[0].len() as i64;
    let ni = i;
    let nj = j + 1;
    if ni >= 0 && ni < n && nj >= 0 && nj < m {
        Some(((ni as usize, nj as usize), grid[ni as usize][nj as usize]))
    } else {
        None
    }
}

fn main() {
    let file_path = "data/day10_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_part1_alt() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(part2(input), 4);
    }

    #[test]
    fn test_part2_alt() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        assert_eq!(part2(input), 8);
    }

    #[test]
    fn test_part2_alt_alt() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(part2(input), 10);
    }
}
