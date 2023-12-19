use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

use crate::utils::Point;

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
        let color = parts[2].to_string()[2..parts[2].len()-1].to_string();
        DigOrder { dir : dir, meters: meters, color: color }
    }
}

#[aoc_generator(day18)]
pub fn parse_input(input: &str) -> Vec<DigOrder>  {
    input.lines().map(|s| DigOrder::from(s)).collect()
}

pub fn count_cubic_meters_interior(input: &Vec<DigOrder>) -> u64 {
    let mut current = Point::new(0, 0);
    let mut points = vec![current];

    for order in input {
        let dir = match order.dir.as_str() {
            "D" => Point::new(0, 1),
            "R" => Point::new(1, 0),
            "L" => Point::new(-1, 0),
            "U" => Point::new(0, -1),
            _ => panic!()
        };
        current = current + Point::new(order.meters * dir.x, order.meters * dir.y);
        points.push(current);
    }

    // Shoelace formula for trapezoids
    let pairs : Vec<(&Point, &Point)> = points.iter().tuple_windows().collect();
    let mut sum : i64 = 0;
    for (p0, p1) in pairs {
        sum += (p0.y + p1.y) as i64 * (p0.x - p1.x) as i64;
    }
    let inside = (sum / 2) as u64;

    // Pick's theorem A = i + b/2 + 1
    let boundary = input.iter().map(|o| o.meters as u64).sum::<u64>();
    inside + boundary/2 + 1

}


#[aoc(day18, part1)]
pub fn count_cubic_meters_part1(input: &Vec<DigOrder>) -> u64 {
    count_cubic_meters_interior(input)
}

fn parse_color(input: &str) -> (String, i32) {
    let meters = i32::from_str_radix(&input[0..input.len()-1], 16).unwrap();
    let dir = match &input[input.len()-1..input.len()] {
         "0" => "R",
         "1" => "D",
         "2" => "L",
         "3" => "U",
         _ => panic!()
    };
    (dir.to_string(), meters)
}

#[aoc(day18, part2)]
pub fn count_cubic_meters_part2(input: &Vec<DigOrder>) -> u64 {
    let fixed_orders = input.iter().map(|o| {
        let (new_dir, new_meters) = parse_color(&o.color);
        DigOrder { dir: new_dir.clone(), color: o.color.clone(), meters: new_meters }
    }).collect::<Vec<DigOrder>>();
    count_cubic_meters_interior(&fixed_orders)
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
        assert_eq!(count_cubic_meters_part1(&input), 62);
    }

    #[test]
    fn test_day18_part2() {
        let input = parse_input(DAY18_EXAMPLE);
        assert_eq!(count_cubic_meters_part2(&input), 952408144115);
    }

}