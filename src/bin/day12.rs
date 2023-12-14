use std::fs;

use cached::proc_macro::cached;
use itertools::Itertools;

struct Record {
    mask: String,
    groups: Vec<i64>,
}

fn part1(input: &str) -> i64 {
    let records = input.lines().map(parse_line).collect_vec();
    records
        .iter()
        .map(|record| combinations(record.mask.clone(), record.groups.clone()))
        .sum()
}

fn parse_line(line: &str) -> Record {
    let (mask, groups) = line
        .split_whitespace()
        .collect_tuple()
        .expect("Should have two parts");
    let mask = mask.to_string();
    let groups = groups
        .split(',')
        .map(|e| e.parse::<i64>().expect("Should be a number"))
        .collect_vec();
    Record { mask, groups }
}

#[cached]
fn combinations(mask: String, groups: Vec<i64>) -> i64 {
    match mask.find('?') {
        None => {
            if matches_exactly(&mask, &groups) {
                1
            } else {
                0
            }
        }
        Some(position) => {
            match mask
                .get(..position)
                .expect("Should be a valid slice")
                .rfind('.')
            {
                None => {
                    let mut res = 0;
                    for c in ['.', '#'] {
                        let mut new_mask = mask.to_owned();
                        let groups = groups.clone();
                        new_mask.replace_range(position..position + 1, &c.to_string());
                        if matches_partially(&new_mask, &groups) {
                            res += combinations(new_mask, groups);
                        }
                    }
                    res
                }
                Some(last) => {
                    let start_groups =
                        compute_groups(mask.get(..last).expect("Should be a valid slice"));
                    if groups.starts_with(&start_groups) {
                        combinations(
                            mask.get(last + 1..)
                                .expect("Should be a valid slice")
                                .to_string(),
                            groups[start_groups.len()..].to_vec(),
                        )
                    } else {
                        0
                    }
                }
            }
        }
    }
}

fn matches_partially(mask: &str, groups: &[i64]) -> bool {
    let mask_groups = compute_partial_groups(mask);
    if mask_groups.is_empty() {
        return true;
    }
    let mut found = 0;
    for g in groups {
        if mask_groups[found] == *g {
            found += 1;
            if found == mask_groups.len() {
                return true;
            }
        }
    }
    false
}

fn compute_partial_groups(mask: &str) -> Vec<i64> {
    let mut res = vec![];
    let mut current = 0;
    let mut valid = true;
    for c in mask.chars() {
        match c {
            '#' => current += 1,
            '.' => {
                if current > 0 && valid {
                    res.push(current);
                }
                current = 0;
                valid = true;
            }
            '?' => {
                current = 0;
                valid = false;
            }
            _ => panic!("Should be a valid mask"),
        }
    }
    if current > 0 && valid {
        res.push(current);
    }
    res
}

fn matches_exactly(mask: &str, groups: &[i64]) -> bool {
    let mask_groups = compute_groups(mask);
    mask_groups == groups
}

fn compute_groups(mask: &str) -> Vec<i64> {
    let mut res = vec![];
    let mut current = 0;
    for c in mask.chars() {
        match c {
            '#' => current += 1,
            '.' => {
                if current > 0 {
                    res.push(current);
                    current = 0;
                }
            }
            _ => panic!("Should be a filled mask"),
        }
    }
    if current > 0 {
        res.push(current);
    }
    res
}

fn part2(input: &str) -> i64 {
    let records = input.lines().map(parse_line2).collect_vec();
    records
        .iter()
        .map(|record| combinations(record.mask.clone(), record.groups.clone()))
        .sum()
}

fn parse_line2(line: &str) -> Record {
    let rec = parse_line(line);
    let mask5 = format!(
        "{}?{}?{}?{}?{}",
        rec.mask, rec.mask, rec.mask, rec.mask, rec.mask
    );
    let groups5 = [
        rec.groups.clone(),
        rec.groups.clone(),
        rec.groups.clone(),
        rec.groups.clone(),
        rec.groups,
    ]
    .iter()
    .flatten()
    .cloned()
    .collect_vec();
    Record {
        mask: mask5,
        groups: groups5,
    }
}

fn main() {
    let file_path = "data/day12_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(part1(input), 21);
    }

    #[test]
    fn test_part1_basic() {
        let input = ".??..??...?##. 1,1,3";

        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_part2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(part2(input), 525152);
    }
}
