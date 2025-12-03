use std::str::FromStr;

advent_of_code::solution!(3);

struct Input {
    banks: Vec<Vec<u64>>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let banks = s
            .lines()
            .map(|line| line.as_bytes().iter().map(|n| (n - b'0') as u64).collect())
            .collect();
        Ok(Self { banks })
    }
}

// Don't use .iter().max() since it returns the last largest item. We want the first largest item. Return (pos, val)
fn find_largest_battery(batteries: &[u64]) -> (usize, u64) {
    batteries.iter().copied().enumerate().fold(
        (0, 0),
        |lhs, rhs| {
            if rhs.1 > lhs.1 { rhs } else { lhs }
        },
    )
}

fn find_largest_joltage(mut batteries: &[u64], num_digits: usize) -> u64 {
    (0..num_digits).fold(0, |total, digit| {
        let cutoff = batteries.len() - (num_digits - digit);
        let (i, battery) = find_largest_battery(&batteries[0..=cutoff]);
        batteries = &batteries[i + 1..];
        10 * total + battery
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let Input { banks } = input.parse().expect("Invalid parse");
    let total = banks
        .into_iter()
        .map(|bank| find_largest_joltage(&bank, 2))
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let Input { banks } = input.parse().expect("Invalid parse");
    let total = banks
        .into_iter()
        .map(|bank| find_largest_joltage(&bank, 12))
        .sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
