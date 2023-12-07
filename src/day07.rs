use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use itertools::Itertools;

use std::cmp::Ordering;
use std::collections::BTreeMap;


#[derive(Clone, PartialEq, PartialOrd, Eq)]
pub struct Hand
{
    pub cards: Vec<char>
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandStrength
{
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

pub fn get_card_strength(c: &char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap()
    }
}

impl Hand {
    fn from(input: &str) -> Hand {
        Hand { cards: input.chars().collect() }
    }

    fn get_strength(&self) -> HandStrength {
        let mut map: BTreeMap<char, u32> = BTreeMap::new();
        for c in &self.cards {
            *map.entry(*c).or_insert(0) += 1;
        }
        let groups : Vec<(char, u32)> = map.iter().sorted_by(|a, b| 
            a.1.cmp(b.1).reverse()
        ).map(|p| (*p.0, *p.1)).collect();

        match groups[0].1 {
            5 => HandStrength::FiveOfAKind,
            4 => HandStrength::FourOfAKind,
            3 => {
                match groups[1].1 {
                    2 => HandStrength::FullHouse,
                    _ => HandStrength::ThreeOfAKind
                }
            } 
            2 => {
                match groups[1].1 {
                    2 => HandStrength::TwoPair,
                    _ => HandStrength::OnePair
                }
            }
            _ => HandStrength::HighCard
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let s1 = self.get_strength();
        let s2 = other.get_strength();
        if s1 == s2 {
            for i in 0..5 {
                let cs1 = get_card_strength(&self.cards[i]);
                let cs2 = get_card_strength(&other.cards[i]);
                if cs1 > cs2 {
                    return Ordering::Greater
                }
                else if cs1 < cs2 {
                    return Ordering::Less;
                }
            }
            panic!();
        }
        s1.cmp(&s2).reverse()
    }
}
#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<(Hand, u64)> {
    input.lines().map(|s| {
        let mut parts = s.split(" ");
        (Hand::from(parts.next().unwrap()), parts.next().unwrap().parse::<u64>().unwrap())
    }).collect()
}

#[aoc(day7, part1)]
pub fn count_winnings(input: &[(Hand, u64)]) -> u64 {
    input.iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .enumerate()
        .map(|p| (p.0 + 1) as u64 * p.1.1).sum()
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
        assert_eq!(input.get_strength(), HandStrength::OnePair);
    }   

    #[test]
    fn test_day7_hand2() {
        let input = Hand::from("T55J5");
        assert_eq!(input.get_strength(), HandStrength::ThreeOfAKind);
    }   

    #[test]
    fn test_day7_full_house() {
        let input = Hand::from("77788");
        assert_eq!(input.get_strength(), HandStrength::FullHouse);
    }   

    #[test]
    fn test_day7_two_pair() {
        let input = Hand::from("KTJJT");
        assert_eq!(input.get_strength(), HandStrength::TwoPair);
    }   

    #[test]
    fn test_day7_part1() {
        let input = parse_input(DAY07_EXAMPLE);
        assert_eq!(count_winnings(&input), 6440)
    }
}
