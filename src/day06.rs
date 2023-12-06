use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use std::iter::zip;

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let mut parts = input.lines();
    let times : Vec<u64> = parts.next().unwrap().split(":").skip(1).next().unwrap().trim().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
    let distances : Vec<u64> = parts.next().unwrap().split(":").skip(1).next().unwrap().trim().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();

    zip(times, distances).collect()
}

// solving with a quadratic in the form of t*press2 -t*press + d = 0
pub fn lower_bound(input: &(u64, u64)) -> u64 {
    let b : f64 = input.0 as f64 * -1f64;
    let c = input.1 as f64;
    
    let x = (-b - f64::sqrt(b.powf(2f64) - 4f64*c)) / 2f64;
    f64::ceil(x + 0.0001f64) as u64
}

pub fn higher_bound(input: &(u64, u64)) -> u64 {
    let b : f64 = input.0 as f64 * -1f64;
    let c = input.1 as f64;
    
    let x = (-b + f64::sqrt(b.powf(2f64) - 4f64*c)) / 2f64;
    f64::floor(x - 0.0001f64) as u64
}

#[aoc(day6, part1)]
pub fn count_winning_times(input: &[(u64, u64)]) -> u64 {
    input.iter().map(|p| higher_bound(p) - lower_bound(p) + 1).product()
}

#[aoc(day6, part2)]
pub fn count_winning_times_concat_numbers(input: &[(u64, u64)]) -> u64 {
    let t = input.iter().map(|p| p.0.to_string()).join("").parse::<u64>().unwrap();
    let d = input.iter().map(|p| p.1.to_string()).join("").parse::<u64>().unwrap();
    
    let p = (t, d);
    higher_bound(&p) - lower_bound(&p) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY06_EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";


    #[test]
    fn test_day6_part1() {
        let input = parse_input(DAY06_EXAMPLE);
        assert_eq!(count_winning_times(&input), 288)
    }

    #[test]
    fn test_day6_lower_bound() {
        assert_eq!(lower_bound(&(7, 9)), 2)
    }

    #[test]
    fn test_day6_higher_bound() {
        assert_eq!(higher_bound(&(7, 9)), 5)
    }

    #[test]
    fn test_day6_part2() {
        let input = parse_input(DAY06_EXAMPLE);
        assert_eq!(count_winning_times_concat_numbers(&input), 71503)
    }

}
