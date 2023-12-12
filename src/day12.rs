use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

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

fn get_groups(input: &str) -> Vec<usize> {
    input.split(".").map(|s| s.len()).filter(|n| *n > 0).collect()
}

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

fn find_arrangements(input: &Row) -> u64 {
    find_arrangements_recursive("", &input.record, &input.groups)
}

#[aoc(day12, part1)]
pub fn sum_arrangements(input: &Vec<Row>) -> u64 {
    input.iter().map(|r| find_arrangements(r)).sum() 
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
    }

    #[test]
    fn test_day12_example2() {
        let row = Row::from(".??..??...?##. 1,1,3");
        assert_eq!(find_arrangements(&row), 4);
    }

    #[test]
    fn test_day12_example6() {
        let row = Row::from("?###???????? 3,2,1");
        assert_eq!(find_arrangements(&row), 10);
    }

    #[test]
    fn test_day12_part1() {
        let input = parse_input(DAY12_EXAMPLE);
        assert_eq!(sum_arrangements(&input), 21);
    }


}
