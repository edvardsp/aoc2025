use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use ndarray::Array2;

advent_of_code::solution!(7);

type Idx = (usize, usize);

struct Map {
    tiles: Array2<char>,
}

impl Map {
    fn start(&self) -> Idx {
        self.tiles
            .indexed_iter()
            .flat_map(|(idx, v)| if *v == 'S' { Some(idx) } else { None })
            .next()
            .unwrap()
    }

    fn root(&self) -> Idx {
        self.next_splitter(self.start()).unwrap()
    }

    fn next_splitter(&self, beam: Idx) -> Option<Idx> {
        let (y, x) = beam;
        let y_max = self.tiles.dim().0;
        (y..y_max)
            .map(|yn| (yn, x))
            .find(|&idx| self.tiles[idx] == '^')
    }

    fn graph(&self) -> Graph {
        let root = self.root();

        let mut nodes: HashMap<_, HashSet<_>> = HashMap::new();
        let mut splitters = VecDeque::from([root]);
        let mut visited = HashSet::new();
        while let Some(splitter) = splitters.pop_front() {
            if !visited.insert(splitter) {
                continue;
            }

            let entry = nodes.entry(splitter).or_default();
            let (y, x) = splitter;
            let left_beam = (y, x - 1);
            let right_beam = (y, x + 1);

            if let Some(left_splitter) = self.next_splitter(left_beam) {
                entry.insert(left_splitter);
                splitters.push_back(left_splitter);
            }
            if let Some(right_splitter) = self.next_splitter(right_beam) {
                entry.insert(right_splitter);
                splitters.push_back(right_splitter);
            }
        }

        Graph { root, nodes }
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let chars: Vec<_> = lines.iter().flat_map(|l| l.chars()).collect();
        let height = lines.len();
        let width = chars.len() / height;
        let shape = (height, width);
        let tiles = Array2::from_shape_vec(shape, chars)?;
        Ok(Self { tiles })
    }
}

struct Graph {
    root: Idx,
    nodes: HashMap<Idx, HashSet<Idx>>,
}

impl Graph {
    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn children(&self, node: &Idx) -> &HashSet<Idx> {
        &self.nodes[node]
    }

    fn timelines(&self) -> usize {
        fn inner(graph: &Graph, node: Idx, memo: &mut HashMap<Idx, usize>) -> usize {
            if let Some(value) = memo.get(&node) {
                return *value;
            }

            let children = graph.children(&node);
            let subset: usize = children
                .iter()
                .copied()
                .map(|child| inner(graph, child, memo))
                .sum();

            let value = match children.len() {
                0 => 2,
                1 => 1 + subset,
                _ => subset,
            };

            memo.insert(node, value);
            value
        }

        let mut memo = HashMap::new();
        inner(self, self.root, &mut memo)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Map = input.parse().expect("Invalid parse");
    let graph = map.graph();
    Some(graph.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Map = input.parse().expect("Invalid parse");
    let graph = map.graph();
    Some(graph.timelines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
