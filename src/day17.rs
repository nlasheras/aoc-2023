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

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct PathPos {
    pub pos : (i32, i32),
    pub dir : (i32, i32),
    pub straight_steps: usize,
    pub heat_loss: u64
}

fn get_neighbors(grid: &Grid<usize>, pos: &PathPos, max_straight: usize, min_turn: usize) -> Vec<PathPos> {
    let mut ret = Vec::new();
    for dir in [pos.dir, (pos.dir.1, pos.dir.0), (-pos.dir.1, -pos.dir.0)] {
        if dir == pos.dir { // straight
            let mut cost = 0;
            let mut newpos = pos.pos;
            for i in 0..(max_straight-pos.straight_steps) {
                newpos = (newpos.0 + dir.0, newpos.1 + dir.1);
                if let Some(heat_loss) = grid.cell_at(newpos.0, newpos.1) {
                    cost += heat_loss;
                    ret.push(PathPos{ pos: newpos, dir: dir, straight_steps: pos.straight_steps + i + 1, heat_loss: cost as u64 })
                }
            }
        } else if pos.straight_steps >= min_turn {
            let pos = (pos.pos.0 + dir.0, pos.pos.1 + dir.1);
            if let Some(heat_loss) = grid.cell_at(pos.0, pos.1) {
                ret.push(PathPos{ pos: pos, dir: dir, straight_steps: 1, heat_loss: heat_loss as u64 })
            }
        }
    }
    ret
}

fn get_neighbors_part1(grid: &Grid<usize>, pos: &PathPos) -> Vec<PathPos> {
    get_neighbors(grid, pos, 3, 0)
}

fn get_neighbors_ultra(grid: &Grid<usize>, pos: &PathPos) -> Vec<PathPos> {
    get_neighbors(grid, pos, 10, 4)
}


fn manhattan_dist(pos: (i32, i32), goal: (i32, i32)) -> u64 {
    ((goal.0 - pos.0).unsigned_abs() + (goal.1 - pos.1).unsigned_abs()) as u64
}

fn get_dir_unit(pos: (i32, i32), prev: (i32, i32)) -> (i32, i32) {
    let dir = (pos.0 - prev.0, pos.1 - prev.1);
    let abs = dir.0.abs() + dir.1.abs();
    (dir.0 / abs, dir.1 / abs)
}

fn find_path(grid: &Grid<usize>, start: (i32, i32), end: (i32, i32), get_neighbors: fn(grid: &Grid<usize>, pos: &PathPos) -> Vec<PathPos>) -> Option<Vec<(i32, i32)>> {
    let mut open_set = DoublePriorityQueue::new();
    let start_pos = PathPos{ pos: start, dir: (1, 0), straight_steps: 1, heat_loss: 0 };
    open_set.push(start_pos.clone(), 0);

    let mut closed_set = BTreeSet::new();
    let mut came_from = BTreeMap::new();

    let inf = u64::MAX;
    let mut g_score = BTreeMap::new();
    g_score.insert(start_pos.clone(), 0u64);

    let mut f_score = BTreeMap::new();
    f_score.insert(start_pos.clone(), manhattan_dist(start, end));

    while !open_set.is_empty() {
        // get the element with smallest fScore
        let (current, _priority) = open_set.pop_min().unwrap();
        if closed_set.contains(&current) {
            continue;
        }

        if current.pos == end {
            let mut path = vec![current.pos];
            let mut path_node = current;
            while came_from.contains_key(&path_node) {
                path_node = *came_from.get(&path_node).unwrap();
                
                let mut prev_pos = path[ path.len() - 1];
                let dir = get_dir_unit(path_node.pos, prev_pos);
                while prev_pos != path_node.pos {
                    prev_pos = (prev_pos.0 + dir.0, prev_pos.1 + dir.1);
                    path.push(prev_pos);
                }
            }
            path.reverse();
            return Some(path);
        }

        closed_set.insert(current);

        for candidate in get_neighbors(grid, &current) {
            if closed_set.contains(&candidate) {
                continue;
            }

            let g_func = candidate.heat_loss;
            let tentative_g_score = g_score.entry(current).or_insert(inf).to_owned() + g_func;

            let neighbor_score = g_score.entry(candidate).or_insert(inf).to_owned();
            if tentative_g_score < neighbor_score {
                *came_from.entry(candidate).or_insert(current) = current;

                g_score
                    .entry(candidate)
                    .and_modify(|e| *e = tentative_g_score)
                    .or_insert(tentative_g_score);

                let score = tentative_g_score + manhattan_dist(candidate.pos, end) as u64;
                f_score
                    .entry(candidate)
                    .and_modify(|e| *e = score)
                    .or_insert(score);

                open_set.push(candidate, score);
            }
        }
    }
    None
}

fn heat_loss_path(input: &Grid<usize>, path: &Vec<(i32, i32)>) -> u64 {
    path.iter().skip(1).map(|(x,y)| input.cell_at(*x, *y).unwrap() as u64).sum::<u64>()
}

#[allow(dead_code)]
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
    if let Some(path) = find_path(input, (0, 0), (input.width() as i32 - 1, input.height() as i32 - 1), get_neighbors_part1) {
        //print_path(input, &path);
        return heat_loss_path(input, &path);
    }
    0
}

#[aoc(day17, part2)]
pub fn sum_heat_loss_ultra(input: &Grid<usize>) -> u64 {
    if let Some(path) = find_path(input, (0, 0), (input.width() as i32 - 1, input.height() as i32 - 1), get_neighbors_ultra) {
        //print_path(input, &path);
        return heat_loss_path(input, &path);
    }
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

    #[test]
    fn test_day17_part2() {
        let input = parse_input(DAY17_EXAMPLE);
        assert_eq!(sum_heat_loss_ultra(&input), 94);
    }

}