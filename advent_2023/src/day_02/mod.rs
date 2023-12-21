//! Solutions to 2023 day 02 problems
//! --- Day 2: Cube Conundrum ---

/// Returns the sum of the IDs of possible games.
pub fn one(file_path: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the sum of the IDs of possible games";
        let expected = 8;
        let actual = one("input/02-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
