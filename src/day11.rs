use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

pub struct Cosmos
{
    pub galaxies : Vec<(usize, usize)>,
    pub size: (usize, usize)
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

    Cosmos{ galaxies: galaxies, size: (grid[0].len(), grid.len())}
}

pub fn expand(input: &Cosmos, factor: usize) -> Cosmos {
    let expand_rows : Vec<usize> = (0..input.size.1).filter(|n| {
        !input.galaxies.iter().any(|g| g.1 == *n)
    }).collect();
    let expand_cols : Vec<usize> = (0..input.size.0).filter(|n| {
        !input.galaxies.iter().any(|g| g.0 == *n)
    }).collect();
    let mut new_galaxies = Vec::new();
    let new_size = (input.size.0 + expand_cols.len(), input.size.1 + expand_rows.len());
    for g in &input.galaxies {
        let offset_cols = expand_cols.iter().filter(|n| **n < g.0).count();
        let offset_rows = expand_rows.iter().filter(|n| **n < g.1).count();
        new_galaxies.push((g.0 + offset_cols*(factor-1), g.1 + offset_rows*(factor-1)));
    }
    Cosmos { galaxies: new_galaxies, size: new_size }
}

pub fn get_shortest_distance(from: &(usize, usize), to: &(usize, usize)) -> u64 {
    // just the manhattan distance works for this since we are working over the expanded Cosmos
    (to.0.abs_diff(from.0) + to.1.abs_diff(from.1)) as u64
}

pub fn sum_shortest_lengths(input: &Cosmos, factor: usize) -> u64 {
    let expand = expand(input, factor);
    let mut sum = 0;
    for i in 0..expand.galaxies.len() {
        for j in i+1..expand.galaxies.len() {
            sum += get_shortest_distance(&expand.galaxies[i], &expand.galaxies[j]);
        }
    }
    sum
}

#[aoc(day11, part1)]
pub fn sum_shortest_lengths_2(input: &Cosmos) -> u64 {
    sum_shortest_lengths(input, 2)
}

#[aoc(day11, part2)]
pub fn sum_shortest_lengths_1million(input: &Cosmos) -> u64 {
    sum_shortest_lengths(input, 1000000)
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
    fn test_day11_expand() {
        let input = parse_input(DAY11_EXAMPLE);
        let expand = expand(&input, 2);
        assert_eq!(expand.size, (13, 12));
    }

    #[test]
    fn test_day11_example1() {
        let input = parse_input(DAY11_EXAMPLE);
        let expand = expand(&input, 2);
        assert_eq!(get_shortest_distance(&expand.galaxies[0], &expand.galaxies[6]), 15);
    }

    #[test]
    fn test_day11_example2() {
        let input = parse_input(DAY11_EXAMPLE);
        let expand = expand(&input, 2);
        assert_eq!(get_shortest_distance(&expand.galaxies[2], &expand.galaxies[5]), 17);
    }

    #[test]
    fn test_day11_example3() {
        let input = parse_input(DAY11_EXAMPLE);
        let expand = expand(&input, 2);
        assert_eq!(get_shortest_distance(&expand.galaxies[7], &expand.galaxies[8]), 5);
    }

    #[test]
    fn test_day11_part1() {
        let input = parse_input(DAY11_EXAMPLE);
        assert_eq!(sum_shortest_lengths_2(&input), 374);
    }

    #[test]
    fn test_day11_part2_x10() {
        let input = parse_input(DAY11_EXAMPLE);
        assert_eq!(sum_shortest_lengths(&input, 10), 1030);
    }

    #[test]
    fn test_day11_part2_x100() {
        let input = parse_input(DAY11_EXAMPLE);
        assert_eq!(sum_shortest_lengths(&input, 100), 8410);
    }
}
