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

pub fn part_two(input: &str) -> Option<u32> {
    let it = input.as_bytes().lines();
    let mut solution = 0;
    for line in it.flatten() {
        let num = find_number(&replace_word_digit(&line));
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

fn replace_word_digit(s: &str) -> String {
    let mut result = s.replace("one", "on1e");
    result = result.replace("two", "t2wo");
    result = result.replace("three", "thr3ee");
    result = result.replace("four", "fo4ur");
    result = result.replace("five", "fi5ve");
    result = result.replace("six", "si6x");
    result = result.replace("seven", "sev7en");
    result = result.replace("eight", "ei8ght");
    result = result.replace("nine", "ni9ne");

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
