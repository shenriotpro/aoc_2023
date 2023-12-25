use std::fs;

use itertools::Itertools;

fn part1(input: &str) -> i64 {
    for line in input.lines() {
        let (src, neighbours) = line.split_once(": ").expect("Should have a colon");
        let neighbours = neighbours.split_whitespace().collect_vec();
        for neighbour in neighbours {
            // dot -Tsvg g.txt > g.svg
            // remove the three edges in the middle (hover)
            // ccomps g.txt -v | head
            println!("{} -- {} [label={}_{}]", src, neighbour, src, neighbour);
        }
    }

    0
}

fn main() {
    let file_path = "data/day25_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test_part1() {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

        assert_eq!(part1(input), 54);
    }
}
