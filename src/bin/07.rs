use itertools::Itertools;
use phf::phf_map;
use std::cmp::Ordering;
use std::fmt;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(Hand::parse)
            .sorted()
            .enumerate()
            .map(|(idx, hand)| hand.bid as u32 * (idx as u32 + 1))
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

/*
   7- Five of a kind, where all five cards have the same label: AAAAA
   6- Four of a kind, where four cards have the same label and one card has a different label: AA8AA
   5- Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
   4- Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
   3- Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
   2- One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
   1- High card, where all cards' labels are distinct: 23456
*/

const HIGH_CARD: u8 = 1;
const ONE_PAIR: u8 = 2;
const TWO_PAIR: u8 = 3;
const THREE_OF_A_KIND: u8 = 4;
const FULL_HOUSE: u8 = 5;
const FOUR_OF_A_KIND: u8 = 6;
const FIVE_OF_A_KIND: u8 = 7;

static CARDS_VALUE: phf::Map<u8, u8> = phf_map! {
    b'2' => 0,
    b'3' => 1,
    b'4' => 2,
    b'5' => 3,
    b'6' => 4,
    b'7' => 5,
    b'8' => 6,
    b'9' => 7,
    b'T' => 8,
    b'J' => 9,
    b'Q' => 10,
    b'K' => 11,
    b'A' => 12,
};

static CARDS_LABELS: phf::Map<u8, u8> = phf_map! {
    0u8 => b'2',
    1u8 => b'3',
    2u8 => b'4',
    3u8 => b'5',
    4u8 => b'6',
    5u8 => b'7',
    6u8 => b'8',
    7u8 => b'9',
    8u8 => b'T',
    9u8 => b'J',
    10u8 => b'Q',
    11u8 => b'K',
    12u8 => b'A',
};

#[derive(Copy, Clone)]
struct Hand {
    cards: [u8; 5],
    hand_type: u8,
    bid: u16,
}

impl Hand {
    fn parse(raw: &str) -> Self {
        let mut tokens = raw.split(' ');
        let cards = tokens
            .next()
            .unwrap()
            .as_bytes()
            .iter()
            .map(|c| CARDS_VALUE.get(c).unwrap())
            .copied()
            .collect::<Vec<u8>>()
            // fixme: is this necessary?
            .try_into()
            .unwrap();
        let bid = tokens.next().unwrap().parse().unwrap();

        Hand {
            cards,
            bid,
            hand_type: Hand::hand_type(cards),
        }
    }

    fn hand_type(cards: [u8; 5]) -> u8 {
        let mut counts = [0; 13];
        for card in cards.iter() {
            counts[*card as usize] += 1;
        }

        let pattern = counts
            .into_iter()
            .filter(|c| *c > 0)
            .sorted()
            .collect::<Vec<u16>>();

        match pattern.as_slice() {
            [1, 1, 1, 1, 1] => HIGH_CARD,
            [1, 1, 1, 2] => ONE_PAIR,
            [1, 2, 2] => TWO_PAIR,
            [1, 1, 3] => THREE_OF_A_KIND,
            [2, 3] => FULL_HOUSE,
            [1, 4] => FOUR_OF_A_KIND,
            [5] => FIVE_OF_A_KIND,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = self
            .cards
            .iter()
            .map(|c| CARDS_LABELS.get(c).unwrap())
            .join("");
        write!(f, "{} {}", repr, self.bid)
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_solution_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(256448566));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_full_house() {
        let hand = Hand::parse("QQAQA 483");
        assert_eq!(hand.hand_type, FULL_HOUSE);
        assert_eq!(hand.cards, [10, 10, 12, 10, 12]);
        assert_eq!(hand.bid, 483);
    }

    #[test]
    fn test_order_example_1() {
        assert_eq!(Hand::parse("QQAQA 483") > Hand::parse("23456 483"), true);
        assert_eq!(Hand::parse("33332 483") > Hand::parse("2AAAA 483"), true);
    }
}
