use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::BTreeSet;

#[derive(Clone)]
pub struct World {
    pub rounds : BTreeSet<(usize, usize)>,
    pub cubes : BTreeSet<(usize, usize)>,
    pub size: (usize, usize)
}

impl World {
    pub fn simulate(&mut self) -> usize {
        let mut new_rounds = BTreeSet::new();
        let mut changes = 0;
        for r in self.rounds.iter() {
            let x = r.0;
            let y = r.1;
            if y > 0 && !self.rounds.contains(&(x, y-1)) && !self.cubes.contains(&(x, y-1)) {
                new_rounds.insert((x, y-1));
                changes += 1;
            }
            else {
                new_rounds.insert((x, y));
            }
        }
        self.rounds = new_rounds;
        changes
    }

    pub fn println(&self) {
        let mut buffer = "".to_string();
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let mut c = '.';
                if self.rounds.contains(&(x, y)) {
                    c = 'O';
                }
                if self.cubes.contains(&(x, y)) {
                    c = '#';
                }
                buffer.push_str(&c.to_string());
            }
            buffer.push('\n');
        }
        println!("{}", buffer)
    }
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> World  {
    let vec : Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let width = vec.first().unwrap().len();
    let height = vec.len();
    let mut rounds = BTreeSet::new();
    let mut cubes = BTreeSet::new();
    let mut y = 0;
    for row in vec {
        let mut x = 0;
        for c in row {
            if c == 'O' {
                rounds.insert((x, y));
            }
            else if c == '#' {
                cubes.insert((x, y));
            }
            x += 1;
        }
        y += 1;
    }
    World { rounds: rounds, cubes: cubes, size: (width, height) }
}


#[aoc(day14, part1)]
pub fn sum_load(input: &World) -> u64 {
    let mut world = input.clone();
    while world.simulate() > 0 { 
    }
    let h = world.size.1;
    world.rounds.iter().map(|p| (h - p.1) as u64 ).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const DAY14_EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_day14_part1() {
        let input = parse_input(DAY14_EXAMPLE);
        assert_eq!(sum_load(&input), 136);
    }

}
