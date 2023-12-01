use std::fs;

fn part1(input: &str) -> i32 {
    let mut res = 0;
    for line in input.lines() {
        let digits: Vec<_> = line.chars().filter(|c| c.is_ascii_digit()).collect();
        // Note: first and last digits may be the same.
        let mut value = String::new();
        value.push(*digits.first().expect("Should have digits"));
        value.push(*digits.last().expect("Should have digits"));
        let value = value.parse::<i32>().expect("Should be an integer");
        res += value;
    }
    res
}

fn part2(input: &str) -> i32 {
    let mut res = 0;
    let hays = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    for line in input.lines() {
        // Note: digits may overlap ("twone"), we can't trvially rewrite them.
        // But there should always be a clear first and last (no prefix/suffix).
        // Regexes don't seem to work well with overlapping matches.
        // "zero" may not be a digit?
        let mut first: Option<&str> = None;
        let mut first_idx: Option<usize> = None;
        for hay in hays {
            if let Some(idx) = line.find(hay) {
                if first_idx.is_none() || idx < first_idx.unwrap() {
                    first_idx = Some(idx);
                    first = Some(hay);
                }
            }
        }
        let mut last: Option<&str> = None;
        let mut last_idx: Option<usize> = None;
        for hay in hays {
            if let Some(idx) = line.rfind(hay) {
                if last_idx.is_none() || idx > last_idx.unwrap() {
                    last_idx = Some(idx);
                    last = Some(hay);
                }
            }
        }
        let mut value = String::new();
        value.push_str(first.expect("Should have digits"));
        value.push_str(last.expect("Should have digits"));
        let value = value
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9");
        let value = value.parse::<i32>().expect("Should be an integer");
        res += value;
    }
    res
}

fn main() {
    let file_path = "data/day01_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(part2(input), 281);
    }

    #[test]
    fn test_part2_twone() {
        let input = "twone";

        assert_eq!(part2(input), 21);
    }
}
