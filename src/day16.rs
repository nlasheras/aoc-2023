use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use crate::utils::Grid;
use crate::utils::Point;

#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> Grid<char>  {
    Grid::from(input, |c| c)
}

pub fn count_energized_from(input: &Grid<char>, start: Point, start_dir: Point) -> u64 {
    let mut energized: Grid<bool> = Grid::new(&vec![false; input.width()*input.height()], input.width());
    let mut rays = Vec::new();
    rays.push((start, start_dir));

    while rays.len() > 0 {
        let mut new_rays = Vec::new();
        for (pos, dir) in rays {
            let c = input.cell_at(pos.x, pos.y).unwrap();
            let was_energized = energized.cell_at(pos.x, pos.y).unwrap();
            energized.set_at(pos.x as usize, pos.y as usize, true);
            match c {
                '.' => {
                    let new_pos = pos + dir;
                    if input.cell_at(new_pos.x, new_pos.y).is_some() {
                        new_rays.push((new_pos, dir));
                    }
                },
                '\\' | '/' => {
                    let new_dir = if c == '\\' { Point::new(dir.y, dir.x) } else { Point::new(-dir.y, -dir.x) };
                    let new_pos = pos + new_dir;
                    if input.cell_at(new_pos.x, new_pos.y).is_some() {
                        new_rays.push((new_pos, new_dir));
                    }
                },
                '|' | '-' => {
                    let is_pointy_end = (c == '|' && dir.x != 0) || (c == '-' && dir.y != 0);
                    if is_pointy_end {
                        // only split rays first time the beam goes over the splitter
                        if !was_energized {
                            let d1 = Point::new(dir.y, dir.x);
                            let d2 = Point::new(-dir.y, -dir.x);
                            let p1 = pos + d1;
                            if input.cell_at(p1.x, p1.y).is_some() {
                                new_rays.push((p1, d1));
                            }
                            let p2 = pos + d2;
                            if input.cell_at(p2.x, p2.y).is_some() {
                                new_rays.push((p2, d2));
                            }
                        }
                    }
                    else {
                        let new_pos = pos + dir;
                        if input.cell_at(new_pos.x, new_pos.y).is_some() {
                            new_rays.push((new_pos, dir));
                        }
                    }
                }, 
                _ => panic!()
            }
        }
        rays = new_rays;
    }
    energized.cells.iter().filter(|b| **b).count() as u64
}

#[aoc(day16, part1)]
pub fn count_energized(input: &Grid<char>) -> u64 {
    count_energized_from(input, Point::new(0, 0), Point::new(1, 0))
}

#[aoc(day16, part2)]
pub fn find_max_energized(input: &Grid<char>) -> u64 {

    let edges : Vec<(Point, Point)> = vec![ 
        (0..input.height()).map(|y| (Point::new(0, y as i32), Point::new(1, 0))).collect::<Vec<(Point, Point)>>(),
        (0..input.height()).map(|y| (Point::new(input.width() as i32 - 1, y as i32), Point::new(-1, 0))).collect::<Vec<(Point, Point)>>(),
        (0..input.width()).map(|x| (Point::new(x as i32, 0), Point::new(0, 1))).collect::<Vec<(Point, Point)>>(),
        (0..input.width()).map(|x| (Point::new(x as i32, input.height() as i32 - 1), Point::new(0, -1))).collect::<Vec<(Point, Point)>>()
    ].concat();

    edges.iter().map(|(p, dir)| count_energized_from(input, *p, *dir)).max().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    const DAY16_EXAMPLE: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn test_day16_part1() {
        let input = parse_input(DAY16_EXAMPLE);
        assert_eq!(count_energized(&input), 46);
    }

    #[test]
    fn test_day16_part2() {
        let input = parse_input(DAY16_EXAMPLE);
        assert_eq!(find_max_energized(&input), 51);
    }

}