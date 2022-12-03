use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        } else if self == &Move::Rock && other == &Move::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

pub fn process_part1(input: &str) -> u32 {
    let result = input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect();
            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Equal) => 3 + moves[1] as u32,
                Some(Ordering::Greater) => 0 + moves[1] as u32,
                Some(Ordering::Less) => 6 + moves[1] as u32,
                None => {
                    panic!("moves should be comparable")
                }
            }
        })
        .sum::<u32>();

    result
}

pub fn process_part2(input: &str) -> u32 {
    let result = input
        .lines()
        .map(|line| {
            let moves: Vec<&str> = line.split(" ").collect();
            let opponent_move = moves[0].parse::<Move>().unwrap();
            match moves[1] {
                "X" => {
                    let required_move = match opponent_move {
                        Move::Rock => Move::Scissors,
                        Move::Paper => Move::Rock,
                        Move::Scissors => Move::Paper,
                    };
                    0 + required_move as u32
                }
                "Y" => 3 + opponent_move as u32,

                "Z" => {
                    let required_move = match opponent_move {
                        Move::Rock => Move::Paper,
                        Move::Paper => Move::Scissors,
                        Move::Scissors => Move::Rock,
                    };
                    6 + required_move as u32
                }

                _ => {
                    panic!("unexpected response")
                }
            }
        })
        .sum::<u32>();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, 15);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(&INPUT);
        assert_eq!(result, 12);
    }
}
