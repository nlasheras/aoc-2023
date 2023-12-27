use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use crate::utils::Grid;

#[aoc_generator(day23)]
pub fn parse_input(input: &str) -> Grid<char> {
    Grid::from(input, |c| c)
}

fn find_start_end(input: &Grid<char>) -> ((i32, i32), (i32, i32)) {
    let mut startx = 0;
    for x in 0..input.width() {
        if let Some('.') = input.cell_at(x as i32, 0) {
            startx = x as i32;
            break;
        }
    }
    let endy = (input.height() - 1) as i32;
    let mut endx = 0;
    for x in (0..input.width()).rev() {
        if let Some('.') = input.cell_at(x as i32, endy) {
            endx = x as i32;
            break;
        }   
    }
    ((startx as i32, 0), (endx as i32, endy as i32))
}

use std::collections::VecDeque;
use std::cmp;

#[aoc(day23, part1)]
pub fn find_longest_path(input: &Grid<char>) -> u64 {
    let (start, end) = find_start_end(input);
    let mut open_set = VecDeque::new();
    open_set.push_back(vec![start]);

    let mut closed_set = Vec::new();
    while !open_set.is_empty() {
        let path = open_set.pop_front().unwrap();
        let current = path[path.len() - 1];

        if current == end {
            //println!("path len {}", path.len());
            closed_set.push(path);
            continue;
        }

        for (c, upos) in input.neighbors_at(current.0, current.1) {
            let pos = (upos.0 as i32, upos.1 as i32);
            if c == '#' {
                continue;
            }
            if path.contains(&pos) {
                continue;
            }
            let mut new_path = path.clone();
            match c {
                '.' => new_path.push(pos),
                '>' => new_path.append(&mut vec![pos, (pos.0+1, pos.1)]),
                '<' => new_path.append(&mut vec![pos, (pos.0-1, pos.1)]),
                '^' => new_path.append(&mut vec![pos, (pos.0, pos.1-1)]),
                'v' => new_path.append(&mut vec![pos, (pos.0, pos.1+1)]),
                _ => panic!()
            }
            if c != '.' {
                if path.contains(&new_path[new_path.len()-1]) {
                    continue;
                }
            }
            open_set.push_back(new_path);
        }
    }

    if !closed_set.is_empty() {
        return closed_set.iter().fold(0, |acc, v| cmp::max(acc, v.len() - 1)) as u64
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY23_EXAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_day23_find_start_end() {
        let input = parse_input(DAY23_EXAMPLE);
        assert_eq!(find_start_end(&input), ((1, 0), (21, 22)));
    }

    #[test]
    fn test_day23_longest_path() {
        let input = parse_input(DAY23_EXAMPLE);
        assert_eq!(find_longest_path(&input), 94);
    }

}