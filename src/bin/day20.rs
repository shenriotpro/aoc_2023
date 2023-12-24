use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use itertools::Itertools;

#[derive(Clone, Debug)]
enum Module {
    FF(FlipFlop),
    Conj(Conjunction),
    BC(Broadcaster),
}

impl Module {
    fn name(&self) -> String {
        match &self {
            Module::FF(ff) => ff.name.clone(),
            Module::Conj(conj) => conj.name.clone(),
            Module::BC(_) => "broadcaster".to_string(),
        }
    }

    fn dest(&self) -> Vec<String> {
        match &self {
            Module::FF(ff) => ff.dest.clone(),
            Module::Conj(conj) => conj.dest.clone(),
            Module::BC(bc) => bc.dest.clone(),
        }
    }

    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse> {
        match self {
            Module::FF(ff) => {
                if pulse.high {
                    vec![]
                } else {
                    ff.on = !ff.on;
                    ff.dest
                        .iter()
                        .map(|d| Pulse {
                            from: ff.name.clone(),
                            high: ff.on,
                            to: d.clone(),
                        })
                        .collect_vec()
                }
            }
            Module::Conj(conj) => {
                conj.input.insert(pulse.from.clone(), pulse.high);
                // TODO: correctly handle low as default
                let all = conj.input.values().all(|b| *b);
                conj.dest
                    .iter()
                    .map(|d| Pulse {
                        from: conj.name.clone(),
                        high: !all,
                        to: d.clone(),
                    })
                    .collect_vec()
            }
            Module::BC(bc) => bc
                .dest
                .iter()
                .map(|d| Pulse {
                    from: "broadcaster".to_string(),
                    high: pulse.high,
                    to: d.clone(),
                })
                .collect_vec(),
        }
    }
}

#[derive(Clone, Debug)]
struct FlipFlop {
    name: String,
    dest: Vec<String>,
    on: bool,
}

#[derive(Clone, Debug)]
struct Conjunction {
    name: String,
    dest: Vec<String>,
    input: HashMap<String, bool>,
}

#[derive(Clone, Debug)]
struct Broadcaster {
    dest: Vec<String>,
}

#[derive(Debug)]
struct Pulse {
    from: String,
    high: bool,
    to: String,
}

fn part1(input: &str) -> i64 {
    let modules = input.lines().map(parse_module).collect_vec();
    let mut modules: HashMap<String, Module> = modules
        .iter()
        .cloned()
        .map(|m| (m.name(), m.clone()))
        .collect();

    for m in modules.clone().values() {
        for d in m.dest() {
            if modules.contains_key(&d) {
                if let Some(Module::Conj(conj)) = modules.get_mut(&d) {
                    conj.input.insert(m.name().clone(), false);
                }
            }
        }
    }

    let mut nb_lows = 0;
    let mut nb_highs = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(Pulse {
            from: "button".to_string(),
            high: false,
            to: "broadcaster".to_string(),
        });

        while !queue.is_empty() {
            let pulse = queue.pop_front().expect("Should not be empty");
            // println!("{:?}", pulse);
            match pulse.high {
                true => nb_highs += 1,
                false => nb_lows += 1,
            }
            let to = modules.get_mut(&pulse.to);
            if let Some(to) = to {
                let next_pulses = to.receive(pulse);
                queue.extend(next_pulses);
            }
        }
    }

    nb_lows * nb_highs
}

fn parse_module(line: &str) -> Module {
    if let Some(module) = line.strip_prefix('%') {
        let (name, dest) = module.split_once(" -> ").expect("Should be a valid module");
        let name = name.to_string();
        let dest = if dest.contains(", ") {
            dest.split(", ").map(str::to_string).collect_vec()
        } else {
            vec![dest.to_string()]
        };
        Module::FF(FlipFlop {
            name,
            dest,
            on: false,
        })
    } else if let Some(module) = line.strip_prefix('&') {
        let (name, dest) = module.split_once(" -> ").expect("Should be a valid module");
        let name = name.to_string();
        let dest = if dest.contains(", ") {
            dest.split(", ").map(str::to_string).collect_vec()
        } else {
            vec![dest.to_string()]
        };
        Module::Conj(Conjunction {
            name,
            dest,
            input: HashMap::new(),
        })
    } else if line.starts_with("broadcaster") {
        let (_, dest) = line.split_once(" -> ").expect("Should be a valid module");
        let dest = if dest.contains(", ") {
            dest.split(", ").map(str::to_string).collect_vec()
        } else {
            vec![dest.to_string()]
        };
        Module::BC(Broadcaster { dest })
    } else {
        panic!("Should be a valid module")
    }
}

fn part2(input: &str) -> i64 {
    let modules = input.lines().map(parse_module).collect_vec();
    let mut modules: HashMap<String, Module> = modules
        .iter()
        .cloned()
        .map(|m| (m.name(), m.clone()))
        .collect();

    for m in modules.clone().values() {
        for d in m.dest() {
            if modules.contains_key(&d) {
                if let Some(Module::Conj(conj)) = modules.get_mut(&d) {
                    conj.input.insert(m.name().clone(), false);
                }
            }
        }
    }

    for i in 1..4000 {
        // println!("{i}");
        let mut queue = VecDeque::new();
        queue.push_back(Pulse {
            from: "button".to_string(),
            high: false,
            to: "broadcaster".to_string(),
        });

        while !queue.is_empty() {
            let pulse = queue.pop_front().expect("Should not be empty");
            if pulse.to == "qt" && pulse.high {
                println!("{}: {:?}", i, pulse)
            }
            let to = modules.get_mut(&pulse.to);
            if let Some(to) = to {
                let next_pulses = to.receive(pulse);
                queue.extend(next_pulses);
            }
        }
        // for module in modules.values() {
        //     println!("{:?}", module);
        // }
    }

    0
}

fn main() {
    let file_path = "data/day20_input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test_part1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        assert_eq!(part1(input), 32000000);
    }

    #[test]
    fn test_part1_alt() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        assert_eq!(part1(input), 11687500);
    }
}
