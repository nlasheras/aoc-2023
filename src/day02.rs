use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use regex::Regex;

#[derive(Clone)]
pub struct CubeSet {
    pub r : u32,
    pub g : u32,
    pub b : u32
}


#[derive(Clone)]
pub struct CubeGame {
    pub id : u32,
    pub sets : Vec<CubeSet>
}

impl CubeGame {
    fn parse_id(input: &str) -> u32 {
        let re = Regex::new(r"Game (\d+)").unwrap();
        re.captures(input)
            .unwrap()
            .iter()
            .nth(1)
            .unwrap().unwrap()
            .as_str().parse::<u32>()
            .unwrap()
    }
    
    fn parse_sets(input: &str) -> Vec<CubeSet> {
        input.split("; ")
            .map(|s| CubeSet::from(s))
            .collect()
    }

    fn from(input: &str) -> CubeGame {
        let mut parts = input.split(": ");
        let id = Self::parse_id(parts.nth(0).unwrap());
        let sets = Self::parse_sets(parts.nth(0).unwrap());
        CubeGame { id: id, sets: sets }
    }

    pub fn is_possible(&self) -> bool {
        self.sets.iter().all(|s| s.r <= 12 && s.g <= 13 && s.b <= 14)
    }
}

impl CubeSet {
    fn from(input: &str) -> CubeSet {
        let mut ret = CubeSet { r: 0, g: 0, b: 0 };
        input.split(", ").into_iter()
        .for_each(|s| {
            let parts : Vec<&str> = s.split(" ").collect();
            let color = *parts.iter().nth(1).unwrap();
            let amount = parts.iter().nth(0).unwrap().parse::<u32>().unwrap();
            match color
            {
                "red" => ret.r += amount,
                "green" => ret.g += amount,
                "blue" => ret.b += amount,
                _ => panic!()
            }
        });
        ret
    }
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<CubeGame> {
    let ret: Vec<CubeGame> = input
        .lines()
        .map(|x| CubeGame::from(x))
        .collect();
    ret
}


#[aoc(day2, part1)]
pub fn solve_part1(entries: &[CubeGame]) -> u64 {
    entries.iter().fold(0, |sum, g| {
        if g.is_possible() {
            return sum + g.id as u64
        }
        sum
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1_game1() {
        let input = CubeGame::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(input.is_possible(), true);
    }

    #[test]
    fn test_day2_part1_game3() {
        let input = CubeGame::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        assert_eq!(input.is_possible(), false);
    }

}
