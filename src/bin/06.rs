use ndarray::Array2;

advent_of_code::solution!(6);

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Mul,
}

impl From<u8> for Op {
    fn from(value: u8) -> Self {
        match value {
            b'+' => Op::Add,
            b'*' => Op::Mul,
            _ => panic!("Invalid value"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<_> = input.lines().collect();
    let (&ops_str, nums_str) = lines.split_last()?;

    let ops: Vec<Op> = ops_str
        .as_bytes()
        .iter()
        .copied()
        .filter(|b| !b.is_ascii_whitespace())
        .map(|o| o.into())
        .collect();
    let nums_vec: Vec<u64> = nums_str
        .iter()
        .flat_map(|l| l.split_whitespace())
        .map(|n| n.parse().unwrap())
        .collect();
    let height = nums_str.len();
    let width = nums_vec.len() / height;
    let shape = (height, width);
    let nums = Array2::from_shape_vec(shape, nums_vec).unwrap();

    let grand_total = ops
        .iter()
        .zip(nums.columns())
        .map(|(op, col)| match op {
            Op::Add => col.sum(),
            Op::Mul => col.product(),
        })
        .sum();

    Some(grand_total)
}

fn col_to_num(idx: usize, bytes: &[&[u8]]) -> Option<u64> {
    if idx >= bytes[0].len() {
        return None;
    }

    let num = bytes
        .iter()
        .map(|l| l[idx])
        .filter(|b| *b != b' ')
        .map(|b| (b - b'0') as u64)
        .fold(0, |acc, x| acc * 10 + x);

    if num == 0 { None } else { Some(num) }
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    let (&ops_str, nums_str) = lines.split_last()?;

    let mut idx = 0;
    let mut nums = Vec::new();
    let mut grand_total = 0;
    while idx < ops_str.len() {
        let op: Op = ops_str[idx].into();
        nums.clear();
        while let Some(num) = col_to_num(idx, nums_str) {
            nums.push(num);
            idx += 1;
        }
        idx += 1;
        grand_total += match op {
            Op::Add => nums.iter().sum::<u64>(),
            Op::Mul => nums.iter().product(),
        };
    }
    Some(grand_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
