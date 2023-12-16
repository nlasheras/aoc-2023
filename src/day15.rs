use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::BTreeMap;

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> Vec<String>  {
    input.split(",").map(|s| s.to_string()).collect()
}


fn hash(input: &str) -> u64 {
    input.chars().fold(0, |current, c| {
        let mut tmp = current + c as u64;
        tmp *= 17;
        return tmp % 256;
    })
}

#[aoc(day15, part1)]
pub fn sum_hashes(input: &Vec<String>) -> u64 {
    input.iter().map(|s| hash(s)).sum()
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Equal(String, u64),
    Minus(String)
}

fn parse_lens(input: &str) -> Operation {
    if input.contains("=") {
        let parts : Vec<&str> = input.split("=").collect();
        return Operation::Equal(parts[0].to_string(), parts[1].parse::<u64>().unwrap());
    } else if input.contains("-") {
        return Operation::Minus(input.split("-").next().unwrap().to_string());
    }
    panic!();
}

#[aoc(day15, part2)]
pub fn sum_focal_lengths(input: &Vec<String>) -> u64 {
    let mut boxes : BTreeMap<u64, Vec<(String, u64)>> = BTreeMap::new();
    for s in input {
        match parse_lens(s) {
            Operation::Equal(code, lens) => {
                let b  = boxes.entry(hash(&code)).or_insert(Vec::new());
                if let Some(pos) = b.iter().position(|(c, _)| *c == code) {
                    b[pos] = (code, lens);
                }
                else {
                    b.push((code, lens));
                }
            },
            Operation::Minus(code) => {
                let b  = boxes.entry(hash(&code)).or_insert(Vec::new());
                if let Some(pos) = b.iter().position(|(c, _)| *c == code) {
                    b.remove(pos);
                }
            }
        }
    }
    boxes.iter().map(|(i, lenses)| {
        lenses.iter().enumerate().map(|(pos, (_, lens))| ((*i+1) * (pos as u64+1) * *lens)).sum::<u64>()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY15_EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_day15_example1() {
        assert_eq!(hash("rn=1"), 30);
    }

    #[test]
    fn test_day15_part1() {
        let input = parse_input(DAY15_EXAMPLE);
        assert_eq!(sum_hashes(&input), 1320);
    }

    #[test]
    fn test_day15_part2_box1() {
        assert_eq!(hash("rn"), 0);
        assert_eq!(hash("cm"), 0);
    }

    #[test]
    fn test_day15_part2_box3() {
        assert_eq!(hash("pc"), 3);
        assert_eq!(hash("ot"), 3);
        assert_eq!(hash("ab"), 3);
    }

    #[test]
    fn test_day15_part2_split() {
        assert_eq!(parse_lens("rn=1"), Operation::Equal(String::from("rn"), 1));
        assert_eq!(parse_lens("cm-"), Operation::Minus(String::from("cm")));
    }

    #[test]
    fn test_day15_part2() {
        let input = parse_input(DAY15_EXAMPLE);
        assert_eq!(sum_focal_lengths(&input), 145);
    }
}