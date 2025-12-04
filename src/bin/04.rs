use std::str::FromStr;

use ndarray::{Array2, s};

advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Map {
    tiles: Array2<char>,
}

impl Map {
    fn paperrolls(&self) -> impl Iterator<Item = (usize, usize)> {
        self.tiles
            .indexed_iter()
            .filter_map(|(idx, elem)| if *elem == '@' { Some(idx) } else { None })
    }

    fn space(&self, idx: (usize, usize)) -> usize {
        let (y, x) = idx;
        self.tiles
            .slice(s![y - 1..=y + 1, x - 1..=x + 1])
            .iter()
            .filter(|elem| **elem == '.')
            .count()
    }

    fn remove(&mut self, idx: (usize, usize)) {
        self.tiles[idx] = '.';
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let tiles: Vec<char> = s
            .lines()
            .flat_map(|line| {
                width = line.len();
                line.chars()
            })
            .collect();
        let height = tiles.len() / width;
        let inner_shape = (height, width);
        let inner_tiles = Array2::from_shape_vec(inner_shape, tiles)?;
        let shape = (height + 2, width + 2);
        let mut tiles = Array2::from_elem(shape, '.');
        tiles
            .slice_mut(s![1..height + 1, 1..width + 1])
            .assign(&inner_tiles);
        Ok(Self { tiles })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Map = input.parse().expect("Invalid parse");
    let accessible = map.paperrolls().filter(|&idx| map.space(idx) > 4).count();
    Some(accessible)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map: Map = input.parse().expect("Invalid parse");
    let mut paperrolls: Vec<_> = map.paperrolls().collect();
    let init_len = paperrolls.len();

    loop {
        let mut none_removed = true;
        paperrolls.retain(|&idx| {
            let remove = map.space(idx) > 4;
            if remove {
                none_removed = false;
                map.remove(idx);
            }
            !remove
        });
        if none_removed {
            break;
        }
    }

    Some(init_len - paperrolls.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
