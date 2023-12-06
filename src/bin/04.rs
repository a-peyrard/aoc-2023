use std::collections::HashSet;
use std::io::BufRead;
advent_of_code::solution!(4);

struct Card {
    #[allow(dead_code)]
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn new(raw: String) -> Self {
        let mut initial = raw.split(':');
        let id: u32 = initial.next().unwrap()[4..].trim().parse().unwrap();
        let mut raw_numbers = initial.next().unwrap().trim().split('|');
        let winning_numbers = raw_numbers
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse().unwrap())
            .collect::<HashSet<u32>>();
        let numbers = raw_numbers
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<u32>>();

        Self {
            id,
            winning_numbers,
            numbers,
        }
    }
    fn score(self) -> u32 {
        let matches = self
            .numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count();

        match matches {
            0 => 0,
            _ => 1 << (matches - 1),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .as_bytes()
            .lines()
            .flatten()
            .map(Card::new)
            .map(|c| c.score())
            .sum::<u32>(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
