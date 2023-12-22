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

pub fn count_steps(input: &Grid<char>, steps: usize) -> u64 {
    let start = find_start(input);
    let mut open_set = BTreeSet::new();
    open_set.insert(start);
    for i in 0..steps {
        let mut new_set : BTreeSet<(i32, i32)> = BTreeSet::new();
        for pos in open_set.iter() {
            for (c, npos) in input.neighbors_at(pos.0 as i32, pos.1 as i32) {
                if c != '#' {
                    new_set.insert((npos.0 as i32, npos.1 as i32));
                }
            }
        }
        open_set = new_set;
    }
    open_set.iter().count() as u64
}

#[aoc(day21, part1)]
pub fn count_64steps(input: &Grid<char>) -> u64 {
    count_steps(input, 64)
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
        assert_eq!(count_steps(&input, 1), 2);
    }

    #[test]
    fn test_day21_6step() {
        let input = parse_input(DAY21_EXAMPLE);
        assert_eq!(count_steps(&input, 6), 16);
    }

}