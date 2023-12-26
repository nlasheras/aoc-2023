use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use crate::utils::Point;

#[derive(Clone)]
pub struct Brick {
    pub id: usize,
    pub start: Point,
    pub end: Point
}

impl Brick {
    fn from(input: &str) -> Brick {
        let points = input.split("~").map(|s| {
            let ns = s.split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            Point::new_3d(ns[0], ns[1], ns[2])
        }).collect::<Vec<Point>>();
        Brick{ id: 0, start: points[0], end: points[1]  }
    }

    fn contains(&self, point: &Point) -> bool {
        point.x >= self.start.x && point.x <= self.end.x && 
        point.y >= self.start.y && point.y <= self.end.y && 
        point.z >= self.start.z && point.z <= self.end.z
    }

    pub fn collide(&self, other: &Brick) -> bool {
        self.contains(&other.start) || self.contains(&other.end) || 
        other.contains(&self.start) || other.contains(&self.end)
    }

    pub fn floor_z(&self) -> i32 {
        self.start.z
    }
}

#[aoc_generator(day22)]
pub fn parse_input(input: &str) -> Vec<Brick> {
    let mut bs = input.lines().map(|l| Brick::from(l)).collect::<Vec<Brick>>();
    for i in 0..bs.len() {
        bs[i].id = i;
    }
    bs
}

fn fall_bricks_once(input: &Vec<Brick>) -> (usize, Vec<Brick>) {
    let mut ret = Vec::new();
    let mut changed = 0;
    for b in input.iter() {
        if b.floor_z() == 1 {
            ret.push(b.clone());
            continue;
        }

        let mut test = b.clone();
        test.start.z -= 1;
        test.end.z -= 1;

        if input.iter().any(|b| b.id != test.id && b.collide(&test)) {
            ret.push(b.clone());
        }
        else {
            ret.push(test);
            changed += 1;
        }
    }
    (changed, ret)
}

fn fall_bricks_until_stable(input: &Vec<Brick>) -> Vec<Brick> {
    let (mut changed, mut stable) = fall_bricks_once(input);
    while changed > 0 {
        (changed, stable) = fall_bricks_once(&stable)
    }
    stable
}

#[aoc(day22, part1)]
pub fn count_disintegrable_bricks(input: &Vec<Brick>) -> u64 {
    let stable = fall_bricks_until_stable(input);
    
    let mut count = 0;
    for i in 0..stable.len() {
        let mut test = stable.clone();
        test.remove(i);

        let (changed, _) = fall_bricks_once(&test);
        if changed == 0 {
            //println!("brick {} can be disintegrated", i);
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY22_EXAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_day22_part1() {
        let input = parse_input(DAY22_EXAMPLE);
        assert_eq!(count_disintegrable_bricks(&input), 5);
    }

 
}