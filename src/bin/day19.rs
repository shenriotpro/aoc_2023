use std::{collections::HashMap, fs};

use itertools::Itertools;

#[derive(Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Clone, Copy)]
struct PartSet {
    x_min: i64,
    x_max: i64,
    m_min: i64,
    m_max: i64,
    a_min: i64,
    a_max: i64,
    s_min: i64,
    s_max: i64,
}

#[derive(Clone)]
enum Result {
    A,
    R,
    WRef(String),
}

#[derive(Clone)]
enum Dimension {
    X,
    M,
    A,
    S,
}

#[derive(Clone)]
enum Rule {
    Res(Result),
    LT {
        dimension: Dimension,
        value: i64,
        result: Result,
    },
    GT {
        dimension: Dimension,
        value: i64,
        result: Result,
    },
}

fn part1(input: &str) -> i64 {
    let (workflows, parts) = input
        .split("\n\n")
        .collect_tuple()
        .expect("Should be a valid input");

    let workflows = workflows.lines().map(parse_workflow).collect_vec();
    let workflows: HashMap<String, Workflow> = workflows
        .iter()
        .map(|w| (w.name.clone(), w.clone()))
        .collect();
    let parts = parts.lines().map(parse_part).collect_vec();

    let mut res = 0;
    for part in parts {
        let mut cur_result = Result::WRef("in".to_string());
        loop {
            match &cur_result {
                Result::WRef(s) => {
                    for rule in &workflows.get(s).expect("Should be a valid workflow").rules {
                        match rule {
                            Rule::Res(res) => {
                                cur_result = res.clone();
                                break;
                            }
                            Rule::LT {
                                dimension,
                                value,
                                result,
                            } => {
                                let v = match dimension {
                                    Dimension::X => part.x,
                                    Dimension::M => part.m,
                                    Dimension::A => part.a,
                                    Dimension::S => part.s,
                                };
                                if v < *value {
                                    cur_result = result.clone();
                                    break;
                                }
                            }
                            Rule::GT {
                                dimension,
                                value,
                                result,
                            } => {
                                let v = match dimension {
                                    Dimension::X => part.x,
                                    Dimension::M => part.m,
                                    Dimension::A => part.a,
                                    Dimension::S => part.s,
                                };
                                if v > *value {
                                    cur_result = result.clone();
                                    break;
                                }
                            }
                        }
                    }
                }
                Result::R => break,
                Result::A => {
                    res += part.x + part.m + part.a + part.s;
                    break;
                }
            }
        }
    }

    res
}

fn parse_workflow(line: &str) -> Workflow {
    let (name, rules) = line.split_once('{').expect("Should have rules");
    let name = name.to_string();

    let (rules, _) = rules.split_once('}').expect("Should have rules");
    let rules = rules.split(',').map(parse_rule).collect_vec();

    Workflow { name, rules }
}

fn parse_rule(s: &str) -> Rule {
    if s.contains('>') {
        let (cond, result) = s.split_once(':').expect("Should be a valid rule");
        let (dimension, value) = cond.split_once('>').expect("Should be a valid condition");

        let dimension = parse_dimension(dimension);
        let value = value.parse().expect("Should be a valid value");
        let result = parse_result(result);

        Rule::GT {
            dimension,
            value,
            result,
        }
    } else if s.contains('<') {
        let (cond, result) = s.split_once(':').expect("Should be a valid rule");
        let (dimension, value) = cond.split_once('<').expect("Should be a valid condition");

        let dimension = parse_dimension(dimension);
        let value = value.parse().expect("Should be a valid value");
        let result = parse_result(result);

        Rule::LT {
            dimension,
            value,
            result,
        }
    } else {
        Rule::Res(parse_result(s))
    }
}

fn parse_result(s: &str) -> Result {
    match s {
        "A" => Result::A,
        "R" => Result::R,
        res => Result::WRef(res.to_string()),
    }
}

fn parse_dimension(s: &str) -> Dimension {
    match s {
        "x" => Dimension::X,
        "m" => Dimension::M,
        "a" => Dimension::A,
        "s" => Dimension::S,
        _ => panic!("Should be a valid dimension"),
    }
}

fn parse_part(line: &str) -> Part {
    let (x, m, a, s) = line
        .split_once('{')
        .expect("Should be a valid part")
        .1
        .split_once('}')
        .expect("Should be a valid part")
        .0
        .split(',')
        .map(|s| {
            s.split_once('=')
                .expect("Should be a valid part")
                .1
                .parse::<i64>()
                .expect("Should be a valid value")
        })
        .collect_tuple()
        .expect("Should be a valid part");
    Part { x, m, a, s }
}

fn part2(input: &str) -> i64 {
    let (workflows, _) = input
        .split("\n\n")
        .collect_tuple()
        .expect("Should be a valid input");

    let workflows = workflows.lines().map(parse_workflow).collect_vec();
    let workflows: HashMap<String, Workflow> = workflows
        .iter()
        .map(|w| (w.name.clone(), w.clone()))
        .collect();

    let parts = PartSet {
        x_min: 1,
        x_max: 4000,
        m_min: 1,
        m_max: 4000,
        a_min: 1,
        a_max: 4000,
        s_min: 1,
        s_max: 4000,
    };
    let result = Result::WRef("in".to_string());
    accepted(parts, result, &workflows)
}

fn accepted(parts: PartSet, result: Result, workflows: &HashMap<String, Workflow>) -> i64 {
    let size = parts_size(parts);
    if size == 0 {
        return 0;
    }
    match result {
        Result::A => size,
        Result::R => 0,
        Result::WRef(s) => {
            let mut res = 0;
            let wflow = workflows.get(&s).expect("Should be a valid workflow");
            let mut cur_parts = parts;
            for rule in &wflow.rules {
                match rule {
                    Rule::Res(r) => {
                        res += accepted(cur_parts, r.clone(), workflows);
                        break;
                    }
                    Rule::LT {
                        dimension,
                        value,
                        result,
                    } => {
                        let mut parts = cur_parts;
                        match dimension {
                            Dimension::X => {
                                if cur_parts.x_max >= *value {
                                    // TODO: unsigned?
                                    parts.x_max = *value - 1;
                                    res += accepted(parts, result.clone(), workflows);
                                    cur_parts.x_min = *value;
                                }
                            }
                            Dimension::M => {
                                if cur_parts.m_max >= *value {
                                    // TODO: unsigned?
                                    parts.m_max = *value - 1;
                                    res += accepted(parts, result.clone(), workflows);
                                    cur_parts.m_min = *value;
                                }
                            }
                            Dimension::A => {
                                if cur_parts.a_max >= *value {
                                    // TODO: unsigned?
                                    parts.a_max = *value - 1;
                                    res += accepted(parts, result.clone(), workflows);
                                    cur_parts.a_min = *value;
                                }
                            }
                            Dimension::S => {
                                if cur_parts.s_max >= *value {
                                    // TODO: unsigned?
                                    parts.s_max = *value - 1;
                                    res += accepted(parts, result.clone(), workflows);
                                    cur_parts.s_min = *value;
                                }
                            }
                        }
                    }
                    Rule::GT {
                        dimension,
                        value,
                        result,
                    } => {
                        let mut parts = cur_parts;
                        match dimension {
                            Dimension::X => {
                                if cur_parts.x_min <= *value {
                                    // TODO: unsigned?
                                    parts.x_min = *value + 1;
                                    res += accepted(parts, result.clone(), workflows);
                                    cur_parts.x_max = *value;
                                }
                            }
                            Dimension::M => {
                                if cur_parts.m_min <= *value {
                                    // TODO: unsigned?
                                    parts.m_min = *value + 1;
                                    res += accepted(parts, result.clone(), workflows);
                                    cur_parts.m_max = *value;
                                }
                            }
                            Dimension::A => {
                                if cur_parts.a_min <= *value {
                                    // TODO: unsigned?
                                    parts.a_min = *value + 1;
                                    res += accepted(parts, result.clone(), workflows);
                                    cur_parts.a_max = *value;
                                }
                            }
                            Dimension::S => {
                                if cur_parts.s_min <= *value {
                                    // TODO: unsigned?
                                    parts.s_min = *value + 1;
                                    res += accepted(parts, result.clone(), workflows);
                                    cur_parts.s_max = *value;
                                }
                            }
                        }
                    }
                }
            }
            res
        }
    }
}

fn parts_size(parts: PartSet) -> i64 {
    (parts.x_max - parts.x_min + 1)
        * (parts.m_max - parts.m_min + 1)
        * (parts.a_max - parts.a_min + 1)
        * (parts.s_max - parts.s_min + 1)
}

fn main() {
    let file_path = "data/day19_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

        assert_eq!(part1(input), 19114);
    }

    #[test]
    fn test_part2() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

        assert_eq!(part2(input), 167409079868000);
    }
}
