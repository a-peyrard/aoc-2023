use std::collections::HashSet;
use std::io::BufRead;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        Grid::new(input.as_bytes().lines().flatten().collect::<Vec<_>>())
            .parts()
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        Grid::new(input.as_bytes().lines().flatten().collect::<Vec<_>>())
            .gear_ratios()
            .sum::<u32>(),
    )
}

struct Grid {
    width: usize,
    height: usize,
    elems: Vec<String>,
}

struct GridPartIterator<'a> {
    grid: &'a Grid,
    rows: Vec<&'a [u8]>,
    x: usize,
    y: usize,
}

impl<'a> Iterator for GridPartIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        let mut is_part = false;
        for j in self.y..self.grid.height {
            for i in self.x..self.grid.width {
                if self.rows[j][i].is_ascii_digit() {
                    if buf.is_empty() && !is_part && self.check_for_symbol(i, j, Direction::East) {
                        is_part = true;
                    }
                    buf.push(char::from(self.rows[j][i]));
                    if !is_part && self.check_for_symbol(i, j, Direction::Center) {
                        is_part = true;
                    }
                } else if !buf.is_empty() {
                    if !is_part && self.check_for_symbol(i, j, Direction::Center) {
                        is_part = true;
                    }
                    if is_part {
                        self.x = i;
                        self.y = j;
                        return Some(buf.parse().unwrap());
                    } else {
                        buf.clear();
                    }
                }
            }
            self.x = 0;
            if is_part {
                self.y = j + 1;
                return Some(buf.parse().unwrap());
            } else {
                buf.clear();
            }
        }

        None
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Center,
}

impl GridPartIterator<'_> {
    fn check_for_symbol(&self, mut x: usize, y: usize, direction: Direction) -> bool {
        let mut res = false;
        match direction {
            Direction::East => {
                if x == 0 {
                    return false;
                }
                x -= 1;
            }
            Direction::West => {
                if x >= (self.grid.width - 1) {
                    return false;
                }
                x += 1;
            }
            _ => {}
        }
        for j in (y as i32 - 1).max(0) as usize..(y + 2).min(self.grid.height) {
            if self.rows[j][x].is_ascii_digit() || self.rows[j][x] == b'.' {
                continue;
            } else {
                res = true;
                break;
            }
        }

        res
    }
}

struct GridGearRatioIterator<'a> {
    grid: &'a Grid,
    rows: Vec<&'a [u8]>,
    x: usize,
    y: usize,
}

impl<'a> Iterator for GridGearRatioIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut numbers: Vec<u32> = Vec::new();
        let mut tested_directions = HashSet::new();
        for j in self.y..self.grid.height {
            for i in self.x..self.grid.width {
                if self.rows[j][i] == b'*' {
                    for directions in [
                        (Direction::North, Direction::NorthWest, Direction::NorthEast),
                        (Direction::East, Direction::NorthEast, Direction::SouthEast),
                        (Direction::South, Direction::SouthEast, Direction::SouthWest),
                        (Direction::West, Direction::SouthWest, Direction::NorthWest),
                    ] {
                        self.find_in_main_direction(
                            i,
                            j,
                            &mut numbers,
                            &mut tested_directions,
                            directions,
                        );
                        if numbers.len() == 2 {
                            self.x = i + 1;
                            self.y = j;
                            return Some(numbers[0] * numbers[1]);
                        }
                    }
                    for sub in [
                        Direction::NorthEast,
                        Direction::SouthEast,
                        Direction::SouthWest,
                        Direction::NorthWest,
                    ] {
                        self.find_in_sub_direction(i, j, &mut numbers, sub, &mut tested_directions);
                        if numbers.len() == 2 {
                            self.x = i + 1;
                            self.y = j;
                            return Some(numbers[0] * numbers[1]);
                        }
                    }
                }
                numbers.clear();
                tested_directions.clear();
            }
            self.x = 0;
        }

        None
    }
}

impl GridGearRatioIterator<'_> {
    fn find_number(&self, (x, y): (usize, usize)) -> Option<u32> {
        let mut buf = String::new();
        if self.rows[y][x].is_ascii_digit() {
            buf.push(char::from(self.rows[y][x]));
            let mut i = x;
            while i >= 1 && self.rows[y][i - 1].is_ascii_digit() {
                i -= 1;
                buf.insert(0, char::from(self.rows[y][i]));
            }
            i = x;
            while i < self.grid.width - 1 && self.rows[y][i + 1].is_ascii_digit() {
                i += 1;
                buf.push(char::from(self.rows[y][i]));
            }

            return Some(buf.parse().unwrap());
        }

        None
    }

    fn find_in_main_direction(
        &self,
        x: usize,
        y: usize,
        numbers: &mut Vec<u32>,
        tested_directions: &mut HashSet<Direction>,
        (main, sub1, sub2): (Direction, Direction, Direction),
    ) {
        let main_val = self
            .grid
            .get_coords(main, x, y)
            .map(|c| self.find_number(c));
        if let Some(num) = main_val.flatten() {
            numbers.push(num);
            tested_directions.insert(sub1);
            tested_directions.insert(sub2);
        }
    }

    fn find_in_sub_direction(
        &self,
        x: usize,
        y: usize,
        numbers: &mut Vec<u32>,
        sub: Direction,
        tested_directions: &mut HashSet<Direction>,
    ) {
        if !tested_directions.contains(&sub) {
            let sub_val = self.grid.get_coords(sub, x, y).map(|c| self.find_number(c));
            if let Some(num) = sub_val.flatten() {
                numbers.push(num);
                tested_directions.insert(sub);
            }
        }
    }
}

impl Grid {
    fn new(raw: Vec<String>) -> Self {
        Self {
            width: raw[0].len(),
            height: raw.len(),
            elems: raw,
        }
    }

    fn parts(&self) -> GridPartIterator {
        GridPartIterator {
            rows: self.elems.iter().map(|s| s.as_bytes()).collect(),
            grid: self,
            x: 0,
            y: 0,
        }
    }

    fn gear_ratios(&self) -> GridGearRatioIterator {
        GridGearRatioIterator {
            rows: self.elems.iter().map(|s| s.as_bytes()).collect(),
            grid: self,
            x: 0,
            y: 0,
        }
    }

    fn get_coords(&self, direction: Direction, x: usize, y: usize) -> Option<(usize, usize)> {
        match direction {
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Direction::NorthEast => {
                if y == 0 || x >= self.width - 1 {
                    None
                } else {
                    Some((x + 1, y - 1))
                }
            }
            Direction::East => {
                if x >= self.width - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Direction::SouthEast => {
                if y >= self.height - 1 || x >= self.width - 1 {
                    None
                } else {
                    Some((x + 1, y + 1))
                }
            }
            Direction::South => {
                if y >= self.height - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Direction::SouthWest => {
                if y >= self.height - 1 || x == 0 {
                    None
                } else {
                    Some((x - 1, y + 1))
                }
            }
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Direction::NorthWest => {
                if y == 0 || x == 0 {
                    None
                } else {
                    Some((x - 1, y - 1))
                }
            }
            Direction::Center => Some((x, y)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_solution_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(556367));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }

    #[test]
    fn test_solution_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(89471771));
    }
}
