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

pub fn get_shortest_distances(input: &Grid<char>) -> Grid<u64> {
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
    return dist
}

#[aoc(day10, part1)]
pub fn farthest_point(input: &Grid<char>) -> u64 {
    let dist = get_shortest_distances(input);
    *dist.cells.iter().filter(|n| **n != u64::MAX).max().unwrap()
}

pub fn is_boundary_horizontal(input: &Grid<char>, pos: &(usize, usize)) -> bool {
    // for checking the loop left to right we only care of F and 7 (clockwise turns)
    let ns = get_pipe_neighbors(input, pos);
    ns.contains(&(pos.0, pos.1 + 1))
}

#[aoc(day10, part2)]
pub fn count_inside_loop(input: &Grid<char>) -> u64 {
    let dist = get_shortest_distances(input);
    let size = input.size();
    let mut render : Grid<char> = Grid::new(&vec!['.'; input.size().0 * input.size().1], input.size().0);
    for y in 0..size.1 {
        let mut is_inside = false;
        for x in 0..size.0 {
            if dist.cell_at(x as i32, y as i32).unwrap() == u64::MAX {
                render.set_at(x, y, if is_inside { 'I' } else { 'O' });
            } else if is_boundary_horizontal(input, &(x,y)) {
                is_inside = !is_inside;
            }
        }
    }
    render.cells.iter().filter(|c| **c == 'I').count() as u64
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

    const DAY10_EXAMPLE3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    #[test]
    fn test_day10_example3() {
        let input = parse_input(DAY10_EXAMPLE3);
        assert_eq!(count_inside_loop(&input), 4);
    }

    const DAY10_EXAMPLE4: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    #[test]
    fn test_day10_example4() {
        let input = parse_input(DAY10_EXAMPLE4);
        assert_eq!(count_inside_loop(&input), 4);
    }

    const DAY10_EXAMPLE5: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    #[test]
    fn test_day10_example5() {
        let input = parse_input(DAY10_EXAMPLE5);
        assert_eq!(count_inside_loop(&input), 8);
    }

    const DAY10_EXAMPLE6: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    
        #[test]
        fn test_day10_example6() {
            let input = parse_input(DAY10_EXAMPLE6);
            assert_eq!(count_inside_loop(&input), 10);
        }
    
         
}
