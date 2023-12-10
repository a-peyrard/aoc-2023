advent_of_code::solution!(10);
use std::collections::{HashSet, VecDeque};

pub fn part_one(input: &str) -> Option<u32> {
    let graph: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    let mut start = (0, 0);
    for (j, row) in graph.iter().enumerate() {
        for (i, elem) in row.iter().enumerate() {
            if *elem == b'S' {
                start = (i, j)
            }
        }
    }

    let mut queue: VecDeque<(usize, usize, u32, usize, usize)> = VecDeque::new();
    queue.push_back((start.0, start.1, 0, 0, 0));
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut result = 0;
    while !queue.is_empty() {
        let (x, y, distance, px, py) = queue.pop_front().unwrap();
        let current_char = graph[y][x];
        if seen.contains(&(x, y)) {
            result = distance;
            break;
        }

        match current_char {
            b'S' => {
                // north
                if y > 0
                    && (graph[y - 1][x] == b'|'
                        || graph[y - 1][x] == b'7'
                        || graph[y - 1][x] == b'F')
                {
                    queue.push_back((x, y - 1, distance + 1, x, y));
                }
                // east
                if x < graph.len() - 1
                    && (graph[y][x + 1] == b'-'
                        || graph[y][x + 1] == b'7'
                        || graph[y][x + 1] == b'J')
                {
                    queue.push_back((x + 1, y, distance + 1, x, y));
                }
                // south
                if y < graph[x].len() - 1
                    && (graph[y + 1][x] == b'|'
                        || graph[y + 1][x] == b'J'
                        || graph[y + 1][x] == b'L')
                {
                    queue.push_back((x, y + 1, distance + 1, x, y));
                }
                // west
                if x > 0
                    && (graph[y][x - 1] == b'-'
                        || graph[y][x - 1] == b'F'
                        || graph[y][x - 1] == b'L')
                {
                    queue.push_back((x - 1, y, distance + 1, x, y));
                }
            }
            b'|' => {
                if y > py {
                    queue.push_back((x, y + 1, distance + 1, x, y))
                } else {
                    queue.push_back((x, y - 1, distance + 1, x, y))
                }
            }
            b'-' => {
                if x > px {
                    queue.push_back((x + 1, y, distance + 1, x, y))
                } else {
                    queue.push_back((x - 1, y, distance + 1, x, y))
                }
            }
            b'L' => match px - x {
                0 => queue.push_back((x + 1, y, distance + 1, x, y)),
                1 => queue.push_back((x, y - 1, distance + 1, x, y)),
                _ => unreachable!(),
            },
            b'J' => match x - px {
                0 => queue.push_back((x - 1, y, distance + 1, x, y)),
                1 => queue.push_back((x, y - 1, distance + 1, x, y)),
                _ => unreachable!(),
            },
            b'7' => match x - px {
                0 => queue.push_back((x - 1, y, distance + 1, x, y)),
                1 => queue.push_back((x, y + 1, distance + 1, x, y)),
                _ => unreachable!(),
            },
            b'F' => match px - x {
                0 => queue.push_back((x + 1, y, distance + 1, x, y)),
                1 => queue.push_back((x, y + 1, distance + 1, x, y)),
                _ => unreachable!(),
            },
            _ => panic!("unexpected end of pipe :/ => {}", current_char),
        }

        seen.insert((x, y));
    }

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_example_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_solution_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(7063));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }
}
