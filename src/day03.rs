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

pub fn has_part_neighbors(grid: &Grid<char>, pos: (i32, i32)) -> bool {
    grid.neighbors8_at(pos.0, pos.1)
        .iter()
        .any(|(c, _)| {
            *c != '.' && !c.is_numeric()
        })
}


#[derive(Clone)]
pub struct Part {
    pub number : u64,
    pub pos : (i32, i32)
}

pub fn get_parts(grid: &Grid<char>) -> Vec<Part> {
    let mut ret = Vec::new();
    let mut current_part : u64 = 0;
    let mut is_part = false;
    for y in 0..grid.size().1 as i32 {
        for x in 0..grid.size().0 as i32 {
            let c = grid.cell_at(x, y).unwrap();
            if c.is_numeric() {
                current_part = current_part * 10 + c.to_digit(10).unwrap() as u64;
                is_part = is_part || has_part_neighbors(grid, (x, y));
            }
            else {
                if is_part {
                    let startx = x - current_part.ilog10() as i32 - 1;
                    ret.push(Part{number: current_part, pos:(startx, y)});
                }
                current_part = 0;
                is_part = false; 
            }
        }
        // check once again when end of line
        if is_part {
            let startx = grid.size().0 as i32 - 1 - current_part.ilog10() as i32 - 1;
            ret.push(Part{number: current_part, pos:(startx, y)});
        }
        current_part = 0;
        is_part = false; 
    }
    ret
}

pub fn get_part_numbers(grid: &Grid<char>) -> Vec<u64> {
    get_parts(grid).iter().map(|p| p.number).collect()
}

impl Part {
    pub fn is_adjacent(&self, pos: (i32, i32)) -> bool {
        let w = self.number.ilog10() as i32;
        pos.1 >= self.pos.1 - 1 && pos.1 <= self.pos.1 + 1 &&
        pos.0 >= self.pos.0 - 1 && pos.0 <= self.pos.0 + w + 1
    }
}
pub fn get_gears(grid: &Grid<char>) -> Vec<(u64, u64)> {
    let mut gears = Vec::new();
    let parts = get_parts(grid);
    for y in 0..grid.size().1 as i32 {
        for x in 0..grid.size().0 as i32 {
            let c = grid.cell_at(x, y).unwrap();
            if c == '*' {
                let adjacent_parts : Vec<&Part> = parts.iter()
                    .filter(|p| p.is_adjacent((x, y))).collect();
                if adjacent_parts.len() == 2 {
                    gears.push((adjacent_parts.iter().nth(0).unwrap().number,
                                adjacent_parts.iter().nth(1).unwrap().number));
                }
            }
        }
    }
    gears
}

#[aoc(day3, part1)]
pub fn sum_part_numbers(grid: &Grid<char>) -> u64 {
    get_part_numbers(grid).iter().fold(0, |sum, n| sum + n)
}

#[aoc(day3, part2)]
pub fn sum_gear_ratios(grid: &Grid<char>) -> u64 {
    get_gears(grid).iter().fold(0, |sum, g| sum + g.0 * g.1)
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

    #[test]
    fn test_day3_part2() {
        let input: Grid<char> = parse_input(DAY03_EXAMPLE);
        assert_eq!(sum_gear_ratios(&input), 467835);
    }

}
