use std::io::BufRead;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        Grid::new(input.as_bytes().lines().flatten().collect::<Vec<_>>())
            .values()
            .sum::<u32>(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
                    if buf.is_empty() && !is_part && self.check_for_symbol(i, j, Direction::Left) {
                        is_part = true;
                    }
                    buf.push(char::from(self.rows[j][i]));
                    if !is_part && self.check_for_symbol(i, j, Direction::None) {
                        is_part = true;
                    }
                } else if !buf.is_empty() {
                    if !is_part && self.check_for_symbol(i, j, Direction::None) {
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

enum Direction {
    Left,
    None,
    #[allow(dead_code)]
    Right,
}

impl GridPartIterator<'_> {
    fn check_for_symbol(&self, mut x: usize, y: usize, direction: Direction) -> bool {
        let mut res = false;
        match direction {
            Direction::Left => {
                if x == 0 {
                    return false;
                }
                x -= 1;
            }
            Direction::Right => {
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

impl Grid {
    fn new(raw: Vec<String>) -> Self {
        Self {
            width: raw[0].len(),
            height: raw.len(),
            elems: raw,
        }
    }

    fn values(&self) -> GridPartIterator {
        GridPartIterator {
            rows: self.elems.iter().map(|s| s.as_bytes()).collect(),
            grid: self,
            x: 0,
            y: 0,
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
    fn test_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(556367));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
