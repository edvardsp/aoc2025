use anyhow::bail;
use std::str::FromStr;

advent_of_code::solution!(1);

struct Input {
    directions: Vec<Direction>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let directions = s
            .lines()
            .map(Direction::from_str)
            .collect::<Result<_, _>>()?;
        Ok(Self { directions })
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left(i32),
    Right(i32),
}

use Direction::{Left, Right};

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_at(1);
        let value = tail.parse()?;
        let rotation = match head {
            "L" => Left(value),
            "R" => Right(value),
            _ => bail!("Invalid direction str {}", head),
        };
        Ok(rotation)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let Input { directions } = input.parse().expect("Invalid parse");

    let password = directions
        .iter()
        .scan(50, |dial, dir| {
            *dial = match dir {
                Left(l) => {
                    *dial -= l;
                    while *dial < 0 {
                        *dial += 100;
                    }
                    *dial
                }
                Right(r) => {
                    *dial += r;
                    while *dial > 99 {
                        *dial -= 100;
                    }
                    *dial
                }
            };
            Some(*dial)
        })
        .filter(|dial| *dial == 0)
        .count();
    Some(password)
}

pub fn part_two(input: &str) -> Option<usize> {
    let Input { directions } = input.parse().expect("Invalid parse");

    let mut dial = 50;
    let mut password = 0;
    for dir in &directions {
        let was_zero = dial == 0;
        match *dir {
            Left(l) => {
                password += l.div_euclid(100) as usize;
                dial -= l.rem_euclid(100);
                if dial < 0 {
                    dial += 100;
                    if !was_zero {
                        password += 1;
                    }
                } else if dial == 0 {
                    password += 1;
                }
            }
            Right(r) => {
                password += r.div_euclid(100) as usize;
                dial += r.rem_euclid(100);
                if dial > 99 {
                    dial -= 100;
                    if !was_zero {
                        password += 1;
                    }
                } else if dial == 0 {
                    password += 1;
                }
            }
        }
    }

    Some(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
