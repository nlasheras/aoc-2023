use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<Vec<i64>>  {
    input.lines().map(|s| {
        s.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect()
    }).collect()
}

pub fn get_next_value(input: &Vec<i64>) -> i64 {
    let seq : Vec<i64> = input.windows(2).map(|p| p[1] - p[0]).collect();
    if seq.iter().all(|n| *n == 0) {
        return *input.iter().last().unwrap();
    }
    get_next_value(&seq) + *input.iter().last().unwrap()
}

#[aoc(day9, part1)]
pub fn sum_next_values(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|v| get_next_value(v)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY09_EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_day9_get_next_value() {
        assert_eq!(get_next_value(&vec![0, 3, 6, 9, 12, 15]), 18);
    }

    #[test]
    fn test_day9_part1() {
        let input = parse_input(DAY09_EXAMPLE);
        assert_eq!(sum_next_values(&input), 114);
    }

}
