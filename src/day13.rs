use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use crate::utils::Grid;

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> Vec<Grid<char>>  {
    input.split("\n\n")
        .map(|s| {
            let vec : Vec<Vec<char>> = s.lines().map(|s| s.chars().collect()).collect();
            let width = vec.first().unwrap().len();
            let cells = vec.into_iter().flatten().collect::<Vec<char>>();
            Grid::new(&cells, width)
        })
        .collect()
}

pub fn has_horizontal_reflect(input: &Grid<char>, column: i32, required_errors: usize) -> bool {
    let width = input.size().0 as i32;
    let height = input.size().1 as i32;
    let mut errors = 0;
    for x in 0..=column {
        let mx = column * 2 - x + 1;
        if mx < 0 || mx >= width {
            continue
        }
        for y in 0..height {
            let c = input.cell_at(x, y).unwrap();
            let mc = input.cell_at(mx, y).unwrap();
            if c != mc {
                errors += 1;
                if errors > required_errors {
                    return false;
                }
            }
        }
    }
    errors == required_errors
}

pub fn has_vertical_reflect(input: &Grid<char>, column: i32, required_errors: usize) -> bool {
    let width = input.size().0 as i32;
    let height = input.size().1 as i32;
    let mut errors = 0;
    for y in 0..=column {
        let my = column * 2 - y + 1;
        if my < 0 || my >= height {
            continue;
        }
        for x in 0..width {
            let c = input.cell_at(x, y).unwrap();
            let mc = input.cell_at(x, my).unwrap();
            if c != mc {
                errors += 1;
                if errors > required_errors {
                    return false;
                }
            }
        }
    }
    errors == required_errors
}

pub fn find_reflection(input: &Grid<char>, required_errors: usize) -> (usize, usize) {
    let size = input.size();
    let mut rx = 0;
    let mut ry = 0;
    for tx in 0..(size.0-1) {
        if has_horizontal_reflect(input, tx as i32, required_errors) {
            rx = tx + 1;
            break;
        }
    }
    for ty in 0..(size.1-1) {
        if has_vertical_reflect(input, ty as i32, required_errors) {
            ry = ty + 1;
            break;
        }
    }
    if rx != 0 && ry != 0 {
        if rx < ry {
            return (rx, 0)
        }
        else if rx > ry {
            return (0, ry)
        }
        panic!()
    }
    (rx, ry)
}

#[aoc(day13, part1)]
pub fn sum_reflections(input: &Vec<Grid<char>>) -> u64 {
    input.iter().map(|g| {
        let r = find_reflection(g, 0);
        (r.1 * 100 + r.0) as u64
    }).sum()
}


#[aoc(day13, part2)]
pub fn sum_reflections_with_1error(input: &Vec<Grid<char>>) -> u64 {
    input.iter().map(|g| {
        let r = find_reflection(g, 1);
        (r.1 * 100 + r.0) as u64
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY13_EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_day13_example1() {
        let input = parse_input(DAY13_EXAMPLE);
        assert_eq!(find_reflection(&input[0], 0), (5, 0));
    }

    #[test]
    fn test_day13_example2() {
        let input = parse_input(DAY13_EXAMPLE);
        assert_eq!(find_reflection(&input[1], 0), (0, 4));
    }

    #[test]
    fn test_day13_part1() {
        let input = parse_input(DAY13_EXAMPLE);
        assert_eq!(sum_reflections(&input), 405);
    }

    const DAY13_FAIL_INPUT1: &str = "..##......##.
..##..##..##.
......##.....
..###.##.###.
..###....###.
###.##..##.#.
###..####..##
###........##
####......###";

    #[test]
    fn test_day13_fail_input_1() {
        let input = parse_input(DAY13_FAIL_INPUT1);
        assert_eq!(find_reflection(&input[0], 0), (1, 0));
    }

    #[test]
    fn test_day13_part2_example1() {
        let input = parse_input(DAY13_EXAMPLE);
        assert_eq!(find_reflection(&input[0], 1), (0, 3));
    }

    #[test]
    fn test_day13_part2_example2() {
        let input = parse_input(DAY13_EXAMPLE);
        assert_eq!(find_reflection(&input[1], 1), (0, 1));
    }

    #[test]
    fn test_day13_part2() {
        let input = parse_input(DAY13_EXAMPLE);
        assert_eq!(sum_reflections_with_1error(&input), 400);
    }

}
