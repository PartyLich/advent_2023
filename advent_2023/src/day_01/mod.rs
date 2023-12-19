//! Solutions to 2023 day 1 problems
//! --- Day 1: Trebuchet?!  ---
use crate::read_file;

fn parse_line(line: &str) -> usize {
    let digits = line
        .chars()
        .filter(|char| char.is_ascii_digit())
        .collect::<Vec<_>>();

    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
        .parse()
        .unwrap()
}

/// Returns the sum of all of the calibration values. Calibration values can be found by combining
/// the first digit and the last digit (in that order) to form a single two-digit number.
pub fn one(file_path: &str) -> usize {
    read_file(file_path).lines().map(parse_line).sum::<usize>()
}

/// Returns the sum of all of the calibration values. Calibration values can be found by combining
/// the first digit and the last digit (in that order) to form a single two-digit number. Some of
/// the digits are actually spelled out with letters: one, two, three, four, five, six, seven,
/// eight, and nine also count as valid "digits".
pub fn two(file_path: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the sum of all of the calibration values";
        let expected = 142;
        let actual = one("input/01-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return the sum of all of the calibration values";
        let expected = 281;
        let actual = two("input/01-t2.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
