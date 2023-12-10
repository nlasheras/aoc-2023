use core::panic;
use std::collections::BTreeMap;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use crate::utils::Grid;

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Grid<char>  {
    let vec: Vec<Vec<char>> = input
        .lines()
        .map(|s| {
            s.chars().collect()
        })
        .collect();

    let width = vec.first().unwrap().len();
    let cells = vec.into_iter().flatten().collect::<Vec<char>>();
    Grid::new(&cells, width)
}

pub fn find_start(input: &Grid<char>) -> (usize, usize) {
    let size = input.size();
    for y in 0..size.1 {
        for x in 0..size.0 {
            if input.cell_at(x as i32, y as i32).unwrap() == 'S' {
                return (x, y)
            }
        }
    }
    panic!()
}

pub fn get_pipe_neighbors(input: &Grid<char>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let c = input.cell_at(pos.0 as i32, pos.1 as i32).unwrap();
    let ns : Vec<(i32, i32)> = match c {
        '|' => vec![(0, -1), (0, 1)],
        '-' => vec![(-1, 0), (1, 0)],
        'L' => vec![(1, 0), (0, -1),],
        'J' => vec![(-1, 0), (0, -1),],
        '7' => vec![(-1, 0), (0, 1)],
        'F' => vec![(1, 0), (0, 1)],
        '.' => vec![],
        'S' => vec![(0, -1), (0, 1), (-1, 0), (1, 0)],
        _ => panic!()
    };
    let ret : Vec<(usize, usize)> = ns.iter()
    .map(|n| (n.0 + pos.0 as i32, n.1 + pos.1 as i32))
    .filter(|n| {
        if let Some(c) = input.cell_at(n.0, n.1) {
            return c != '.'
        }
        false
    })
    .map(|n| (n.0 as usize, n.1 as usize))
    .collect();
    if c == 'S' {
        return ret.iter()
            .filter(|n| get_pipe_neighbors(input, n).contains(pos))
            .map(|n| n.clone())
            .collect();
    }
    ret
}

#[aoc(day10, part1)]
pub fn farthest_point(input: &Grid<char>) -> u64 {
    let start = find_start(input);
    let mut candidates : Vec<(usize, usize)> = vec![start];
    let mut dist : Grid<u64> = Grid::new(&vec![u64::MAX; input.size().0 * input.size().1], input.size().0);
    dist.set_at(start.0, start.1, 0);
    let mut come_from : BTreeMap<(usize, usize), (usize, usize)> = BTreeMap::new();
    while !candidates.is_empty() {
        let c = candidates.pop().unwrap();
        let dist_c = dist.cell_at(c.0 as i32, c.1 as i32).unwrap();
        let ns = get_pipe_neighbors(input, &c);
        for n in ns {
            let dist_n = dist.cell_at(n.0 as i32, n.1 as i32).unwrap();
            if dist_n <= dist_c + 1 {
                continue;
            }

            candidates.push(n);
            come_from.entry(n).and_modify(|e| *e = n );
            dist.set_at(n.0, n.1, dist_c + 1);
        }
    }
    *dist.cells.iter().filter(|n| **n != u64::MAX).max().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    const DAY10_EXAMPLE1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    #[test]
    fn test_day10_find_start() {
        let input = parse_input(DAY10_EXAMPLE1);
        assert_eq!(find_start(&input), (1, 1));
    }

    #[test]
    fn test_day10_example1() {
        let input = parse_input(DAY10_EXAMPLE1);
        assert_eq!(farthest_point(&input), 4);
    }

    const DAY10_EXAMPLE2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_day10_example2() {
        let input = parse_input(DAY10_EXAMPLE2);
        assert_eq!(farthest_point(&input), 8);
    }
}
