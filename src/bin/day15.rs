use std::fs;

use itertools::Itertools;

#[derive(Clone)]
struct Lens {
    label: String,
    focal: u8,
}

fn part1(input: &str) -> i64 {
    // TODO: be careful about whitespaces
    input.split(',').map(hash).sum()
}

fn hash(input: &str) -> i64 {
    let mut res = 0;
    // TODO: be careful about whitespaces
    for c in input.chars() {
        res += c as u32;
        res *= 17;
        res %= 256;
    }
    res as i64
}

fn part2(input: &str) -> i64 {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    for op in input.split(',') {
        if op.contains('-') {
            let label = op.split_once('-').expect("Should have a delimiter").0;
            let hash = hash(label);
            boxes[hash as usize] = boxes[hash as usize]
                .iter()
                .filter(|lens| lens.label != label)
                .cloned()
                .collect_vec();
        } else {
            let (label, focal) = op.split_once('=').expect("Should have a delimiter");
            let focal = focal.parse::<u8>().expect("Should be a digit");
            let lens = boxes[hash(label) as usize]
                .iter_mut()
                .find(|lens| lens.label == label);
            match lens {
                Some(lens) => {
                    lens.focal = focal;
                }
                None => {
                    boxes[hash(label) as usize].push(Lens {
                        label: label.to_string(),
                        focal,
                    });
                }
            }
        }
    }

    let mut res = 0;
    for (i, box_) in boxes.iter().enumerate() {
        for (j, lens) in box_.iter().enumerate() {
            res += ((i + 1) * (j + 1)) as i64 * lens.focal as i64;
        }
    }
    res
}

fn main() {
    let file_path = "data/day15_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(part1(input), 1320);
    }

    #[test]
    fn test_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(part2(input), 145);
    }
}
