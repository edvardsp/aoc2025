use itertools::Itertools;
use std::str::FromStr;
use z3::ast::Int;

advent_of_code::solution!(10);

struct Machine {
    indicators: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    fn fewest_presses_lights(&self) -> usize {
        self.buttons
            .iter()
            .powerset()
            .find(|set| {
                let mut lights = vec![false; self.indicators.len()];
                set.iter()
                    .flat_map(|buttons| *buttons)
                    .copied()
                    .for_each(|b| lights[b] ^= true);
                lights == self.indicators
            })
            .map(|set| set.len())
            .expect("No combinations found")
    }

    fn fewest_presses_joltage(&self) -> usize {
        let opt = z3::Optimize::new();

        // Amount of times each button is pressed
        let presses: Vec<_> = (0..self.buttons.len())
            .map(|idx| Int::fresh_const(&format!("button_{idx}")))
            .collect();

        // All buttons are pressed 0 or more times
        presses.iter().for_each(|button| opt.assert(&button.ge(0)));

        for (pos, &target) in self.joltage.iter().enumerate() {
            // A given joltage target
            let target = Int::from_u64(target as u64);

            // Find all buttons which increment the given joltage
            let mut terms = Vec::new();
            for (idx, button) in self.buttons.iter().enumerate() {
                if button.contains(&pos) {
                    terms.push(&presses[idx]);
                }
            }

            // The sum of all button presses which contribute to this joltage has to equal the target joltage
            let sum = Int::add(&terms);
            opt.assert(&sum.eq(target));
        }

        // The total amount of presses which has to be minimized
        let total = Int::fresh_const("total");
        opt.assert(&total.eq(Int::add(&presses)));
        opt.minimize(&total);

        let z3::SatResult::Sat = opt.check(&[]) else {
            panic!("No solution found");
        };

        let model = opt.get_model().unwrap();
        let result = model.eval(&total, true).unwrap();
        result.as_u64().unwrap() as usize
    }
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut indicators = Vec::new();
        let mut buttons = Vec::new();
        let mut joltage = Vec::new();
        for item in s.split(" ") {
            if let Some(item) = item.strip_prefix("[") {
                let item = item.strip_suffix("]").unwrap();
                indicators = item.chars().map(|c| c == '#').collect();
            } else if let Some(item) = item.strip_prefix("(") {
                let item = item.strip_suffix(")").unwrap();
                let button = item
                    .split(",")
                    .map(|n| n.parse())
                    .collect::<Result<_, _>>()?;
                buttons.push(button);
            } else if let Some(item) = item.strip_prefix("{") {
                let item = item.strip_suffix("}").unwrap();
                joltage = item
                    .split(",")
                    .map(|n| n.parse())
                    .collect::<Result<_, _>>()?;
            }
        }
        Ok(Self {
            indicators,
            buttons,
            joltage,
        })
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Machine>> {
    input.lines().map(|l| l.parse()).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let machines = parse_input(input).expect("Invalid parse");
    let value = machines.iter().map(Machine::fewest_presses_lights).sum();
    Some(value)
}

pub fn part_two(input: &str) -> Option<usize> {
    let machines = parse_input(input).expect("Invalid parse");
    let value = machines.iter().map(Machine::fewest_presses_joltage).sum();
    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
