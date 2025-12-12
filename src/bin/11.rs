use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;

advent_of_code::solution!(11);

fn key(s: &str) -> u64 {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

struct Graph {
    edges: HashMap<u64, Vec<u64>>,
}

impl Graph {
    fn paths(&self, start: u64, goal: u64) -> usize {
        fn inner(
            node: u64,
            goal: u64,
            edges: &HashMap<u64, Vec<u64>>,
            paths: &mut HashMap<u64, usize>,
        ) -> usize {
            if let Some(value) = paths.get(&node) {
                return *value;
            }

            let value = if node == goal {
                1
            } else if let Some(children) = edges.get(&node) {
                children
                    .iter()
                    .map(|child| inner(*child, goal, edges, paths))
                    .sum()
            } else {
                0
            };
            paths.insert(node, value);
            value
        }

        let mut paths = HashMap::new();
        inner(start, goal, &self.edges, &mut paths)
    }
}

impl FromStr for Graph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut edges = HashMap::new();

        for l in s.lines() {
            let (from, tos) = l.split_once(": ").unwrap();
            let entry: &mut Vec<_> = edges.entry(key(from)).or_default();
            for to in tos.split(" ") {
                entry.push(key(to));
            }
        }

        Ok(Self { edges })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph: Graph = input.parse().expect("Invalid parse");
    Some(graph.paths(key("you"), key("out")))
}

pub fn part_two(input: &str) -> Option<usize> {
    let graph: Graph = input.parse().expect("Invalid parse");
    Some(
        graph.paths(key("svr"), key("fft"))
            * graph.paths(key("fft"), key("dac"))
            * graph.paths(key("dac"), key("out")),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file_part;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
