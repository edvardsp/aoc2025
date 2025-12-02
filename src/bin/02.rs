use std::ops::Range;
use std::str::FromStr;

use anyhow::bail;

advent_of_code::solution!(2);

struct Input {
    ids: Vec<Range<usize>>,
}

impl Input {
    fn id_iter(&self) -> impl Iterator<Item = usize> {
        self.ids.iter().flat_map(|range| range.clone())
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ids = s
            .split(",")
            .map(|n| {
                let Some((start, end)) = n.split_once("-") else {
                    bail!("Invalid range");
                };
                let start: usize = start.parse()?;
                let end: usize = end.parse()?;
                Ok(start..(end + 1))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { ids })
    }
}

struct DigitBytes {
    buffer: [u8; 64],
    len: usize,
}

impl DigitBytes {
    fn new(mut n: usize) -> Self {
        let mut buffer = [0; 64];
        let mut len = 0;
        while n != 0 {
            buffer[len] = (n % 10) as u8;
            n /= 10;
            len += 1;
        }
        Self { buffer, len }
    }

    fn as_bytes(&self) -> &[u8] {
        &self.buffer[..self.len]
    }
}

fn is_invalid(n: &usize) -> bool {
    let digit = DigitBytes::new(*n);
    let bytes = digit.as_bytes();
    if !bytes.len().is_multiple_of(2) {
        return false;
    }

    let (lhs, rhs) = bytes.split_at(bytes.len() / 2);
    lhs == rhs
}

fn is_invalid2(n: &usize) -> bool {
    let digit = DigitBytes::new(*n);
    let bytes = digit.as_bytes();
    let len = bytes.len();
    (1..=len / 2)
        .filter(|chunk_len| len.is_multiple_of(*chunk_len))
        .any(|chunk_len| {
            let head = &bytes[0..chunk_len];
            bytes[chunk_len..]
                .chunks_exact(chunk_len)
                .all(|chunk| head == chunk)
        })
}

pub fn part_one(input: &str) -> Option<usize> {
    let input: Input = input.parse().expect("Invalid parse");
    let invalid_ids = input.id_iter().filter(is_invalid).sum();
    Some(invalid_ids)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input: Input = input.parse().expect("Invalid parse");
    let invalid_ids = input.id_iter().filter(is_invalid2).sum();
    Some(invalid_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
