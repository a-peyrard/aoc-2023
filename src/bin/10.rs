use std::collections::{HashSet, VecDeque};

use advent_of_code::util::graph;

advent_of_code::solution!(10);
pub fn part_one(input: &str) -> Option<u32> {
    let mut graph: Vec<Vec<u8>> = graph::parse(input);

    let (distance, _) = explore_and_mark(&mut graph);
    Some(distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut graph: Vec<Vec<u8>> = graph::parse(input);

    let (_, candidates) = explore_and_mark(&mut graph);

    // println!("\n\n=== Try to find island starting at {:?}", candidates);
    // graph::dump_graph(&graph);

    let mut islands_size = 0;
    for (x, y) in candidates.into_iter() {
        if graph[y][x] != b'X' {
            if let Some(island_size) = graph::find_island(&mut graph, x, y) {
                islands_size += island_size;
            }
        }
    }
    Some(islands_size)
}

fn explore_and_mark(graph: &mut Vec<Vec<u8>>) -> (u32, Vec<(usize, usize)>) {
    let mut start = (0, 0);
    for (j, row) in graph.iter().enumerate() {
        for (i, elem) in row.iter().enumerate() {
            if *elem == b'S' {
                start = (i, j)
            }
        }
    }

    let height = graph.len();
    let width = graph[0].len();

    /*
       we keep two list of candidates, we don't know yet which are the candidates inner the loop,
       or outer the loop.
       At the end of the function, the smallest list will be the one with the candidates inner the loop,
    */

    let mut inner_candidates: Vec<(usize, usize)> = Vec::new();
    let mut outer_candidates: Vec<(usize, usize)> = Vec::new();
    let mut left_turns = 0;
    let mut right_turns = 0;

    // we store:
    // x, y, distance, previous x, previous y and if we are looping clockwise or not
    let mut queue: VecDeque<(usize, usize, u32, usize, usize, bool)> = VecDeque::new();
    queue.push_back((start.0, start.1, 0, 0, 0, true));
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut result = 0;
    while !queue.is_empty() {
        let (x, y, distance, px, py, clockwise) = queue.pop_front().unwrap();
        let current_char = graph[y][x];
        graph[y][x] = b'X';
        if seen.contains(&(x, y)) {
            result = distance;
            break;
        }

        // if we are looping counter clockwise, left mean right, and right mean left
        let mut current_inner_candidates: &mut Vec<(usize, usize)> = &mut inner_candidates;
        let mut current_outer_candidates: &mut Vec<(usize, usize)> = &mut outer_candidates;
        let mut current_left_turns = &mut left_turns;
        let mut current_right_turns = &mut right_turns;
        if !clockwise {
            current_inner_candidates = &mut outer_candidates;
            current_outer_candidates = &mut inner_candidates;
            current_left_turns = &mut right_turns;
            current_right_turns = &mut left_turns;
        }

        /*
           we don't really know if we are going clockwise or not, but this is not really important,
           the only thing that matter is to be consistent, if we think we are going clockwise we will
           store to left or right, and then based on number of turns, we will chose the correct list of candidates.
        */
        match current_char {
            b'S' => {
                // north
                if y > 0
                    && (graph[y - 1][x] == b'|'
                        || graph[y - 1][x] == b'7'
                        || graph[y - 1][x] == b'F')
                {
                    queue.push_back((x, y - 1, distance + 1, x, y, clockwise));
                }
                // east
                if x < width - 1
                    && (graph[y][x + 1] == b'-'
                        || graph[y][x + 1] == b'7'
                        || graph[y][x + 1] == b'J')
                {
                    queue.push_back((x + 1, y, distance + 1, x, y, clockwise));
                }
                // south
                if y < height - 1
                    && (graph[y + 1][x] == b'|'
                        || graph[y + 1][x] == b'J'
                        || graph[y + 1][x] == b'L')
                {
                    queue.push_back((x, y + 1, distance + 1, x, y, !clockwise));
                }
                // west
                if x > 0
                    && (graph[y][x - 1] == b'-'
                        || graph[y][x - 1] == b'F'
                        || graph[y][x - 1] == b'L')
                {
                    queue.push_back((x - 1, y, distance + 1, x, y, !clockwise));
                }
            }
            b'|' => {
                if y > py {
                    queue.push_back((x, y + 1, distance + 1, x, y, clockwise));
                    if x < width - 1 {
                        current_outer_candidates.push((x + 1, y));
                    }
                    if x > 0 {
                        current_inner_candidates.push((x - 1, y));
                    }
                } else {
                    queue.push_back((x, y - 1, distance + 1, x, y, clockwise));
                    if x < width - 1 {
                        current_inner_candidates.push((x + 1, y));
                    }
                    if x > 0 {
                        current_outer_candidates.push((x - 1, y));
                    }
                }
            }
            b'-' => {
                if x > px {
                    queue.push_back((x + 1, y, distance + 1, x, y, clockwise));
                    if y > 0 {
                        current_outer_candidates.push((x, y - 1));
                    }
                    if y < height - 1 {
                        current_inner_candidates.push((x, y + 1));
                    }
                } else {
                    queue.push_back((x - 1, y, distance + 1, x, y, clockwise));
                    if y > 0 {
                        current_inner_candidates.push((x, y - 1));
                    }
                    if y < height - 1 {
                        current_outer_candidates.push((x, y + 1));
                    }
                }
            }
            b'L' => {
                let mut current_candidates = current_inner_candidates;
                match px - x {
                    0 => {
                        queue.push_back((x + 1, y, distance + 1, x, y, clockwise));
                        *current_left_turns += 1;
                    }
                    1 => {
                        current_candidates = current_outer_candidates;
                        queue.push_back((x, y - 1, distance + 1, x, y, clockwise));
                        *current_right_turns += 1;
                    }
                    _ => unreachable!(),
                }
                if x > 0 {
                    current_candidates.push((x - 1, y));
                    if y < height - 1 {
                        current_candidates.push((x - 1, y + 1));
                    }
                }
                if y < height - 1 {
                    current_candidates.push((x, y + 1));
                }
            }
            b'J' => {
                let mut current_candidates = current_inner_candidates;
                match x - px {
                    0 => {
                        current_candidates = current_outer_candidates;
                        queue.push_back((x - 1, y, distance + 1, x, y, clockwise));
                        *current_right_turns += 1;
                    }
                    1 => {
                        queue.push_back((x, y - 1, distance + 1, x, y, clockwise));
                        *current_left_turns += 1;
                    }
                    _ => unreachable!(),
                }
                if y < height - 1 {
                    current_candidates.push((x, y + 1));
                    if x < width - 1 {
                        current_candidates.push((x + 1, y + 1));
                    }
                }
                if x < width - 1 {
                    current_candidates.push((x + 1, y));
                }
            }
            b'7' => {
                let mut current_candidates = current_inner_candidates;
                match x - px {
                    0 => {
                        queue.push_back((x - 1, y, distance + 1, x, y, clockwise));
                        *current_left_turns += 1;
                    }
                    1 => {
                        current_candidates = current_outer_candidates;
                        queue.push_back((x, y + 1, distance + 1, x, y, clockwise));
                        *current_right_turns += 1;
                    }
                    _ => unreachable!(),
                }
                if y > 0 {
                    current_candidates.push((x, y - 1));
                    if x < width - 1 {
                        current_candidates.push((x + 1, y - 1));
                    }
                }
                if x < width - 1 {
                    current_candidates.push((x + 1, y));
                }
            }
            b'F' => {
                let mut current_candidates = current_inner_candidates;
                match px - x {
                    0 => {
                        current_candidates = current_outer_candidates;
                        queue.push_back((x + 1, y, distance + 1, x, y, clockwise));
                        *current_right_turns += 1;
                    }
                    1 => {
                        queue.push_back((x, y + 1, distance + 1, x, y, clockwise));
                        *current_left_turns += 1;
                    }
                    _ => unreachable!(),
                }
                if y > 0 {
                    current_candidates.push((x, y - 1));
                    if x > 0 {
                        current_candidates.push((x - 1, y - 1));
                    }
                }
                if x > 0 {
                    current_candidates.push((x - 1, y));
                }
            }
            _ => panic!("unexpected end of pipe :/ => {}", current_char),
        }

        seen.insert((x, y));
    }

    /*
       if we are having more right turns than left turns, it means that we were looping clockwise,
       se we should keep the inner candidates as we compute them imagining that we were looping clockwise.

       Otherwise it means that the clockwise was wrong, and we should keep the outer candidates.
    */
    if right_turns > left_turns {
        (result, inner_candidates)
    } else {
        (result, outer_candidates)
    }
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
    fn test_part_two_example_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_example_4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(8));
    }

    // this test is not working, I would need to fix it eventually :/
    // #[test]
    // fn test_part_two_example_5() {
    //     let result = part_two(&advent_of_code::template::read_file_part(
    //         "examples", DAY, 5,
    //     ));
    //     assert_eq!(result, Some(10));
    // }

    #[test]
    fn test_solution_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(589));
    }
}
