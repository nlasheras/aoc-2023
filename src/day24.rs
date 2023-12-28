use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use crate::utils::Vector;

type Point = Vector<i64>;

#[derive(Clone)]
pub struct Hailstone {
    pub position: Point,
    pub velocity: Point
}

impl Hailstone {
    fn from(input: &str) -> Hailstone {
        let mut parts = input.split(" @ ");
        let pos : Vec<i64> = parts.next().unwrap().split(", ").map(|s| s.trim().parse::<i64>().unwrap() ).collect();
        let vel : Vec<i64> = parts.next().unwrap().split(", ").map(|s| s.trim().parse::<i64>().unwrap()).collect();
        Hailstone{ position: Point::new_3d(pos[0], pos[1], pos[2]), velocity: Point::new_3d(vel[0], vel[1], vel[2]) }
    }
}

#[aoc_generator(day24)]
pub fn parse_input(input: &str) -> Vec<Hailstone> {
    input.lines().map(Hailstone::from).collect()
}

fn find_crossing_2d(a: &Hailstone, b: &Hailstone) -> Option<(f64, f64)> {
    let dx = b.position.x - a.position.x;
    let dy = b.position.y - a.position.y;
    let det = b.velocity.x * a.velocity.y - b.velocity.y * a.velocity.x;
    if det == 0 {
        return None;
    }
    let u = (dy * b.velocity.x - dx * b.velocity.y) as f64 / det as f64;
    let v = (dy * a.velocity.x - dx * a.velocity.y) as f64 / det as f64;
    if u < 0.0 || v < 0.0 {
        return None; // cross in the past
    }
    Some((a.position.x as f64 + u*(a.velocity.x as f64), a.position.y as f64 + u*(a.velocity.y as f64)))
}

pub fn count_crossing(entries: &[Hailstone], min: i64, max: i64) -> u64 {
    let mut count = 0;
    for i in 0..entries.len() {
        for j in i+1..entries.len() {
            if let Some(p) = find_crossing_2d(&entries[i], &entries[j]) {
                if p.0 >= min as f64  && p.0 <= max as f64 &&
                   p.1 >= min as f64  && p.1 <= max as f64 {
                    count += 1;
                   }
            }
        }
    }

    count
}

#[aoc(day24, part1)]
pub fn count_crossing_part1(entries: &[Hailstone]) -> u64 {
    count_crossing(entries, 200000000000000, 400000000000000)
}

#[allow(dead_code)]
fn find_crossing_3d(a: &Hailstone, b: &Hailstone) -> Option<(f64, f64, f64)> {
    let da = a.velocity;
	let db = b.velocity;
	let dc = b.position - a.position;
	
    let div = da.cross(&db).length_sq() as f64;
    if div == 0.0 {
        return None;
    }
	let s = dc.cross(&db).dot(&da.cross(&db)) as f64 / div;
	if s >= 0.0 {
        let x = a.position.x as f64 + a.velocity.x as f64 * s;
        let y = a.position.y as f64 + a.velocity.y as f64 * s;
        let z = a.position.z as f64 + a.velocity.z as f64 * s;
		return Some((x, y, z))
	}
	None
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY24_EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_day24_part1() {
        let input = parse_input(DAY24_EXAMPLE);
        assert_eq!(count_crossing(&input, 7, 27), 2);
    }

    #[test]
    fn test_day24_crossing3d() {
        let h0 = Hailstone::from("19, 13, 30 @ -2,  1, -2");
        let res = Hailstone::from("24, 13, 10 @ -3, 1, 2");
        assert_eq!(find_crossing_3d(&h0, &res), Some((9f64, 18f64, 20f64)));
    }
}
