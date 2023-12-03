use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use crate::utils::Grid;

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Grid<char> {
    let vec: Vec<Vec<char>> = input
        .lines()
        .map(|s| {
            s.chars()
                .collect()
        })
        .collect();

    let width = vec.first().unwrap().len();
    let cells = vec.into_iter().flatten().collect::<Vec<char>>();
    Grid::new(&cells, width)
}

pub fn has_part_neighbors(grid: &Grid<char>, pos: (usize, usize)) -> bool {
    grid.neighbors8_at(pos.0 as i32, pos.1 as i32)
        .iter()
        .any(|(c,pos)| {
            *c != '.' && !c.is_numeric()
        })
}

pub fn get_part_numbers(grid: &Grid<char>) -> Vec<u64> {
    //Vec::from([467, 35, 633, 617, 592, 755, 664, 598])
    let mut ret = Vec::new();
    let mut current_part : u64 = 0;
    let mut is_part = false;
    for y in 0..grid.size().1 {
        for x in 0..grid.size().0 {
            let c = grid.cell_at(x as i32, y as i32).unwrap();
            if c.is_numeric() {
                current_part = current_part * 10 + c.to_digit(10).unwrap() as u64;
                is_part = is_part || has_part_neighbors(grid, (x, y));
            }
            else {
                if is_part {
                    ret.push(current_part);
                }
                current_part = 0;
                is_part = false; 
            }
        }
        // check once again when end of line
        if is_part {
            ret.push(current_part);
        }
        current_part = 0;
        is_part = false; 
    }
    ret
}

#[aoc(day3, part1)]
pub fn sum_part_numbers(grid: &Grid<char>) -> u64 {
    get_part_numbers(grid).iter().fold(0, |sum, n| sum + n)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY03_EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";


    #[test]
    fn test_day3_get_numbers() {
        let input = parse_input(DAY03_EXAMPLE);
        assert_eq!(get_part_numbers(&input), [467, 35, 633, 617, 592, 755, 664, 598]);
    }

    #[test]
    fn test_day3_part1() {
        let input = parse_input(DAY03_EXAMPLE);
        assert_eq!(sum_part_numbers(&input), 4361);
    }

}
