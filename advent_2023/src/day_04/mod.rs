//! Solutions to 2023 day 04 problems
//! --- Day 4: Scratchcards ---
use crate::read_file;

fn parse_line(line: &str) -> (Vec<usize>, Vec<usize>) {
    let parse_numbers = |num_str: &str| -> Vec<usize> {
        num_str
            .split_whitespace()
            .map(|num_str| num_str.parse())
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    };

    let (left, have_str) = line.split_once(" | ").unwrap();
    let (_game, win_str) = left.split_once(": ").unwrap();

    (parse_numbers(win_str), parse_numbers(have_str))
}

/// Return the point total of the winning cards.
pub fn one(file_path: &str) -> usize {
    read_file(file_path)
        .lines()
        .map(parse_line)
        .map(|(winners, have)| {
            let count: u32 = have
                .into_iter()
                .filter(|n| winners.contains(n))
                .count()
                .try_into()
                .unwrap();

            if count == 0 {
                return 0;
            }

            2_usize.pow(count - 1)
        })
        .sum()
}

/// Returns the total number of scratchcards.
pub fn two(file_path: &str) -> usize {
    let win_list: Vec<_> = read_file(file_path)
        .lines()
        .map(parse_line)
        .map(|(winners, have)| have.into_iter().filter(|n| winners.contains(n)).count())
        .collect();
    let mut cards = vec![1_usize; win_list.len()];
    for (idx, wins) in win_list.iter().enumerate().filter(|(_idx, &n)| n > 0) {
        for i in (idx + 1)..(idx + 1 + wins) {
            cards[i] += cards[idx];
        }
    }

    cards.iter().sum()
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

    #[test]
    fn part_two() {
        let msg = "should return the total number of scratchcards";
        let expected = 30;
        let actual = two("input/04-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
