use std::{fs, str::FromStr};

use aoc_2023::split_parse;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SEED2_RE: Regex = Regex::new(r"(\d+) (\d+)").expect("Should be a valid regex");
    static ref ENTRY_RE: Regex = Regex::new(r"(?<destination>\d+) (?<source>\d+) (?<length>\d+)")
        .expect("Should be a valid regex");
}

#[derive(Debug)]
struct Seeds1 {
    data: Vec<i64>,
}

#[derive(Debug)]
struct Map {
    data: Vec<Entry>,
}

impl Map {
    fn apply(&self, source: i64) -> i64 {
        for entry in &self.data {
            if source >= entry.source && source < entry.source + entry.length {
                let delta = source - entry.source;
                return entry.destination + delta;
            }
        }
        source
    }
}

#[derive(Debug)]
struct Entry {
    destination: i64,
    source: i64,
    length: i64,
}

fn part1(input: &str) -> i64 {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let seeds = sections[0]
        .parse::<Seeds1>()
        .expect("Should be able to parse seeds");
    let maps = sections[1..]
        .iter()
        .map(|section| section.parse::<Map>().expect("Should be able to parse map"))
        .collect::<Vec<_>>();

    seeds
        .data
        .iter()
        .cloned()
        .map(|x| maps.iter().fold(x, |acc, m| m.apply(acc)))
        .min()
        .expect("Should have a value")
}

#[derive(Debug)]
struct Seeds2 {
    data: Vec<(i64, i64)>,
}

fn part2(input: &str) -> i64 {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let seeds = sections[0]
        .parse::<Seeds2>()
        .expect("Should be able to parse seeds");
    let maps = sections[1..]
        .iter()
        .map(|section| section.parse::<Map>().expect("Should be able to parse map"))
        .collect::<Vec<_>>();

    let mut bounds = seeds
        .data
        .iter()
        .flat_map(|(b, n)| [*b, *b + *n])
        .collect_vec();

    for (i, map) in maps.iter().enumerate() {
        let new_bounds = map
            .data
            .iter()
            .flat_map(|e| [e.source, e.source + e.length])
            .filter(|x| is_reachable(*x, &seeds, &maps[..i]))
            .collect_vec();
        bounds.extend(new_bounds);
        bounds = bounds.iter().map(|x| map.apply(*x)).collect_vec();
    }

    *bounds.iter().min().expect("Should have a result")
}

fn is_reachable(goal: i64, seeds: &Seeds2, maps: &[Map]) -> bool {
    if maps.is_empty() {
        seeds.data.iter().any(|(b, n)| goal >= *b && goal < *b + *n)
    } else {
        let map = maps.last().expect("Should have a map");
        let mut sources = map
            .data
            .iter()
            .filter(|e| goal >= e.destination && goal < e.destination + e.length)
            .map(|e| goal - (e.destination - e.source))
            .collect_vec();
        if sources.is_empty() {
            sources = vec![goal];
        }
        sources
            .iter()
            .any(|s| is_reachable(*s, seeds, &maps[..maps.len() - 1]))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSeeds1Error;

impl FromStr for Seeds1 {
    type Err = ParseSeeds1Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = split_parse(s);

        Ok(Seeds1 { data })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSeeds2Error;

impl FromStr for Seeds2 {
    type Err = ParseSeeds2Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = SEED2_RE.captures_iter(s).map(|c| c.extract());

        let mut data = vec![];
        for (_, [source, length]) in caps {
            let source = source.parse::<i64>().map_err(|_| ParseSeeds2Error)?;
            let length = length.parse::<i64>().map_err(|_| ParseSeeds2Error)?;
            data.push((source, length));
        }

        Ok(Seeds2 { data })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMapError;

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        // Skip the first line.
        lines.next();

        let mut data = vec![];
        for line in lines {
            data.push(line.parse::<Entry>().map_err(|_| ParseMapError)?);
        }

        Ok(Map { data })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseEntryError;

impl FromStr for Entry {
    type Err = ParseEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = ENTRY_RE.captures(s).ok_or(ParseEntryError)?;

        let destination = caps
            .name("destination")
            .ok_or(ParseEntryError)?
            .as_str()
            .parse::<i64>()
            .map_err(|_| ParseEntryError)?;
        let source = caps
            .name("source")
            .ok_or(ParseEntryError)?
            .as_str()
            .parse::<i64>()
            .map_err(|_| ParseEntryError)?;
        let length = caps
            .name("length")
            .ok_or(ParseEntryError)?
            .as_str()
            .parse::<i64>()
            .map_err(|_| ParseEntryError)?;

        Ok(Entry {
            destination,
            source,
            length,
        })
    }
}

fn main() {
    let file_path = "data/day05_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(part1(input), 35);
    }

    #[test]
    fn test_part2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(part2(input), 46);
    }
}
