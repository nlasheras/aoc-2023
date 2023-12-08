use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::BTreeMap;
use regex::Regex;

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


#[aoc(day8, part1)]
pub fn count_steps(input: &(Vec<char>, BTreeMap<String, (String, String)>)) -> u64 {
    0
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
        assert_eq!(count_steps(&input), 2);
    }   

    const DAY08_EXAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";


    #[test]
    fn test_day8_example2() {
        let input = parse_input(DAY08_EXAMPLE2);
        assert_eq!(count_steps(&input), 6);
    }   

}
