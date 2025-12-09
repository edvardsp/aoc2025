use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;

use rayon::prelude::*;

advent_of_code::solution!(9);

type Idx = (usize, usize);

struct Input {
    tiles: Vec<Idx>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(",").unwrap();
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();
                (x, y)
            })
            .collect();
        Ok(Self { tiles })
    }
}

fn box_area(c0: &Idx, c1: &Idx) -> usize {
    let (x0, y0) = c0;
    let (x1, y1) = c1;
    (x0.abs_diff(*x1) + 1) * (y0.abs_diff(*y1) + 1)
}

fn covers(outer: &RangeInclusive<usize>, inner: &RangeInclusive<usize>) -> bool {
    outer.start() <= inner.start() && inner.end() <= outer.end()
}

fn contains_box(
    verticals: &HashMap<usize, Vec<RangeInclusive<usize>>>,
    c0: &Idx,
    c1: &Idx,
) -> bool {
    let (x0, y0) = *c0;
    let (x1, y1) = *c1;
    let x_min = x0.min(x1);
    let x_max = x0.max(x1);
    let y_min = y0.min(y1);
    let y_max = y0.max(y1);

    let x_inner = x_min..=x_max;
    (y_min..=y_max)
        .map(|y| &verticals[&y])
        .all(|xs| xs.iter().any(|x_outer| covers(x_outer, &x_inner)))
}

pub fn part_one(input: &str) -> Option<usize> {
    let Input { tiles } = input.parse().expect("Invalid parse");

    tiles
        .iter()
        .tuple_combinations()
        .par_bridge()
        .map(|(c0, c1)| box_area(c0, c1))
        .max()
}

pub fn part_two(input: &str) -> Option<usize> {
    let Input { tiles } = input.parse().expect("Invalid parse");

    let mut verticals: HashMap<usize, Vec<usize>> = HashMap::new();

    let corners_iter = tiles.iter().cycle().take(tiles.len() + 2).tuple_windows();
    for (&(x0, y0), &(x1, y1), &(x2, y2)) in corners_iter {
        match (x1.cmp(&x2), y1.cmp(&y2)) {
            // Horizontal edge going right, add a vertical if we came from below
            (Ordering::Less, _) if y1 < y0 => {
                verticals.entry(y1).or_default().push(x1);
            }
            // Horizontal edge going left, add a vertical if we came from above
            (Ordering::Greater, _) if y1 > y0 => {
                verticals.entry(y1).or_default().push(x1);
            }
            // Vertical edge going down
            (Ordering::Equal, Ordering::Less) => {
                for y in (y1 + 1)..y2 {
                    verticals.entry(y).or_default().push(x1);
                }
                if x1 > x0 {
                    verticals.entry(y1).or_default().push(x1);
                }
            }
            // Vertical edge going up
            (Ordering::Equal, Ordering::Greater) => {
                for y in (y2 + 1)..y1 {
                    verticals.entry(y).or_default().push(x1);
                }
                if x1 < x0 {
                    verticals.entry(y1).or_default().push(x1);
                }
            }
            _ => {}
        }
    }

    let verticals: HashMap<usize, Vec<RangeInclusive<usize>>> = verticals
        .into_iter()
        .map(|(y, mut xs)| {
            xs.sort_unstable();
            let ranges = xs.chunks_exact(2).map(|ch| ch[0]..=ch[1]).collect();
            (y, ranges)
        })
        .collect();

    let mut pairs: Vec<(Idx, Idx)> = tiles.iter().copied().tuple_combinations().collect();
    pairs.par_sort_unstable_by_key(|(c0, c1)| Reverse(box_area(c0, c1)));
    pairs
        .into_par_iter()
        .find_first(|(c0, c1)| contains_box(&verticals, c0, c1))
        .map(|(c0, c1)| box_area(&c0, &c1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
