use std::str::FromStr;

/// Split a string and return values that can be parsed.
/// Note that the wrong type may result in silent failures.
pub fn split_parse<T: FromStr>(s: &str) -> Vec<T> {
    s.split_whitespace()
        .map(|s| s.parse())
        .filter_map(Result::ok)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::split_parse;

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
}
