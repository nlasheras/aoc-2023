use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

pub struct DigOrder {
    pub dir : String,
    pub meters : i32,
    pub color : String
}

impl DigOrder {
    fn from(input: &str) -> DigOrder {
        let parts : Vec<&str> = input.split_whitespace().collect();
        let dir = parts[0].to_string();
        let meters = parts[1].parse::<i32>().unwrap();
        let color = parts[2].to_string();
        DigOrder { dir : dir, meters: meters, color: color }
    }
}

#[aoc_generator(day18)]
pub fn parse_input(input: &str) -> Vec<DigOrder>  {
    input.lines().map(|s| DigOrder::from(s)).collect()
}


#[aoc(day18, part1)]
pub fn count_cubic_meters_interior(input: &Vec<DigOrder>) -> u64 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    const DAY18_EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_day18_part1() {
        let input = parse_input(DAY18_EXAMPLE);
        assert_eq!(count_cubic_meters_interior(&input), 62);
    }

}