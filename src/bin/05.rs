use std::{ops::RangeInclusive, str::FromStr};

use anyhow::bail;
use itertools::Itertools;

advent_of_code::solution!(5);

struct Input {
    fresh: Vec<RangeInclusive<usize>>,
    available: Vec<usize>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((fresh_str, available_str)) = s.split_once("\n\n") else {
            bail!("Invalid str");
        };
        let fresh = fresh_str
            .lines()
            .map(|l| {
                let Some((start, end)) = l.split_once("-") else {
                    bail!("Invalid str");
                };
                let start = start.parse()?;
                let end = end.parse()?;
                Ok(start..=end)
            })
            .collect::<Result<_, _>>()?;
        let available = available_str
            .lines()
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;
        Ok(Self { fresh, available })
    }
}

fn is_overlapping(lhs: &RangeInclusive<usize>, rhs: &RangeInclusive<usize>) -> bool {
    (lhs.start() <= rhs.end()) && (rhs.start() <= lhs.end())
}

fn merge(lhs: &RangeInclusive<usize>, rhs: &RangeInclusive<usize>) -> RangeInclusive<usize> {
    let start = *lhs.start().min(rhs.start());
    let end = *lhs.end().max(rhs.end());
    start..=end
}

pub fn part_one(input: &str) -> Option<usize> {
    let Input { fresh, available } = input.parse().expect("Invalid parse");
    let total = available
        .into_iter()
        .filter(|n| fresh.iter().any(|r| r.contains(n)))
        .count();
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input: Input = input.parse().expect("Invalid parse");
    let mut fresh = input.fresh;
    fresh.sort_by_key(|r| *r.start());

    let total = fresh
        .into_iter()
        .coalesce(|lhs, rhs| {
            if is_overlapping(&lhs, &rhs) {
                Ok(merge(&lhs, &rhs))
            } else {
                Err((lhs, rhs))
            }
        })
        .map(|r| r.end() - r.start() + 1)
        .sum();
    Some(total)
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
        assert_eq!(result, Some(14));
    }
}
