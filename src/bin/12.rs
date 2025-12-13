use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(12, 1);

struct Input {
    shapes: Vec<Vec<char>>,
    regions: Vec<Region>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.split("\n\n").collect();
        let shapes = lines[0..lines.len() - 1]
            .iter()
            .map(|l| l.lines().skip(1).flat_map(|r| r.chars()).collect())
            .collect();
        let regions = lines[lines.len() - 1]
            .lines()
            .map(|l| l.parse())
            .collect::<Result<_, _>>()?;
        Ok(Self { shapes, regions })
    }
}

struct Region {
    shape: (usize, usize),
    quantity: Vec<usize>,
}

impl Region {
    fn area(&self) -> usize {
        self.shape.0 * self.shape.1
    }
}

impl FromStr for Region {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (shape_str, quantity_str) = s.split_once(": ").unwrap();
        let (shape_x, shape_y) = shape_str.split_once('x').unwrap();
        let shape_x = shape_x.parse()?;
        let shape_y = shape_y.parse()?;
        let shape = (shape_x, shape_y);
        let quantity = quantity_str
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self { shape, quantity })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let Input { shapes, regions } = input.parse().expect("Invalid parse");

    let tiles: HashMap<usize, usize> = shapes
        .iter()
        .enumerate()
        .map(|(i, shape)| {
            let tiles = shape.iter().filter(|c| **c == '#').count();
            (i, tiles)
        })
        .collect();

    let value = regions
        .iter()
        .filter(|region| {
            let area = region.area();
            let total_tiles: usize = region
                .quantity
                .iter()
                .enumerate()
                .map(|(i, q)| q * tiles[&i])
                .sum();
            total_tiles <= area
        })
        .count();

    Some(value)
}
