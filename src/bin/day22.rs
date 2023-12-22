use std::{collections::HashSet, fs, ops::RangeInclusive};

use itertools::Itertools;

#[derive(Clone, Debug)]
struct Block {
    xs: RangeInclusive<i64>,
    ys: RangeInclusive<i64>,
    zs: RangeInclusive<i64>,
}

fn part1(input: &str) -> i64 {
    let mut blocks = input.lines().map(parse_block).collect_vec();

    blocks.sort_by_key(|b| *b.zs.start());
    let mut cubes = HashSet::new();
    for block in &blocks {
        for x in block.xs.clone() {
            for y in block.ys.clone() {
                for z in block.zs.clone() {
                    cubes.insert((x, y, z));
                }
            }
        }
    }

    let mut to_move = (0..blocks.len()).collect_vec();
    while !to_move.is_empty() {
        let mut next_to_move = vec![];
        for i in to_move {
            let mut stuck = false;
            let block = &blocks[i];
            if *block.zs.start() == 1 {
                continue;
            }
            'outer: for x in block.xs.clone() {
                for y in block.ys.clone() {
                    let z = *block.zs.start();
                    if cubes.contains(&(x, y, z - 1)) {
                        stuck = true;
                        break 'outer;
                    }
                }
            }
            if !stuck {
                next_to_move.push(i);
                for x in block.xs.clone() {
                    for y in block.ys.clone() {
                        for z in block.zs.clone() {
                            cubes.remove(&(x, y, z));
                            cubes.insert((x, y, z - 1));
                        }
                    }
                }
                blocks[i].zs = (block.zs.start() - 1)..=(block.zs.end() - 1);
            }
        }
        to_move = next_to_move;
    }

    let mut res = blocks.len() as i64;
    for (i, block) in blocks.iter().enumerate() {
        for x in block.xs.clone() {
            for y in block.ys.clone() {
                for z in block.zs.clone() {
                    cubes.remove(&(x, y, z));
                }
            }
        }
        for (j, block2) in blocks.iter().enumerate() {
            if i == j {
                continue;
            }
            if *block2.zs.start() == 1 {
                continue;
            }
            let mut stuck = false;
            'outer: for x in block2.xs.clone() {
                for y in block2.ys.clone() {
                    let z = *block2.zs.start();
                    if cubes.contains(&(x, y, z - 1)) {
                        stuck = true;
                        break 'outer;
                    }
                }
            }
            if !stuck {
                res -= 1;
                break;
            }
        }
        for x in block.xs.clone() {
            for y in block.ys.clone() {
                for z in block.zs.clone() {
                    cubes.insert((x, y, z));
                }
            }
        }
    }
    res
}

fn parse_block(line: &str) -> Block {
    let (mins, maxs) = line.split_once('~').expect("Should have a delimiter");
    let mins = mins.split(',').flat_map(str::parse).collect_vec();
    let maxs = maxs.split(',').flat_map(str::parse).collect_vec();
    Block {
        xs: mins[0]..=maxs[0],
        ys: mins[1]..=maxs[1],
        zs: mins[2]..=maxs[2],
    }
}

fn part2(input: &str) -> i64 {
    let mut blocks = input.lines().map(parse_block).collect_vec();

    blocks.sort_by_key(|b| *b.zs.start());
    let mut cubes = HashSet::new();
    for block in &blocks {
        for x in block.xs.clone() {
            for y in block.ys.clone() {
                for z in block.zs.clone() {
                    cubes.insert((x, y, z));
                }
            }
        }
    }

    let mut to_move = (0..blocks.len()).collect_vec();
    while !to_move.is_empty() {
        let mut next_to_move = vec![];
        for i in to_move {
            let mut stuck = false;
            let block = &blocks[i];
            if *block.zs.start() == 1 {
                continue;
            }
            'outer: for x in block.xs.clone() {
                for y in block.ys.clone() {
                    let z = *block.zs.start();
                    if cubes.contains(&(x, y, z - 1)) {
                        stuck = true;
                        break 'outer;
                    }
                }
            }
            if !stuck {
                next_to_move.push(i);
                for x in block.xs.clone() {
                    for y in block.ys.clone() {
                        for z in block.zs.clone() {
                            cubes.remove(&(x, y, z));
                            cubes.insert((x, y, z - 1));
                        }
                    }
                }
                blocks[i].zs = (block.zs.start() - 1)..=(block.zs.end() - 1);
            }
        }
        to_move = next_to_move;
    }

    let mut res = 0;
    for (i, _) in blocks.iter().enumerate() {
        res += count_moving(&[&blocks[..i], &blocks[i + 1..]].concat());
    }
    res
}

fn count_moving(blocks: &[Block]) -> i64 {
    let mut blocks = blocks.to_owned();
    blocks.sort_by_key(|b| *b.zs.start());
    let mut cubes = HashSet::new();
    for block in &blocks {
        for x in block.xs.clone() {
            for y in block.ys.clone() {
                for z in block.zs.clone() {
                    cubes.insert((x, y, z));
                }
            }
        }
    }

    let mut moved = HashSet::new();
    let mut to_move = (0..blocks.len()).collect_vec();
    while !to_move.is_empty() {
        let mut next_to_move = vec![];
        for i in to_move {
            let mut stuck = false;
            let block = &blocks[i];
            if *block.zs.start() == 1 {
                continue;
            }
            'outer: for x in block.xs.clone() {
                for y in block.ys.clone() {
                    let z = *block.zs.start();
                    if cubes.contains(&(x, y, z - 1)) {
                        stuck = true;
                        break 'outer;
                    }
                }
            }
            if !stuck {
                moved.insert(i);
                next_to_move.push(i);
                for x in block.xs.clone() {
                    for y in block.ys.clone() {
                        for z in block.zs.clone() {
                            cubes.remove(&(x, y, z));
                            cubes.insert((x, y, z - 1));
                        }
                    }
                }
                blocks[i].zs = (block.zs.start() - 1)..=(block.zs.end() - 1);
            }
        }
        to_move = next_to_move;
    }

    moved.len() as i64
}

fn main() {
    let file_path = "data/day22_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        assert_eq!(part1(input), 5);
    }

    #[test]
    fn test_part2() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        assert_eq!(part2(input), 7);
    }
}
