use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let directions = lines
        .first()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|c| match c {
            b'R' => 1,
            b'L' => 0,
            _ => panic!("unexpected direction"),
        })
        .collect::<Vec<usize>>();

    let mut mapping: HashMap<String, usize> = HashMap::new();
    let mut current: usize = 0;
    let mut nodes: Vec<[usize; 2]> = vec![[0, 0]; lines.len() - 2];
    for line in lines.iter().skip(2) {
        let (pos, successors) = parse_node(line, &mut mapping, &mut current);

        nodes[pos] = successors;
    }

    follow_instructions(
        directions,
        nodes,
        *mapping.get("AAA").unwrap(),
        *mapping.get("ZZZ").unwrap(),
    )
}

fn parse_node(
    line: &str,
    mapping: &mut HashMap<String, usize>,
    current: &mut usize,
) -> (usize, [usize; 2]) {
    let (pos_s, successors_raw) = line.split_once(" = ").unwrap();
    let pos = find_or_generate(mapping, current, pos_s);

    let (left_s, right_s) = successors_raw[1..successors_raw.len() - 1]
        .split_once(", ")
        .unwrap();
    let left = find_or_generate(mapping, current, left_s);
    let right = find_or_generate(mapping, current, right_s);

    (pos, [left, right])
}

fn follow_instructions(
    directions: Vec<usize>,
    nodes: Vec<[usize; 2]>,
    start: usize,
    target: usize,
) -> Option<u64> {
    let len = directions.len();
    let mut step: u64 = 0;
    let mut position: usize = start;
    while position != target {
        let direction = directions[(step % len as u64) as usize];
        position = nodes[position][direction];
        step += 1;
    }

    Some(step)
}

fn find_or_generate(
    mapping: &mut HashMap<String, usize>,
    current: &mut usize,
    location: &str,
) -> usize {
    if let Some(&index) = mapping.get(location) {
        index
    } else {
        let index = *current;
        *current += 1;
        mapping.insert(location.to_string(), index);
        index
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_example2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_example1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two_example2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, None);
    }

    #[test]
    fn test_solution_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(16897));
    }
}
