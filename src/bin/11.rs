use advent_of_code::util::graph;
use std::collections::HashSet;
advent_of_code::solution!(11);

pub type Galaxy = (usize, usize);

pub trait Distance {
    fn distance(self, other: Self) -> usize;
}

impl Distance for (usize, usize) {
    fn distance(self, other: Self) -> usize {
        let (x1, y1) = self;
        let (x2, y2) = other;
        ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as usize
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = graph::parse(input);

    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut expanded_rows: HashSet<usize> = HashSet::new();
    let mut expanded_cols: HashSet<usize> = HashSet::new();
    for i in 0..graph[0].len() {
        expanded_cols.insert(i);
    }

    for (j, row) in graph.iter().enumerate() {
        let mut has_galaxy = false;
        for (i, elem) in row.iter().enumerate() {
            if *elem == b'#' {
                has_galaxy = true;
                expanded_cols.remove(&i);
            }
        }
        if !has_galaxy {
            expanded_rows.insert(j);
        }
    }

    let mut expanded_rows_so_far = 0;
    for (j, row) in graph.iter().enumerate() {
        if expanded_rows.contains(&j) {
            expanded_rows_so_far += 1;
            continue;
        }
        let mut expanded_cols_so_far = 0;
        for (i, elem) in row.iter().enumerate() {
            if expanded_cols.contains(&i) {
                expanded_cols_so_far += 1;
                continue;
            }
            if *elem == b'#' {
                galaxies.push((i + expanded_cols_so_far, j + expanded_rows_so_far));
            }
        }
    }

    Some(
        PairIterator::new(galaxies)
            .map(|(g1, g2)| g1.distance(g2))
            .sum::<usize>() as u32,
    )
}

struct PairIterator {
    galaxies: Vec<Galaxy>,
    i: usize,
    j: usize,
}

impl PairIterator {
    fn new(galaxies: Vec<Galaxy>) -> Self {
        Self {
            galaxies,
            i: 0,
            j: 1,
        }
    }
}

impl Iterator for PairIterator {
    type Item = (Galaxy, Galaxy);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.galaxies.len() {
            return None;
        }
        if self.j >= self.galaxies.len() {
            self.i += 1;
            self.j = self.i + 1;
            return self.next();
        }
        let result = Some((self.galaxies[self.i], self.galaxies[self.j]));
        self.j += 1;
        result
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_solution_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(10885634));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_should_calculate_distance_example_1() {
        let g1: Galaxy = (4, 0);
        let g7: Galaxy = (9, 10);
        assert_eq!(g1.distance(g7), 15);
    }
}
