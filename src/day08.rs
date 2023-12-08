use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::BTreeMap;
use regex::Regex;

use num::integer::lcm;

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> (Vec<char>, BTreeMap<String, (String, String)>) {
    let mut parts = input.lines();
    let instructions = parts.next().unwrap();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let mut network = BTreeMap::new();
    let network_lines : Vec<String> = input.lines().skip(2).map(|s| s.to_string()).collect();
    for l in network_lines {
        if let Some(c) = re.captures(&l) {
            let key = c.get(1).unwrap().as_str();
            let value = (c.get(2).unwrap().as_str(), c.get(3).unwrap().as_str());
            network.insert(key.to_string().clone(), (value.0.to_string().clone(), value.1.to_string().clone()));
        }
    }
    (instructions.chars().collect(), network)
}

pub fn count_steps(input: &(Vec<char>, BTreeMap<String, (String, String)>), node: &str) -> u64 {
    let mut count : u64 = 0;
    let mut current = &String::from(node);
    while !current.ends_with("Z") {
        let node = input.1.get(current).unwrap();
        let i = count as usize % input.0.len();
        let instruction = input.0[i];
        match instruction {
            'L' => current = &node.0,
            'R' => current = &node.1,
            _ => panic!()
        }
        count += 1;
    }
    count
}

#[aoc(day8, part1)]
pub fn count_steps_from_aaa(input: &(Vec<char>, BTreeMap<String, (String, String)>)) -> u64 {
    count_steps(input, "AAA")
}

#[aoc(day8, part2)]
pub fn count_steps_from_all(input: &(Vec<char>, BTreeMap<String, (String, String)>)) -> u64 {
    let start : Vec<&String> = input.1.keys().filter(|s| s.ends_with("A")).collect();

    start.iter().map(|n| count_steps(input, n)).fold(1, |ret, n| lcm(ret, n))
}


#[cfg(test)]
mod tests {
    use super::*;

    const DAY08_EXAMPLE1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_day8_example1() {
        let input = parse_input(DAY08_EXAMPLE1);
        assert_eq!(count_steps_from_aaa(&input), 2);
    }

    const DAY08_EXAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";


    #[test]
    fn test_day8_example2() {
        let input = parse_input(DAY08_EXAMPLE2);
        assert_eq!(count_steps_from_aaa(&input), 6);
    }

    const DAY08_EXAMPLE3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_day8_example3() {
        let input = parse_input(DAY08_EXAMPLE3);
        assert_eq!(count_steps_from_all(&input), 6);
    }
    
}
