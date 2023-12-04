use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::BTreeSet;

use std::iter;
use std::cmp;

type Line = (Vec<u64>, BTreeSet<u64>);

pub fn parse_numbers(input: &str) -> Vec<u64> {
    input.split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

pub fn parse_line(input: &str) -> Line {
    let mut parts = input.split(" | ");
    let card_numbers = parse_numbers(parts.next().unwrap().split(": ").skip(1).nth(0).unwrap());
    let winning = parse_numbers(parts.next().unwrap());
    (card_numbers, BTreeSet::from_iter(winning))
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|s| parse_line(s) )
        .collect()
}

pub fn count_matches(input: &Line) -> u32 {
    input.0.iter().filter(|n| input.1.contains(n)).count() as u32
}

pub fn points_line(input: &Line) -> u64 {
    let count = count_matches(input);
    if count > 0 {
        return 2u64.pow(count - 1)
    }
    0
}

#[aoc(day4, part1)]
pub fn sum_points(input: &[Line]) -> u64 {
    input.iter().fold(0, |sum, l| sum + points_line(l))
}

pub fn get_copies(input: &[Line]) -> Vec<u64> {
    let mut ret : Vec<u64> = iter::repeat(1).take(input.len()).collect();
    for i in 0..input.len() {
        let l = &input[i];
        let points = count_matches(l) as usize;
        let copies = ret[i];
        for j in i+1..cmp::min(i+points+1, input.len()) {
            ret[j] += copies
        }
    }
    ret
}

#[aoc(day4, part2)]
pub fn sum_copies(input: &[Line]) -> u64 {
    get_copies(input).iter().fold(0, |sum, n| sum + n)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY04_EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";


    #[test]
    fn test_day4_part1() {
        let input = parse_input(DAY04_EXAMPLE);
        assert_eq!(sum_points(&input), 13);
    }

    #[test]
    fn test_day4_part2() {
        let input = parse_input(DAY04_EXAMPLE);
        assert_eq!(sum_copies(&input), 30);
    }
}
