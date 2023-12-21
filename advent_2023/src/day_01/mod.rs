//! Solutions to 2023 day 1 problems
//! --- Day 1: Trebuchet?!  ---
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

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

lazy_static! {
    static ref DIGIT_RE: Regex =
        Regex::new(r#".*?(\d|one|two|three|four|five|six|seven|eight|nine)"#).unwrap();
    static ref END_DIGIT_RE: Regex =
        Regex::new(r#".*(\d|one|two|three|four|five|six|seven|eight|nine).*?$"#).unwrap();
}

/// A single numeric digit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Digit(usize);

impl FromStr for Digit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.len() == 1 && s.chars().last().unwrap().is_ascii_digit() => {
                Ok(s.parse().unwrap())
            }
            "one" => Ok(1_usize),
            "two" => Ok(2),
            "three" => Ok(3),
            "four" => Ok(4),
            "five" => Ok(5),
            "six" => Ok(6),
            "seven" => Ok(7),
            "eight" => Ok(8),
            "nine" => Ok(9),
            _ => Err(format!("Failed to parse {}", s)),
        }
        .map(Digit)
    }
}

fn parse_two(line: &str) -> usize {
    let first_digit = DIGIT_RE
        .captures_iter(line)
        .next()
        .map(|digi| digi.get(1).unwrap().into())
        .map(Digit::from_str)
        .unwrap()
        .unwrap()
        .0;
    let last_digit = END_DIGIT_RE
        .captures_iter(line)
        .last()
        .map(|digi| digi.get(1).unwrap().into())
        .map(Digit::from_str)
        .unwrap_or(Ok(Digit(first_digit)))
        .unwrap()
        .0;

    (first_digit * 10) + last_digit
}

/// Returns the sum of all of the calibration values. Calibration values can be found by combining
/// the first digit and the last digit (in that order) to form a single two-digit number. Some of
/// the digits are actually spelled out with letters: one, two, three, four, five, six, seven,
/// eight, and nine also count as valid "digits".
pub fn two(file_path: &str) -> usize {
    read_file(file_path).lines().map(parse_two).sum()
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
    fn parse_part_two() {
        let msg = "should parse a line with a single digit";
        let expected = 99;
        let actual = parse_two("hzfplpdt9");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn handle_overlaps() {
        let msg = "should parse the last digit with character overlaps";
        let expected = 82;
        let actual = parse_two("8ninefivegzk7ftqbceightwogfv");
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
