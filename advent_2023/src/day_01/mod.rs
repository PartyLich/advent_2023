//! Solutions to 2023 day 1 problems
//! --- Day 1: Trebuchet?!  ---

/// Returns the sum of all of the calibration values. Calibration values can be found by combining
/// the first digit and the last digit (in that order) to form a single two-digit number.
pub fn one(file_path: &str) -> usize {
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
}
