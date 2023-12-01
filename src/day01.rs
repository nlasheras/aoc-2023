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

pub fn find_first_digit(input: &String) -> u32 {
    for c in input.chars() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap()
        }
    }
    0
}

pub fn find_last_digit(input: &String) -> u32 {
    for c in input.chars().rev() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap()
        }
    }
    0
}

pub fn find_calibration_value(input: &String) -> u64 {
    (find_first_digit(input) * 10 + find_last_digit(input)) as u64
}

#[aoc(day1, part1)]
pub fn solve_part1(entries: &[String]) -> u64 {
    let mut sum = 0;
    for line in entries {
        sum += find_calibration_value(line)
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_string3() {
        let input = String::from("a1b2c3d4e5f");
        assert_eq!(find_first_digit(&input), 1);
        assert_eq!(find_last_digit(&input), 5);
        assert_eq!(find_calibration_value(&input), 15);
    }
}
