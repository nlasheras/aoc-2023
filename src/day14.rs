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
    pub fn simulate(&mut self, delta: (i32, i32)) -> usize {
        let mut new_rounds = BTreeSet::new();
        let mut changes = 0;
        for r in self.rounds.iter() {

            let nx = r.0 as i32 + delta.0;
            let ny = r.1 as i32 + delta.1;

            if ny < 0 || ny>= self.size.1 as i32 || nx < 0 || nx >= self.size.0 as i32 {
                new_rounds.insert(*r);
                continue;
            }

            let moved_r = (nx as usize, ny as usize);
            if !self.rounds.contains(&moved_r) && !self.cubes.contains(&moved_r) {
                new_rounds.insert(moved_r);
                changes += 1;
            }
            else {
                new_rounds.insert(*r);
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
    while world.simulate((0, -1)) > 0 { 
    }
    let h = world.size.1;
    world.rounds.iter().map(|p| (h - p.1) as u64 ).sum()
}

pub fn find_cycle(input: &Vec<u64>) -> Option<(usize, usize)> {
    for window_size in 4..input.len()/2 {
        let max_offset = input.len() - window_size*2;
        for offset in 0..max_offset+1 {
            let mut found = true;
            for i in 0..window_size {
                if input[offset+i] != input[offset+window_size+i] {
                    found = false;
                    break;
                }
            }
            if found {
                return Some((offset, window_size));
            }
        }
    }
    None
}

#[aoc(day14, part2)]
pub fn sum_load_with_spins(input: &World) -> u64 {
    let cycles = 1000000000;
    let mut world = input.clone();
    let mut loads = Vec::new();
    for _ in 0..cycles {
        while world.simulate((0, -1)) > 0 {}
        while world.simulate((-1, 0)) > 0 {}
        while world.simulate((0, 1)) > 0 {}
        while world.simulate((1, 0)) > 0 {}
        let load: u64 = world.rounds.iter().map(|p| (world.size.1 - p.1) as u64 ).sum();
        loads.push(load);
        if let Some((start, window)) = find_cycle(&loads) {
            let idx = (cycles - start) % window + start - 1;
            return loads[idx];
        }
    }
    world.rounds.iter().map(|p| (world.size.1 - p.1) as u64 ).sum()
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

    #[test]
    fn test_day14_part2() {
        let input = parse_input(DAY14_EXAMPLE);
        assert_eq!(sum_load_with_spins(&input), 64);
    }
    
    #[test]
    fn test_find_cycle() {
        let example = vec![87, 69, 69, 69, 65, 64, 65, 63, 68, 69, 69, 65, 64, 65, 63, 68, 69, 69, 65, 64, 65, 63, 68, 69, 69, 65, 64, 65, 63, 68, 69, 69, 65, 64, 65, 63, 68, 69, 69, 65, 64];
        assert_eq!(find_cycle(&example), Some((2, 7)));
    }

}
