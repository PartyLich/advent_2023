//! Solutions to 2023 day 03 problems
//! --- Day 3: Gear Ratios ---
use std::collections::HashSet;

use crate::{read_file, Coord};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Num {
    row: usize,
    start: usize,
    end: usize,
    value: usize,
}

/// Parse a schematic to locate symbols, digits, and complete numbers.
fn parse_schematic(file: &str) -> (HashSet<Coord>, HashSet<Coord>, Vec<Num>) {
    let mut symbols = HashSet::new();
    let mut digits = HashSet::new();
    let mut numbers = Vec::new();

    for (row, line) in file.lines().enumerate() {
        let mut start = None;
        let mut end = None;
        let mut save_num = |start: &mut Option<usize>, end: &mut Option<usize>| {
            if let Some(start) = start.take() {
                let end = end.take().unwrap();
                numbers.push(Num {
                    row,
                    start,
                    end,
                    value: line[start..=end].parse::<usize>().unwrap(),
                })
            }
        };

        for (col, ch) in line.chars().enumerate() {
            match ch {
                '.' => save_num(&mut start, &mut end),
                ch if ch.is_ascii_digit() => {
                    if start.is_some() {
                        end = end.map(|x| x + 1);
                    } else {
                        start = Some(col);
                        end = Some(col);
                    }

                    digits.insert((row as isize, col as isize).into());
                }
                _ => {
                    save_num(&mut start, &mut end);
                    symbols.insert((row as isize, col as isize).into());
                }
            };
        }

        save_num(&mut start, &mut end);
    }

    (symbols, digits, numbers)
}

/// Returns the sum of the engine part numbers.
pub fn one(file_path: &str) -> usize {
    let input = read_file(file_path);
    let mut result = Vec::new();

    let (symbols, mut digits, mut numbers) = parse_schematic(&input);
    for symbol in symbols {
        // check neighbors for digit
        [
            //prev row
            Coord(-1, 0),
            Coord(-1, 1),
            Coord(-1, -1),
            // current row
            Coord(0, 1),
            Coord(0, -1),
            // next row
            Coord(1, -1),
            Coord(1, 0),
            Coord(1, 1),
        ]
        .iter()
        .filter_map(|offset| {
            let location = *offset + symbol;
            if digits.remove(&location) {
                Some(location)
            } else {
                None
            }
        })
        .for_each(|coord| {
            let parts: (Vec<Num>, Vec<Num>) = numbers.iter().partition(|num| {
                num.row == coord.0 as usize && (num.start..=num.end).contains(&(coord.1 as usize))
            });

            numbers = parts.1;
            result.extend(parts.0.into_iter().map(|num| num.value));
        });
    }

    result.iter().sum()
}

/// Returns the sum of the gear ratios.
pub fn two(file_path: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the sum of the engine part numbers";
        let expected = 4361;
        let actual = one("input/03-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return the sum of the gear ratios";
        let expected = 467835;
        let actual = two("input/03-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
