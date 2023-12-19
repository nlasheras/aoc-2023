use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use crate::utils::Grid;

use priority_queue::DoublePriorityQueue;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[aoc_generator(day17)]
pub fn parse_input(input: &str) -> Grid<usize>  {
    Grid::from(input, |c| c.to_string().parse::<usize>().unwrap())
}

fn heat_loss_path(input: &Grid<usize>, path: &Vec<(i32, i32)>) -> u64 {
    path.iter().skip(1).map(|(x,y)| input.cell_at(*x, *y).unwrap() as u64).sum::<u64>()
}


fn print_path(input: &Grid<usize>, path: &Vec<(i32, i32)>) {
    let mut buffer = "".to_string();
    for y in 0..input.size().1 {
        for x in 0..input.size().0 {
            let cell = input.cell_at(x as i32, y as i32).unwrap();
            if let Some(pos) = path.iter().position(|p| p.0 == x as i32 && p.1 == y as i32) {
                if pos == 0 {
                    buffer.push_str(&cell.to_string());
                } else {
                    let dir = (path[pos].0 - path[pos-1].0, path[pos].1 - path[pos-1].1);
                    let char = match dir {
                        (1, 0) => '>',
                        (-1, 0) => '<',
                        (0, 1) => 'v',
                        (0, -1) => '^',
                        _ => panic!()
                    };
                    buffer.push(char);
                }
            } else {
                buffer.push_str(&cell.to_string());
            }
        }
        buffer.push('\n');
    }
    println!("{}", buffer)
}


#[aoc(day17, part1)]
pub fn sum_heat_loss(input: &Grid<usize>) -> u64 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    const DAY17_EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_day17_part1() {
        let input = parse_input(DAY17_EXAMPLE);
        assert_eq!(sum_heat_loss(&input), 102);
    }

}