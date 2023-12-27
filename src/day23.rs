use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::cmp;

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

pub fn find_longest_path_bfs(input: &Grid<char>, is_icy: bool) -> u64 {
    let (start, end) = find_start_end(input);
    let mut open_set = VecDeque::new();
    open_set.push_back(vec![start]);

    let mut closed_set = Vec::new();
    while !open_set.is_empty() {
        let path = open_set.pop_front().unwrap();
        let current = path[path.len() - 1];

        if current == end {
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

            let tc = if is_icy { c } else { '.' };
            match tc {
                '.' => new_path.push(pos),
                '>' => new_path.append(&mut vec![pos, (pos.0+1, pos.1)]),
                '<' => new_path.append(&mut vec![pos, (pos.0-1, pos.1)]),
                '^' => new_path.append(&mut vec![pos, (pos.0, pos.1-1)]),
                'v' => new_path.append(&mut vec![pos, (pos.0, pos.1+1)]),
                _ => panic!()
            }
            if is_icy && c != '.' {
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

#[aoc(day23, part1)]
pub fn find_longest_path_icy(input: &Grid<char>) -> u64 {
    find_longest_path_bfs(input, true)
}

fn build_graph(input: &Grid<char>) -> BTreeMap<(i32, i32), Vec<((i32, i32), usize)>> {
    let mut ret = BTreeMap::new();
    for y in 0..input.height() {
        for x in 0..input.width() {
            let c = input.cell_at(x as i32, y as i32).unwrap();
            if c == '#' {
                continue;
            }

            let neighbors = input.neighbors_at(x as i32, y as i32);
            if neighbors.iter().filter(|(c, _)| *c != '#').count() != 2 { // junction (start/end have 1 neighbor)
                let mut pending = VecDeque::new();
                let mut closed = BTreeSet::new();
                closed.insert((x, y));
                for (_, pos) in neighbors.iter().filter(|(c, _)| *c != '#') {
                    pending.push_back((pos.clone(), 1));
                    closed.insert(pos.clone());
                }
                while let Some((pos, dist)) = pending.pop_front() {
                    let pos_neighbors = input.neighbors_at(pos.0 as i32, pos.1 as i32);
                    if pos_neighbors.iter().filter(|(c, _)| *c != '#').count() != 2 {
                        ret.entry((x as i32, y as i32)).or_insert(Vec::new()).push(((pos.0 as i32, pos.1 as i32), dist))
                    } else {
                        for (_, npos) in pos_neighbors.iter().filter(|(c, _)| *c != '#') {
                            if closed.contains(&npos) {
                                continue;
                            }
                            pending.push_back((npos.clone(), dist+1));
                            closed.insert(npos.clone());
                        }
                    }
                }
            }
        }
    }
    ret
}

#[aoc(day23, part2)]
pub fn find_longest_path_part2(input: &Grid<char>) -> u64 {
    let graph = build_graph(input);

    let (start, end) = find_start_end(input);
    let mut open_set = VecDeque::new();
    open_set.push_back(vec![start]);

    let mut closed_set = Vec::new();
    while !open_set.is_empty() {
        let path = open_set.pop_front().unwrap();
        let current = path[path.len() - 1];

        if current == end {
            closed_set.push(path);
            continue;
        }

        let neighbors = graph.get(&current).unwrap();
        for (npos, _) in neighbors {
            if path.contains(npos) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(npos.clone());
            open_set.push_back(new_path);
        }
    }

    if !closed_set.is_empty() {
        return closed_set.iter().fold(0, |acc, v| {
            let mut sum = 0;
            let mut current = v[0];
            for pos in v.iter().skip(1) {
                let dists = &graph[&current];
                for (npos, d) in dists {
                    if npos == pos {
                        sum += d;
                        break;
                    }
                }
                current = pos.clone();
            }
            cmp::max(acc, sum) 
        }) as u64
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
        assert_eq!(find_longest_path_icy(&input), 94);
    }

    #[test]
    fn test_day23_longest_path_no_ice() {
        let input = parse_input(DAY23_EXAMPLE);
        assert_eq!(find_longest_path_part2(&input), 154);
    }
}