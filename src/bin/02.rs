use std::io::BufRead;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let known_max = Rgb::new(12, 13, 14);
    Some(
        input
            .as_bytes()
            .lines()
            .flatten()
            .map(|s| s.split(':').nth(1).unwrap().to_owned())
            .map(|s| {
                s.split(';')
                    .map(str::trim)
                    .map(Rgb::parse)
                    .collect::<Vec<_>>()
            })
            .map(Rgb::max)
            .enumerate()
            .map(|(i, rgb)| match rgb.less_or_equal(&known_max) {
                true => i as u32 + 1,
                false => 0,
            })
            .sum::<u32>(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn parse(s: &str) -> Self {
        let tokens = s.split(',').map(str::trim);
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for token in tokens {
            if token.ends_with("red") {
                r = token[0..token.len() - 4].parse::<u8>().unwrap()
            } else if token.ends_with("green") {
                g = token[0..token.len() - 6].parse::<u8>().unwrap()
            } else if token.ends_with("blue") {
                b = token[0..token.len() - 5].parse::<u8>().unwrap()
            }
        }

        Self::new(r, g, b)
    }

    fn max(vec: Vec<Rgb>) -> Self {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for rgb in vec {
            r = r.max(rgb.r);
            g = g.max(rgb.g);
            b = b.max(rgb.b);
        }

        Self::new(r, g, b)
    }

    fn less_or_equal(&self, other: &Self) -> bool {
        self.r <= other.r && self.g <= other.g && self.b <= other.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
