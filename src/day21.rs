use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::BTreeSet;

use crate::utils::Grid;

#[aoc_generator(day21)]
pub fn parse_input(input: &str) -> Grid<char> {
    Grid::from(input, |c| c)
}

fn find_start(input: &Grid<char>) -> (i32, i32) {
    for y in 0..input.height() {
        for x in 0..input.width() {
            if input.cell_at(x as i32, y as i32) == Some('S') {
                return (x as i32, y as i32)
            }
        }
    }
    panic!()
}

pub fn get_neighbors(input: &Grid<char>, pos: &(i32, i32), infinite: bool) -> Vec<(char, (i32, i32))> {
    if infinite {
        let mut ret = Vec::new();
        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let npos = (pos.0 + dir.0, pos.1 + dir.1);
            let wrapped_pos = (npos.0.rem_euclid(input.width() as i32), npos.1.rem_euclid(input.height() as i32));
            let c = input.cell_at(wrapped_pos.0, wrapped_pos.1).unwrap();
            ret.push((c, npos));
        }
        return ret;
    } else {
        return input.neighbors_at(pos.0, pos.1).iter().map(|(c, p)| (*c, (p.0 as i32, p.1 as i32))).collect();
    }
}

pub fn count_steps(input: &Grid<char>, steps: usize, infinite: bool) -> u64 {
    let start = find_start(input);
    let mut open_set = BTreeSet::new();
    open_set.insert(start);
    for i in 0..steps {
        let mut new_set : BTreeSet<(i32, i32)> = BTreeSet::new();
        for pos in open_set.iter() {
            for (c, npos) in get_neighbors(input, pos, infinite) {
                if c != '#' {
                    new_set.insert(npos);
                }
            }
        }
        if infinite && ((i+1) % input.width() == 0){
            println!("{},{}", i+1, new_set.iter().count());
        }
        open_set = new_set;
    }
    open_set.iter().count() as u64
}

#[aoc(day21, part1)]
pub fn count_64steps(input: &Grid<char>) -> u64 {
    count_steps(input, 64, false)
}

#[aoc(day21, part2)]
pub fn print_64periods(input: &Grid<char>) -> u64 {
    count_steps(input, input.width() * 10, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY21_EXAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_day21_find_start() {
        let input = parse_input(DAY21_EXAMPLE);
        assert_eq!(find_start(&input), (5, 5));
    }

    #[test]
    fn test_day21_1step() {
        let input = parse_input(DAY21_EXAMPLE);
        assert_eq!(count_steps(&input, 1, false), 2);
    }

    #[test]
    fn test_day21_6step() {
        let input = parse_input(DAY21_EXAMPLE);
        assert_eq!(count_steps(&input, 6, false), 16);
    }

    #[test]
    fn test_day21_part2() {
        let input = parse_input(DAY21_EXAMPLE);
        assert_eq!(count_steps(&input, 50, true), 1594);
    }

}