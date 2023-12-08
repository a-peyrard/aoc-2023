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
        let mut winning_possibilities = 0;
        let pivot = self.time / 2;
        for n in pivot..self.time + 1 {
            if Race::distance_at(self.time, n) > self.distance {
                winning_possibilities += 1;
            }
        }

        for n in (0..pivot).rev() {
            if Race::distance_at(self.time, n) > self.distance {
                winning_possibilities += 1;
            }
        }

        winning_possibilities
    }

    fn distance_at(time: u64, press: u64) -> u64 {
        (time - press) * press
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
    fn test_should_compute_distance_at_examples() {
        // Don't hold the button at all (that is, hold it for 0 milliseconds) at the start of the race. The boat won't move; it will have traveled 0 millimeters by the end of the race.
        // Hold the button for 1 millisecond at the start of the race. Then, the boat will travel at a speed of 1 millimeter per millisecond for 6 milliseconds, reaching a total distance traveled of 6 millimeters.
        // Hold the button for 2 milliseconds, giving the boat a speed of 2 millimeters per millisecond. It will then get 5 milliseconds to move, reaching a total distance of 10 millimeters.
        // Hold the button for 3 milliseconds. After its remaining 4 milliseconds of travel time, the boat will have gone 12 millimeters.
        // Hold the button for 4 milliseconds. After its remaining 3 milliseconds of travel time, the boat will have gone 12 millimeters.
        // Hold the button for 5 milliseconds, causing the boat to travel a total of 10 millimeters.
        // Hold the button for 6 milliseconds, causing the boat to travel a total of 6 millimeters.
        // Hold the button for 7 milliseconds. That's the entire duration of the race. You never let go of the button. The boat can't move until you let go of the button. Please make sure you let go of the button so the boat gets to move. 0 millimeters.
        assert_eq!(Race::distance_at(7, 0), 0);
        assert_eq!(Race::distance_at(7, 1), 6);
        assert_eq!(Race::distance_at(7, 2), 10);
        assert_eq!(Race::distance_at(7, 3), 12);
        assert_eq!(Race::distance_at(7, 4), 12);
        assert_eq!(Race::distance_at(7, 5), 10);
        assert_eq!(Race::distance_at(7, 6), 6);
        assert_eq!(Race::distance_at(7, 7), 0);
    }
}
