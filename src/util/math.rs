pub fn lcm(numbers: &[usize]) -> usize {
    let mut result = 1;
    let mut div = 2;
    let mut numbers = numbers.to_vec();

    while numbers.iter().any(|&n| n != 1) {
        let mut divided = false;
        for n in numbers.iter_mut() {
            if *n % div == 0 {
                *n /= div;
                divided = true;
            }
        }

        if divided {
            result *= div;
        } else {
            div += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(&[2, 3, 4]), 12);
        assert_eq!(lcm(&[2, 3, 4, 5]), 60);
        assert_eq!(lcm(&[2, 3, 4, 5, 6]), 60);
        assert_eq!(lcm(&[2, 3, 4, 5, 6, 7]), 420);
        assert_eq!(lcm(&[2, 3, 4, 5, 6, 7, 8]), 840);
        assert_eq!(lcm(&[2, 3, 4, 5, 6, 7, 8, 9]), 2520);
        assert_eq!(lcm(&[2, 3, 4, 5, 6, 7, 8, 9, 10]), 2520);
        assert_eq!(lcm(&[37, 23, 11]), 9361);
    }
}
