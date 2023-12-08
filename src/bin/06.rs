use itertools::Itertools;
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        Race::parse(input)
            .into_iter()
            .map(|r| r.winning_possibilities())
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(Race::parse_unique(input).winning_possibilities())
}

#[derive(Copy, Clone)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn parse(input: &str) -> Vec<Self> {
        let mut lines = input.lines();
        let times = Race::parse_line(lines.next().unwrap());
        let distances = Race::parse_line(lines.next().unwrap());

        let mut result = Vec::new();
        for (idx, time) in times.into_iter().enumerate() {
            result.push(Self {
                time,
                distance: distances[idx],
            })
        }

        result
    }

    fn parse_unique(input: &str) -> Self {
        let mut lines = input.lines();
        let time = Race::parse_line_unique(lines.next().unwrap());
        let distance = Race::parse_line_unique(lines.next().unwrap());

        Self { time, distance }
    }

    fn parse_line(line: &str) -> Vec<u64> {
        line.split_whitespace()
            .skip(1)
            .map(|w| w.parse().unwrap())
            .collect::<Vec<u64>>()
    }

    fn parse_line_unique(line: &str) -> u64 {
        line.split_whitespace().skip(1).join("").parse().unwrap()
    }

    fn winning_possibilities(&self) -> u64 {
        let t = self.time as f64;
        let d = self.distance as f64;

        let low: f64 = (t - ((t * t) - 4f64 * d).sqrt()) / 2f64;
        let high: f64 = (t + ((t * t) - 4f64 * d).sqrt()) / 2f64;

        let mut low_int = low.ceil() as i64;
        if low.fract() == 0f64 {
            low_int += 1;
        }
        let mut high_int = high.floor() as i64;
        if high.fract() == 0f64 {
            high_int -= 1;
        }

        (high_int - low_int + 1) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_solution_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1108800));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }

    #[test]
    fn test_solution_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(36919753));
    }

    #[test]
    fn test_winning_possibilities_1() {
        assert_eq!(
            Race {
                time: 7,
                distance: 9
            }
            .winning_possibilities(),
            4
        );
    }

    #[test]
    fn test_winning_possibilities_2() {
        assert_eq!(
            Race {
                time: 15,
                distance: 40
            }
            .winning_possibilities(),
            8
        );
    }
}
