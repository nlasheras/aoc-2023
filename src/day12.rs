use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Row
{
    pub record : String,
    pub groups: Vec<usize>
}

impl Row 
{
    fn from(input: &str) -> Row {
        let mut parts = input.split(" ");
        let rs = parts.next().unwrap();
        let gs = parts.next().unwrap().split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        Row { record: rs.to_string(), groups: gs }
    }
}

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> Vec<Row>  {
    input.lines()
        .map(|s| Row::from(s))
        .collect()
}

#[allow(dead_code)]
fn get_groups(input: &str) -> Vec<usize> {
    input.split(".").map(|s| s.len()).filter(|n| *n > 0).collect()
}

#[allow(dead_code)]
fn find_arrangements_recursive(current: &str, remaining: &str, groups: &Vec<usize>) -> u64 {
    if let Some(idx) = remaining.find("?") {
        let mut new_c = current.to_string();
        if idx > 0 {
            new_c += &remaining[0..idx]
        } 
        let new_r = &remaining[idx+1..remaining.len()];
        return find_arrangements_recursive(&(new_c.clone() + "#"), new_r, groups) +
         find_arrangements_recursive(&(new_c.clone() + ".") , new_r, groups) 
    } else {
        let test = current.to_string() + remaining;
        let test_groups = get_groups(&test);
        if test_groups.eq(groups) {
            return 1;
        }
        0
    }
}

#[allow(dead_code)]
fn find_arrangements_part1(input: &Row) -> u64 { 
    find_arrangements_recursive(&"", &input.record, &input.groups)
}

use memoize::memoize;

#[memoize]
pub fn find_arrangements_dp(input: Row, i: usize, g: usize, r0: u64) -> u64 {
    /* In order to memoize the function, I do an function that brute forces the result using indexes on the already processed 
    part of the record (i) and the current group we are looking (j) */
    let record = &input.record;
    let groups = &input.groups;
    if i >= record.len() {
        // after we have the full string processed, it's a valid arrangement if we have consumed all the groups
        return if g == groups.len() { 1 } else { 0 }
    }

    let mut r = r0;
    let c = record.chars().nth(i).unwrap();
    if c != '#' {
        // we try recursively advancing the string (assume that char i is a .)
        r += find_arrangements_dp(input.clone(), i+1, g, 0);
    }
    
    if g < groups.len() {
        // assume that i is a # and try to match the current group g
        let j = i + groups[g];
        if j <= record.len() {
            let only_damaged = record[i..j].chars().all(|c| c == '#' || c == '?');
            let is_j_damaged = record.chars().nth(j) == Some('#');
            if only_damaged && !is_j_damaged {
                r += find_arrangements_dp(input.clone(), j+1, g+1, 0);
            }
        }
    }
    r
}

fn find_arrangements(input: &Row) -> u64 {
    find_arrangements_dp(input.clone(), 0, 0, 0)
}

fn find_arrangements_with_fold(input: &Row) -> u64 {
    let mut record = String::from("");
    let mut groups = Vec::new();
    for i in 0..5 {
        if i > 0 {
            record += "?";
        }
        record += &input.record;
        groups.append(&mut input.groups.clone());
    }
    let tmp = Row{ record: record.clone(), groups: groups.clone() };
    find_arrangements_dp(tmp, 0, 0, 0)
}

#[aoc(day12, part1)]
pub fn sum_arrangements(input: &Vec<Row>) -> u64 {
    input.iter().map(|r| find_arrangements(r)).sum() 
}

#[aoc(day12, part2)]
pub fn sum_arrangements_with_fold(input: &Vec<Row>) -> u64 {
    input.iter().map(|r| find_arrangements_with_fold(r)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY12_EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_day12_example1() {
        let row = Row::from("???.### 1,1,3");
        assert_eq!(find_arrangements(&row), 1);
        assert_eq!(find_arrangements_part1(&row), 1);
    }

    #[test]
    fn test_day12_example2() {
        let row = Row::from(".??..??...?##. 1,1,3");
        assert_eq!(find_arrangements(&row), 4);
        assert_eq!(find_arrangements_part1(&row), 4);
    }

    #[test]
    fn test_day12_example6() {
        let row = Row::from("?###???????? 3,2,1");
        assert_eq!(find_arrangements(&row), 10);
        assert_eq!(find_arrangements_part1(&row), 10);
    }

    #[test]
    fn test_day12_part1() {
        let input = parse_input(DAY12_EXAMPLE);
        assert_eq!(sum_arrangements(&input), 21);
    }

    #[test]
    fn test_day12_part2() {
        let input = parse_input(DAY12_EXAMPLE);
        assert_eq!(sum_arrangements_with_fold(&input), 525152);
    }


}
