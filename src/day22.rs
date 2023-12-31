use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::VecDeque;

use crate::utils::Point;

#[derive(Clone)]
pub struct Brick {
    pub id: usize,
    pub start: Point,
    pub end: Point,
}

impl Brick {
    fn from(input: &str) -> Brick {
        let points = input.split("~").map(|s| {
            let ns = s.split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            Point::new_3d(ns[0], ns[1], ns[2])
        }).collect::<Vec<Point>>();
        Brick{ id: 0, start: points[0], end: points[1] }
    }

    fn contains(&self, point: &Point) -> bool {
        point.x >= self.start.x && point.x <= self.end.x && 
        point.y >= self.start.y && point.y <= self.end.y && 
        point.z >= self.start.z && point.z <= self.end.z
    }

    pub fn collide(&self, other: &Brick) -> bool {
        for z in self.start.z..=self.end.z {
            if z > other.end.z || z < other.start.z {
                continue;
            }
            for x in self.start.x..=self.end.x {
                for y in self.start.y..=self.end.y {
                    if other.contains(&Point::new_3d(x, y, z)) {
                        return true;
                    }
                }
            }
        }
        false
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
    for idx in 0..input.len() {
        let b = &input[idx];
        if b.start.z == 1 {
            ret.push(b.clone());
            continue;
        }

        let mut test = b.clone();
        test.start.z -= 1;
        test.end.z -= 1;

        if input[idx+1..input.len()].iter().any(|b| b.id != test.id && b.collide(&test)) || ret.iter().any(|b| b.collide(&test)) {
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

pub struct BrickSupport {
    pub supports : HashMap<usize, Vec<usize>>,
    pub supported : HashMap<usize, Vec<usize>>
}

pub fn get_supports(input: &Vec<Brick>) -> BrickSupport {
    let stable = fall_bricks_until_stable(input);

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
    BrickSupport{ supported: supported, supports: supports }
}

impl BrickSupport {
    pub fn supports(&self, id: usize) -> Vec<usize> {
        if self.supports.contains_key(&id) {
            return self.supports.get(&id).unwrap().clone()
        }
        Vec::new()
    }

    pub fn supported_by(&self, id: usize) -> Vec<usize> {
        if self.supported.contains_key(&id) {
            return self.supported.get(&id).unwrap().clone()
        }
        Vec::new()
    }
}

#[aoc(day22, part1)]
pub fn count_disintegrable_bricks(input: &Vec<Brick>) -> u64 {
    let helper = get_supports(input);
    let mut count = 0;
    for b in input.iter() {
        let above = helper.supports(b.id);
        // we can safely desintegrate bricks that are not the single support of another brick
        if above.iter().all(|b1| helper.supported_by(*b1).len() > 1) {
            count += 1;
        }
    }
    count
}

#[aoc(day22, part2)]
pub fn count_chain_reaction(input: &Vec<Brick>) -> u64 {
    let helper = get_supports(input);
    let mut count = 0;
    for b in input.iter() {
        let above = helper.supports(b.id);
        if above.is_empty() {
            continue; // won't generate a chain reaction
        }
        let mut falling: BTreeSet<usize> = BTreeSet::new();
        let mut pending = VecDeque::new();
        pending.push_back(b.id);
        while !pending.is_empty() {
            let p = pending.pop_front().unwrap();
            let above = helper.supports(p);
            // in this version we keep checking all the nodes upwards, adding all nodes supported 
            // only by the brick or other bricks that we detect they are falling
            for a in above {
                let below_a = helper.supported_by(a);
                if below_a.iter().all(|x| *x == b.id || falling.contains(x)) {
                    if falling.insert(a) {
                        pending.push_back(a);
                    }
                }
            }
        }
        count += falling.len() as u64;
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
    fn test_day22_part2() {
        let input = parse_input(DAY22_EXAMPLE);
        assert_eq!(count_chain_reaction(&input), 7);
    }

    #[test]
    fn test_day22_collide1() {
        let b1 = Brick::from("1,0,1~1,2,1");
        let b2 = Brick::from("0,0,1~2,0,1");
        assert!(b1.collide(&b2));
    }

    #[test]
    fn test_day22_collide2() {
        let b1 = Brick::from("5,6,2~5,9,2");
        let b2 = Brick::from("3,7,2~6,7,2");
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