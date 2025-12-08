use anyhow::Context;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

advent_of_code::solution!(8);

struct Input {
    junctions: Vec<Point>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let junctions = s.lines().map(|l| l.parse()).collect::<Result<_, _>>()?;
        Ok(Self { junctions })
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Point {
    fn distance(&self, other: &Self) -> u64 {
        let x = self.x.abs_diff(other.x) as u64;
        let y = self.y.abs_diff(other.y) as u64;
        let z = self.z.abs_diff(other.z) as u64;
        (x * x + y * y + z * z).isqrt()
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',').map(|n| n.parse());
        let x = iter.next().context("Invalid s")??;
        let y = iter.next().context("Invalid s")??;
        let z = iter.next().context("Invalid s")??;
        Ok(Self { x, y, z })
    }
}

fn make_pairs(junctions: &[Point]) -> impl Iterator<Item = (Point, Point)> {
    junctions
        .iter()
        .tuple_combinations()
        .map(|(p0, p1)| (*p0, *p1, p0.distance(p1)))
        .sorted_by_key(|(_, _, distance)| *distance)
        .map(|(p0, p1, _)| (p0, p1))
}

struct Decorations {
    circuits: Vec<HashSet<Point>>,
    belongs: HashMap<Point, usize>,
}

impl Decorations {
    fn new(junctions: &[Point]) -> Self {
        let circuits = junctions.iter().map(|p| HashSet::from([*p])).collect();
        let belongs = junctions.iter().enumerate().map(|(i, p)| (*p, i)).collect();
        Self { circuits, belongs }
    }

    fn assemble(&mut self, pair: &(Point, Point)) {
        let (p0, p1) = pair;
        let pos0 = self.belongs[p0];
        let pos1 = self.belongs[p1];
        if pos0 == pos1 {
            return;
        }

        let (extend_pos, clear_pos) = if self.circuits[pos0].len() < self.circuits[pos1].len() {
            (pos1, pos0)
        } else {
            (pos0, pos1)
        };

        let mut tmp_circuit = HashSet::new();
        std::mem::swap(&mut tmp_circuit, &mut self.circuits[clear_pos]);

        for p in &tmp_circuit {
            self.belongs.insert(*p, pos0);
        }
        self.circuits[extend_pos].extend(tmp_circuit);
    }

    fn product_of(&self, top: usize) -> usize {
        self.circuits
            .iter()
            .map(|c| c.len())
            .sorted_by_key(|l| Reverse(*l))
            .take(top)
            .product()
    }

    fn all_connected(&self) -> bool {
        // All junctions are connected when only a single non-empty circuit exists
        self.circuits.iter().filter(|c| !c.is_empty()).count() == 1
    }
}

pub fn part_one_solve(input: &str, combine: usize) -> Option<usize> {
    let Input { junctions } = input.parse().expect("Invalid parse");

    let mut decorations = Decorations::new(&junctions);

    make_pairs(&junctions)
        .take(combine)
        .for_each(|pair| decorations.assemble(&pair));
    Some(decorations.product_of(3))
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_solve(input, 1000)
}

pub fn part_two(input: &str) -> Option<usize> {
    let Input { junctions } = input.parse().expect("Invalid parse");

    let mut decorations = Decorations::new(&junctions);

    make_pairs(&junctions)
        .find(|pair| {
            decorations.assemble(pair);
            decorations.all_connected()
        })
        .map(|(p0, p1)| p0.x as usize * p1.x as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_solve(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
