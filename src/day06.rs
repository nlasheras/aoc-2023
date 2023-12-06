use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::iter::zip;

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let mut parts = input.lines();
    let times : Vec<u64> = parts.next().unwrap().split(":").skip(1).next().unwrap().trim().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
    let distances : Vec<u64> = parts.next().unwrap().split(":").skip(1).next().unwrap().trim().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();

    zip(times, distances).collect()
}


pub fn lower_bound(input: &(u64, u64)) -> u64 {
    let (t, d) = *input;
    for press in 0..t {
        if (t - press)*press > d {
            return press
        }
    }
    0
}

pub fn higher_bound(input: &(u64, u64)) -> u64 {
    let (t, d) = *input;
    for press in (0..t).rev() {
        if (t - press)*press > d {
            return press
        }
    }
    0
}
#[aoc(day6, part1)]
pub fn count_winning_times(input: &[(u64, u64)]) -> u64 {
    input.iter().map(|p| higher_bound(p) - lower_bound(p) + 1).product()
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
}
