//! Solutions to 2023 day 02 problems
//! --- Day 2: Cube Conundrum ---

use crate::read_file;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct GameInfo {
    pub red: usize,
    pub blue: usize,
    pub green: usize,
}

fn parse_game(str: &str) -> Vec<GameInfo> {
    let (_game_number, data) = str.split_once(':').unwrap();
    data.split(';')
        .map(|game_str| {
            game_str.split(',').fold(GameInfo::default(), |acc, block| {
                let mut parts = block.split_whitespace();
                let count_str = parts.next().unwrap();
                let count = count_str.parse::<usize>().unwrap();

                let color = parts.last().unwrap();
                match color {
                    "red" => GameInfo {
                        red: acc.red + count,
                        ..acc
                    },
                    "blue" => GameInfo {
                        blue: acc.blue + count,
                        ..acc
                    },
                    "green" => GameInfo {
                        green: acc.green + count,
                        ..acc
                    },
                    _ => acc,
                }
            })
        })
        .collect()
}

fn is_possible(max: GameInfo, game: &[GameInfo]) -> bool {
    game.iter()
        .all(|f| f.red <= max.red && f.blue <= max.blue && f.green <= max.green)
}

/// Returns the sum of the IDs of possible games.
pub fn one(file_path: &str) -> usize {
    read_file(file_path)
        .lines()
        .map(parse_game)
        .enumerate()
        .filter(|(_, game_data)| {
            is_possible(
                GameInfo {
                    red: 12,
                    green: 13,
                    blue: 14,
                },
                game_data,
            )
        })
        .fold(0, |acc, (idx, _)| acc + (idx + 1))
}

/// Returns the sum of the minimum number of red, green, and blue cubes in each game multiplied
/// together.
pub fn two(file_path: &str) -> usize {
    read_file(file_path)
        .lines()
        .map(parse_game)
        .map(|game_data| {
            game_data
                .into_iter()
                .fold(GameInfo::default(), |acc, game| GameInfo {
                    red: acc.red.max(game.red),
                    green: acc.green.max(game.green),
                    blue: acc.blue.max(game.blue),
                })
        })
        .map(|game| game.red * game.green * game.blue)
        .sum()
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

    #[test]
    fn part_two() {
        let msg = "should return the sum of the 'power' of the minimum cube set";
        let expected = 2286;
        let actual = two("input/02-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
