use itertools::Itertools;
use std::io::BufRead;
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.as_bytes().lines().flatten();

    // parse seeds
    let mut seeds = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    lines.next(); // skip empty line

    // parse mappers
    let mut mappers = Vec::new();
    let mut name = lines.next().unwrap();
    let mut raw_mappings = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            mappers.push(Mapper::new(name.to_string(), raw_mappings));
            raw_mappings = Vec::new();
            name = lines.next().unwrap();
        } else {
            raw_mappings.push(line.to_string());
        }
    }
    mappers.push(Mapper::new(name.to_string(), raw_mappings));

    // map the seeds
    for seed in &mut seeds {
        for mapper in mappers.iter() {
            *seed = mapper.map(*seed);
        }
    }

    seeds.iter().min().copied()
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

struct Mapper {
    #[allow(dead_code)]
    name: String,
    mappings: Vec<(u64, u64, u64)>,
}

impl Mapper {
    fn new(name: String, raw_mappings: Vec<String>) -> Self {
        let mappings = raw_mappings
            .iter()
            .filter_map(|s| {
                s.split(' ')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_tuple::<(u64, u64, u64)>()
            })
            .collect::<Vec<(u64, u64, u64)>>();

        Self { name, mappings }
    }

    fn map(&self, input: u64) -> u64 {
        let mut res = input;
        for (destination, source, length) in &self.mappings {
            if input >= *source && input < *source + *length {
                res = input - *source + *destination;
                break;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_solution_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(525792406));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
