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

#[aoc(day23, part1)]
pub fn find_longest_path(input: &Grid<char>) -> u64 {
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

 

}