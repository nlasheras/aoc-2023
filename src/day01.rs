use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<String> {
    let ret: Vec<String> = input
        .lines()
        .map(|x| x.to_owned())
        .collect();
    ret
}

pub fn get_digits(input: &String, also_text: bool) -> Vec<u32> {
    let nums = [ "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut ret = Vec::<u32>::new();
    for i in 0..input.len() {
        let c = input.chars().nth(i).unwrap();
        if c.is_numeric() {
            ret.push(c.to_digit(10).unwrap())
        }
        if also_text {
            let sub = &input[i..input.len()];
            for j in 0..10 {
                if sub.starts_with(nums[j]) {
                    ret.push(j as u32);
                    break;
                }
            }
        }
    }
    ret
}

pub fn find_calibration_value(input: &String, also_text: bool) -> u64 {
    let digits = get_digits(input, also_text);
    let first = *digits.first().unwrap();
    let last = *digits.last().unwrap();
    (first * 10 + last) as u64
}

#[aoc(day1, part1)]
pub fn solve_part1(entries: &[String]) -> u64 {
    entries.iter().fold(0, |sum, line| sum + find_calibration_value(line, false))
}

#[aoc(day1, part2)]
pub fn solve_part2(entries: &[String]) -> u64 {
    entries.iter().fold(0, |sum, line| sum + find_calibration_value(line, true))
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1_example3() {
        let input = String::from("a1b2c3d4e5f");
        assert_eq!(find_calibration_value(&input, false), 15);
    }

    #[test]
    fn test_day1_part2_example7() {
        let input = String::from("7pqrstsixteen");
        assert_eq!(find_calibration_value(&input, true), 76);
    }
}
