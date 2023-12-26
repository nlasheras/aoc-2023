use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use crate::utils::Point;

#[derive(Clone)]
pub struct Brick {
    pub id: usize,
    pub start: Point,
    pub end: Point,
    pub falling: bool
}

impl Brick {
    fn from(input: &str) -> Brick {
        let points = input.split("~").map(|s| {
            let ns = s.split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            Point::new_3d(ns[0], ns[1], ns[2])
        }).collect::<Vec<Point>>();
        Brick{ id: 0, start: points[0], end: points[1], falling: points[0].z != 1 }
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
    bs.sort_by(|b1, b2| b1.start.z.cmp(&b2.start.z));
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

use std::collections::HashMap;

#[aoc(day22, part1)]
pub fn count_disintegrable_bricks(input: &Vec<Brick>) -> u64 {
    let mut stable = fall_bricks_until_stable(input);
    for b in stable.iter_mut() {
        b.falling = b.start.z > 1;
    }

    let mut supports = HashMap::<usize, Vec<usize>>::new();
    let mut supported = HashMap::<usize, Vec<usize>>::new();
    for i in 0..stable.len() {
        let mut test = stable.clone();
        test.remove(i);

        let mut b = stable[i].clone();
        b.start.z -= 1;
        b.end.z -= 1;
        let bs = test.iter().filter(|b1| b1.collide(&b)).collect::<Vec<&Brick>>();
        for b2 in bs {
            let e = supports.entry(b2.id).or_insert(vec![]);
            e.push(b.id);
            let e2 = supported.entry(b.id).or_insert(vec![]);
            e2.push(b2.id);
        }
    }
    let mut count = 0;
    for b in input.iter() {
        let sup = supports.entry(b.id).or_default();
        if sup.iter().all(|b1| supported.entry(*b1).or_default().len() > 1) {
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

    #[test]
    fn test_day22_collide1() {
        let b1 = Brick::from("1,0,1~1,2,1");
        let b2 = Brick::from("0,0,1~2,0,1");
        assert!(b1.collide(&b2));
    }

    const DAY22_REDDIT1: &str = "0,0,1~0,1,1
1,1,1~1,1,1
0,0,2~0,0,2
0,1,2~1,1,2";

    #[test]
    fn test_day22_reddit1() {
        let input = parse_input(DAY22_REDDIT1);
        assert_eq!(count_disintegrable_bricks(&input), 3);
    }

    const DAY22_REDDIT2: &str = "0,0,1~1,0,1
0,1,1~0,1,2
0,0,5~0,0,5
0,0,4~0,1,4";

    #[test]
    fn test_day22_reddit2() {
        let input = parse_input(DAY22_REDDIT2);
        assert_eq!(count_disintegrable_bricks(&input), 2);
    }

    const DAY22_REDDIT3: &str = "0,0,1~0,0,1
1,1,1~1,1,1
0,0,2~0,1,2
0,1,3~1,1,3";

    #[test]
    fn test_day22_reddit3() {
        let input = parse_input(DAY22_REDDIT3);
        assert_eq!(count_disintegrable_bricks(&input), 2);
    }

}