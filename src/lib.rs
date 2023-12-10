use std::str::FromStr;

/// Split a string and return values that can be parsed.
/// Note that the wrong type may result in silent failures.
pub fn split_parse<T: FromStr>(s: &str) -> Vec<T> {
    s.split_whitespace()
        .map(|s| s.parse())
        .filter_map(Result::ok)
        .collect()
}

// TODO: refactor Grid
/// Parse a grid of characters into vectors.
pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

/// Find an element in a grid.
pub fn grid_find<T: PartialEq>(grid: &Vec<Vec<T>>, hay: T) -> Option<(usize, usize)> {
    (0..grid.len())
        .flat_map(|i| (0..grid[0].len()).map(move |j| (i, j)))
        .find(|(i, j)| grid[*i][*j] == hay)
}

/// Get the element below in a grid.
pub fn grid_down<T: PartialEq + Copy>(
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

/// Get the element above in a grid.
pub fn grid_up<T: PartialEq + Copy>(
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

/// Get the element to the left in a grid.
pub fn grid_left<T: PartialEq + Copy>(
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

/// Get the element to the right in a grid.
pub fn grid_right<T: PartialEq + Copy>(
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

#[cfg(test)]
mod tests {
    use super::{grid_down, grid_find, grid_left, grid_right, grid_up, parse_grid, split_parse};

    #[test]
    fn test_split_parse_empty() {
        let input = "";

        assert_eq!(split_parse::<i64>(input), vec![]);
    }

    #[test]
    fn test_split_parse_trivial() {
        let input = "42";

        assert_eq!(split_parse::<String>(input), vec!["42"]);
    }

    #[test]
    fn test_split_parse_int() {
        let input = "Time:      7  15   30";

        assert_eq!(split_parse::<i64>(input), vec![7, 15, 30]);
    }

    #[test]
    fn test_split_parse_bool() {
        let input = "Time:      true false";

        assert_eq!(split_parse::<bool>(input), vec![true, false]);
    }

    #[test]
    fn test_char_grid() {
        let input = "abcdefg
hijklmn
opqrstu";

        let grid = parse_grid(input);
        assert_eq!(
            grid,
            vec![
                vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
                vec!['h', 'i', 'j', 'k', 'l', 'm', 'n'],
                vec!['o', 'p', 'q', 'r', 's', 't', 'u'],
            ]
        );

        let a = grid_find(&grid, 'a').expect("Should have found a");
        assert_eq!(a, (0, 0));

        assert_eq!(grid_up(&grid, a), None);
        assert_eq!(grid_left(&grid, a), None);
        assert_eq!(grid_right(&grid, a), Some(((0, 1), 'b')));
        assert_eq!(grid_down(&grid, a), Some(((1, 0), 'h')));

        let u = grid_find(&grid, 'u').expect("Should have found u");
        assert_eq!(u, (2, 6));

        assert_eq!(grid_down(&grid, u), None);
        assert_eq!(grid_right(&grid, u), None);
        assert_eq!(grid_left(&grid, u), Some(((2, 5), 't')));
        assert_eq!(grid_up(&grid, u), Some(((1, 6), 'n')));

        assert_eq!(grid_find(&grid, 'z'), None);
    }
}
