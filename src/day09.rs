use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<Vec<i64>>  {
    input.lines().map(|s| {
        s.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect()
    }).collect()
}

pub fn extrapolate_right(input: &Vec<i64>) -> Vec<i64> {
    let mut seq : Vec<i64> = input.windows(2).map(|p| p[1] - p[0]).collect();
    if !seq.iter().all(|n| *n == 0) {
        seq = extrapolate_right(&seq);
    }
    let right = *seq.iter().last().unwrap() + *input.iter().last().unwrap();
    [&input[..], &[right][..]].concat()
}

pub fn extrapolate_left(input: &Vec<i64>) -> Vec<i64> {
    let mut seq : Vec<i64> = input.windows(2).map(|p| p[1] - p[0]).collect();
    if !seq.iter().all(|n| *n == 0) {
        seq = extrapolate_left(&seq);
    }
    let left = *input.iter().nth(0).unwrap() - seq.iter().nth(0).unwrap();
    [&[left][..], &input[..]].concat()
}

#[aoc(day9, part1)]
pub fn sum_next_values(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|v| *extrapolate_right(v).iter().last().unwrap()).sum()
}

#[aoc(day9, part2)]
pub fn sum_first_values(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|v| *extrapolate_left(v).iter().nth(0).unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY09_EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_day9_get_next_value() {
        assert_eq!(extrapolate_right(&vec![0, 3, 6, 9, 12, 15]), vec![0, 3, 6, 9, 12, 15, 18]);
    }

    #[test]
    fn test_day9_part1() {
        let input = parse_input(DAY09_EXAMPLE);
        assert_eq!(sum_next_values(&input), 114);
    }

    #[test]
    fn test_day9_get_previous_value() {
        assert_eq!(extrapolate_left(&vec![10, 13, 16, 21, 30, 45]), vec![5, 10, 13, 16, 21, 30, 45]);
    }

    #[test]
    fn test_day9_part2() {
        let input = parse_input(DAY09_EXAMPLE);
        assert_eq!(sum_first_values(&input), 2);
    }
}
