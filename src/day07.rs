use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use itertools::Itertools;

use std::cmp::Ordering;
use std::collections::BTreeMap;


#[derive(Clone)]
pub struct Hand
{
    pub cards: Vec<char>
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType
{
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

pub fn get_card_strength(c: &char, use_jokers: bool) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => if use_jokers { 1 } else { 11 },
        'T' => 10,
        _ => c.to_digit(10).unwrap()
    }
}

impl Hand {
    fn from(input: &str) -> Hand {
        Hand { cards: input.chars().collect() }
    }

    fn get_type(&self, use_jokers: bool) -> HandType {
        let mut map: BTreeMap<char, u32> = BTreeMap::new();
        let mut jokers = 0;
        for c in &self.cards {
            if use_jokers && *c == 'J' {
                jokers += 1;
            }
            else {
                *map.entry(*c).or_insert(0) += 1;
            }
        }
        let groups : Vec<(char, u32)> = map.iter().sorted_by(|a, b| 
            a.1.cmp(b.1).reverse()
        ).map(|p| (*p.0, *p.1)).collect();

        let bigger_group = if groups.len() > 0 { groups[0].1 }  else { 0 };
        match bigger_group + jokers {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                match groups[1].1 {
                    2 => HandType::FullHouse,
                    _ => HandType::ThreeOfAKind
                }
            } 
            2 => {
                match groups[1].1 {
                    2 => HandType::TwoPair,
                    _ => HandType::OnePair
                }
            }
            _ => HandType::HighCard
        }
    }

    fn cmp(&self, other: &Self, use_jokers: bool) -> Ordering {
        let t1 = self.get_type(use_jokers);
        let t2 = other.get_type(use_jokers);
        if t1 == t2 {
            for i in 0..5 {
                let s1 = get_card_strength(&self.cards[i], use_jokers);
                let s2 = get_card_strength(&other.cards[i], use_jokers);
                if s1 > s2 {
                    return Ordering::Greater
                }
                else if s1 < s2 {
                    return Ordering::Less;
                }
            }
            panic!();
        }
        t1.cmp(&t2).reverse()
    }
}


#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<(Hand, u64)> {
    input.lines().map(|s| {
        let mut parts = s.split(" ");
        (Hand::from(parts.next().unwrap()), parts.next().unwrap().parse::<u64>().unwrap())
    }).collect()
}


pub fn count_winnings(input: &[(Hand, u64)], rank: fn(&Hand, &Hand) -> Ordering) -> u64 {
    input.iter()
        .sorted_by(|a, b| rank(&a.0, &b.0))
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u64 * bid).sum()
}

#[aoc(day7, part1)]
pub fn count_winnings_part1(input: &[(Hand, u64)]) -> u64 {
    count_winnings(input, |a,b| a.cmp(&b, false))
}

#[aoc(day7, part2)]
pub fn count_winnings_part2(input: &[(Hand, u64)]) -> u64 {
    count_winnings(input, |a,b| a.cmp(&b, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY07_EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_day7_hand1() {
        let input = Hand::from("32T3K");
        assert_eq!(input.get_type(false), HandType::OnePair);
    }   

    #[test]
    fn test_day7_hand2() {
        let input = Hand::from("T55J5");
        assert_eq!(input.get_type(false), HandType::ThreeOfAKind);
    }   

    #[test]
    fn test_day7_full_house() {
        let input = Hand::from("77788");
        assert_eq!(input.get_type(false), HandType::FullHouse);
    }   

    #[test]
    fn test_day7_two_pair() {
        let input = Hand::from("KTJJT");
        assert_eq!(input.get_type(false), HandType::TwoPair);
    }   

    #[test]
    fn test_day7_part1() {
        let input = parse_input(DAY07_EXAMPLE);
        assert_eq!(count_winnings_part1(&input), 6440)
    }

    #[test]
    fn test_day7_hand2_part2() {
        let input = Hand::from("T55J5");
        assert_eq!(input.get_type(true), HandType::FourOfAKind);
    }   

    #[test]
    fn test_day7_two_pair_part2() {
        let input = Hand::from("KTJJT");
        assert_eq!(input.get_type(true), HandType::FourOfAKind);
    }   

    #[test]
    fn test_day7_part2() {
        let input = parse_input(DAY07_EXAMPLE);
        assert_eq!(count_winnings_part2(&input), 5905)
    }
}
