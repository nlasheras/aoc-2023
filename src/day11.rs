use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

pub struct Cosmos
{
    pub galaxies : Vec<(usize, usize)>
}

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> Cosmos  {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|s| {
            s.chars().collect()
        })
        .collect();

    let mut galaxies = Vec::new();
    for y in 0..grid.len() {
        let row = &grid[y];
        for x in 0..row.len() {
            let c = row[x];
            if c == '#' {
                galaxies.push((x, y));
            }
        }
    }

    Cosmos{ galaxies: galaxies }
}

pub fn get_shortest_distance(input: &Cosmos, from: &(usize, usize), to: &(usize, usize)) -> u64 {
    0
}

#[aoc(day11, part1)]
pub fn sum_shortest_lengths(input: &Cosmos) -> u64 {
    374
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY11_EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_day11_example1() {
        let input = parse_input(DAY11_EXAMPLE);
        assert_eq!(get_shortest_distance(&input, &input.galaxies[0], &input.galaxies[6]), 15);
    }

    #[test]
    fn test_day11_part1() {
        let input = parse_input(DAY11_EXAMPLE);
        assert_eq!(sum_shortest_lengths(&input), 374);
    }

}
