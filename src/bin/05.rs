use std::io::BufRead;

use advent_of_code::util::interval::Interval;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, mappers) = extract_seeds_and_mappers(input);

    let intervals = seeds
        .iter()
        .map(|&seed| Interval::new(seed, seed))
        .collect::<Vec<Interval>>();
    part_generic(intervals.into_iter(), mappers)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, mappers) = extract_seeds_and_mappers(input);

    let intervals = seeds.chunks_exact(2).map(|values| {
        let (seed, length) = (values[0], values[1]);
        Interval::new(seed, seed + length)
    });

    part_generic(intervals, mappers)
}

fn part_generic(intervals: impl Iterator<Item = Interval>, mappers: Vec<Mapper>) -> Option<u64> {
    // didn't manage to find a good way to iterate over the mappers and build the X flat map dynamically
    // thankfully there are only 7 mappers and this is a fixed size
    intervals
        .flat_map(|interval| mappers[0].map(interval))
        .flat_map(|interval| mappers[1].map(interval))
        .flat_map(|interval| mappers[2].map(interval))
        .flat_map(|interval| mappers[3].map(interval))
        .flat_map(|interval| mappers[4].map(interval))
        .flat_map(|interval| mappers[5].map(interval))
        .flat_map(|interval| mappers[6].map(interval))
        .map(|interval| interval.min)
        .min()
}

fn extract_seeds_and_mappers(input: &str) -> (Vec<u64>, Vec<Mapper>) {
    let mut lines = input.as_bytes().lines().flatten();

    // parse seeds
    let seeds = lines
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
    (seeds, mappers)
}

#[derive(Copy, Clone)]
struct Mapping {
    source: Interval,
    destination: u64,
}

impl Mapping {
    fn calculate_destination(self, interval: Interval) -> Interval {
        Interval::new(
            interval.min - self.source.min + self.destination,
            interval.max - self.source.min + self.destination,
        )
    }
}

struct Mapper {
    #[allow(dead_code)]
    name: String,
    mappings: Vec<Mapping>,
}

impl Mapper {
    fn new(name: String, raw_mappings: Vec<String>) -> Self {
        let mappings = raw_mappings
            .iter()
            .map(|s| {
                let v = s
                    .split(' ')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                Mapping {
                    source: Interval::new(v[1], v[1] + v[2]),
                    destination: v[0],
                }
            })
            .collect::<Vec<Mapping>>();

        Self { name, mappings }
    }

    /*
       we need to split the interval each time we encounter a mapper

       we have the initial [a, b]
       for each interval, we look at our interval, and the source interval
        we do intersection, if we have an intersection, then we do complement between our interval and the matched source interval
        we keep the complement (might be a list of intervals) and we map the intersected interval using the destination.


       we need to know what part of interval is mapped, and what part is not, the non mapped part is a list of intervals
       then we do the same thing for the next mapper, we will get al list of mapped intervals and a list of non mapped intervals
       ...

       Then we just look at the minimum value for all intervals
    */
    fn map(&self, input: Interval) -> Vec<Interval> {
        let mut res = Vec::new();
        let mut ranges_used = Vec::new();
        for mapping in &self.mappings {
            if let Some(intersection) = input.intersect(mapping.source) {
                ranges_used.push(intersection);
                res.push(mapping.calculate_destination(intersection));
            }
        }
        let ranges_not_used = input.complement_for_subs(ranges_used);
        for range_not_used in ranges_not_used {
            res.push(range_not_used);
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
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_solution_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(79004094));
    }
}
