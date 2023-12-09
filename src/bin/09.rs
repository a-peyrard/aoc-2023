advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(Series::parse)
            .map(|s| s.next_value())
            .sum(),
    )
}

struct Series {
    content: Vec<i32>,
}

impl Series {
    fn parse(input: &str) -> Self {
        Self {
            content: input
                .split(' ')
                .map(|token| token.parse::<i32>().unwrap())
                .collect(),
        }
    }

    fn next_value(&self) -> i32 {
        let mut matrix: Vec<Vec<i32>> = Vec::new();
        matrix.push(self.content.clone());
        let mut all_same_diff = false;
        while !all_same_diff {
            let current = matrix.last().unwrap();
            let mut diffs = vec![0; current.len() - 1];
            let previous = current[1] - current[0];
            all_same_diff = true;
            for i in 1..current.len() {
                let diff = current[i] - current[i - 1];
                if diff != previous {
                    all_same_diff = false;
                }
                diffs[i - 1] = diff
            }
            matrix.push(diffs);
        }

        let mut increment = 0;
        for row in matrix.iter().skip(1).rev() {
            increment += row.last().unwrap();
        }

        self.content.last().unwrap() + increment
    }
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
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_next_value_should_work_for_example_1() {
        let series = Series::parse("0 3 6 9 12 15");
        assert_eq!(series.next_value(), 18);
    }

    #[test]
    fn test_next_value_should_work_for_example_2() {
        let series = Series::parse("1 3 6 10 15 21");
        assert_eq!(series.next_value(), 28);
    }

    #[test]
    fn test_next_value_should_work_for_example_3() {
        let series = Series::parse("10 13 16 21 30 45");
        assert_eq!(series.next_value(), 68);
    }

    #[test]
    fn test_solution_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1930746032));
    }
}
