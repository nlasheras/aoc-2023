use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use regex::Regex;

pub struct RuleCond {
    pub var : char,
    pub op : char,
    pub number : u32,
    pub result : String
}

pub struct Rule {
    pub name: String,
    pub conditions: Vec<RuleCond>,
    pub else_result: String
}

impl Rule {
    pub fn from(input:&str) -> Rule {
        //px{a<2006:qkq,m>2090:A,rfg}
        let start_brace = input.find('{').unwrap();
        let end_brace = input.find('}').unwrap();
        let name = input[0..start_brace].to_string();
        let cond_str = input[start_brace+1..end_brace].split(',').collect::<Vec<&str>>();
        
        let mut conditions = Vec::new();
        for i in 0..cond_str.len()-1 {
            let s = cond_str[i];
            let colon = s.find(':').unwrap();
            let number = s[2..colon].parse::<u32>().unwrap();
            let result = s[colon+1..s.len()].to_string();
            conditions.push(RuleCond{ var: s.chars().nth(0).unwrap(), op: s.chars().nth(1).unwrap(), number: number, result: result });
        }
        let else_result = cond_str.iter().last().unwrap().to_string();

        Rule { name: name, conditions: conditions, else_result: else_result }
    }

    pub fn eval(&self, part: &Part) -> String {
        for r in self.conditions.iter() {
            let value = match r.var {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => panic!()
            };
            let is_true = match r.op {
                '>' => value > r.number,
                '<' => value < r.number,
                _ => panic!()
            };
            if is_true {
                return r.result.clone();
            }

        }
        self.else_result.clone()
    }
}

#[derive(Clone, Copy)]
pub struct Part {
    pub x : u32,
    pub m : u32,
    pub a : u32,
    pub s : u32
}

impl Part {
    pub fn from(input:&str) -> Part {
        let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
        let caps = re.captures(input).unwrap();
        let nums = caps.iter().skip(1).map(|s| s.unwrap().as_str().parse::<u32>().unwrap()).collect::<Vec<u32>>();
        Part { x: nums[0], m: nums[1], a: nums[2], s: nums[3] }
    }

    pub fn rating_number(&self) -> u64 {
        (self.x+self.m+self.a+self.s) as u64
    }
}

#[aoc_generator(day19)]
pub fn parse_input(input: &str) -> (BTreeMap<String, Rule>, Vec<Part>)  {
    let input_parts = input.split("\n\n").collect::<Vec<&str>>();

    let rules_vec = input_parts[0].lines().map(|s| Rule::from(s)).collect::<Vec<Rule>>();
    let mut rules = BTreeMap::new();
    for r in rules_vec {
        rules.insert(r.name.clone(), r);
    }

    let parts = input_parts[1].lines().map(|s| Part::from(s)).collect();
    (rules, parts)
}

pub fn get_acepted_parts(rules: &BTreeMap<String, Rule>, parts: &Vec<Part>) -> Vec<Part> {
    let mut accepted = Vec::new();
    for p in parts {
        let mut rule_name = "in".to_string();
        while rule_name != "A".to_string() && rule_name != "R".to_string() {
            let rule  = rules.get(&rule_name).unwrap();
            rule_name = rule.eval(p);
        }
        if rule_name == "A" {
            accepted.push(*p);
        }
    }
    accepted
}


#[aoc(day19, part1)]
pub fn sum_rating_numbers(input: &(BTreeMap<String, Rule>, Vec<Part>)) -> u64 {
    let accepted = get_acepted_parts(&input.0, &input.1);
    accepted.iter().map(|p| p.rating_number()).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const DAY19_EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
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

    #[test]
    fn test_day19_part1() {
        let input = parse_input(DAY19_EXAMPLE);
        assert_eq!(sum_rating_numbers(&input), 19114);
    }

}