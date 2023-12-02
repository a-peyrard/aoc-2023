use std::io::BufRead;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let it = input.as_bytes().lines();
    let mut solution = 0;
    for line in it.flatten() {
        let num = find_number(&line);
        solution += num;
    }

    Some(solution)
}

fn find_number(s: &String) -> u32 {
    let bytes = s.as_bytes();
    let mut first: u32 = 0;
    let mut last: u32 = 0;
    for b in bytes {
        if b.is_ascii_digit() {
            first = (b - b'0') as u32;
            break;
        }
    }
    for b in bytes.iter().rev() {
        if b.is_ascii_digit() {
            last = (b - b'0') as u32;
            break;
        }
    }

    first * 10 + last
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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
