//! Solutions to 2023 day 04 problems
//! --- Day 4: Scratchcards ---

pub fn one(file_path: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the point total of the winning cards";
        let expected = 13;
        let actual = one("input/04-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
