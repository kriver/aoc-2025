use std::fmt::Debug;

use lpsolve::{ConstraintType, Problem, Verbosity};
use regex::Regex;

use crate::util::load;

pub struct Machine {
    lights: u32, // required state of lights
    buttons: Vec<u32>,
    joltage: Vec<f64>, // required joltage levels
}

type Machines = Vec<Machine>;

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Machine {{ lights: {:b}, buttons: [", self.lights)?;
        for (i, b) in self.buttons.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:b}", b)?;
        }
        write!(f, "], joltage: {:?} }}", self.joltage)
    }
}

impl From<String> for Machine {
    fn from(s: String) -> Self {
        let re = Regex::new(r"^[({]([0-9,]+)[)}]$").unwrap();
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let lights = parts[0]
            .chars()
            .rev()
            .filter(|c| *c == '#' || *c == '.')
            .fold(0, |acc, c| {
                acc << 1
                    | match c {
                        '#' => 1,
                        '.' => 0,
                        _ => panic!("Invalid light state"),
                    }
            });
        let wirings = parts[1..parts.len() - 1]
            .iter()
            .map(|p| {
                re.captures(p)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .fold(0, |acc, n| acc | (1 << n))
            })
            .collect::<Vec<_>>();
        let joltage = re
            .captures(parts[parts.len() - 1])
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split(',')
            .map(|n| n.parse::<f64>().unwrap())
            .collect::<_>();
        Machine {
            lights,
            buttons: wirings,
            joltage,
        }
    }
}

#[derive(Debug)]
struct MachineMultiSubSetIterator<'a> {
    machine: &'a Machine,
    elements: usize,
    indices: Vec<usize>,
}

impl<'a> MachineMultiSubSetIterator<'a> {
    fn new(machine: &'a Machine) -> Self {
        let elements = machine.buttons.len();
        MachineMultiSubSetIterator {
            machine,
            elements,
            indices: vec![0],
        }
    }

    fn update_indices(&mut self) {
        let mut i = (self.indices.len() - 1) as isize;
        loop {
            if i < 0 {
                // increase sub-set size
                self.indices = vec![0; self.indices.len() + 1];
                break;
            }
            let j = i as usize;
            if self.indices[j] == self.elements - 1 {
                i -= 1;
                continue;
            }
            self.indices[j] += 1;
            let current = self.indices[j];
            let sz = self.indices.len();
            for k in j + 1..sz {
                self.indices[k] = current;
            }
            break;
        }
    }
}

impl<'a> Iterator for MachineMultiSubSetIterator<'a> {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self
            .indices
            .iter()
            .map(|i| self.machine.buttons[*i])
            .collect::<Vec<u32>>();
        self.update_indices();
        Some(item)
    }
}

impl Machine {
    fn solve_1(&self) -> usize {
        let mut it = MachineMultiSubSetIterator::new(self);
        loop {
            let subset = it.next().unwrap();
            let combination = subset.iter().fold(0, |acc, b| acc ^ b);
            if combination == self.lights {
                return subset.len();
            }
        }
    }

    fn solve_2(&self) -> usize {
        let coefficients = (0..self.joltage.len())
            .map(|i| {
                self.buttons
                    .iter()
                    .map(|b| if (b & (1 << i)) != 0 { 1.0 } else { 0.0 })
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<_>>();

        let n_rows = coefficients.len();
        let n_cols = coefficients[0].len();

        let mut builder = Problem::builder()
            .verbosity(Verbosity::Critical)
            .rows(n_rows as i32)
            .cols(n_cols as i32)
            .minimize()
            .objective(&vec![1.0; n_cols]);
        builder = coefficients
            .iter()
            .enumerate()
            .fold(builder, |builder, (i, row)| {
                builder.constraint(&row.to_vec(), ConstraintType::Eq, self.joltage[i])
            });
        for i in 1..=n_cols {
            builder = builder.integer_variable(i as i32);
        }
        let solution = builder.solve().unwrap();
        solution.objective_value().round() as usize
    }
}

pub fn input() -> Machines {
    let lines: Vec<String> = load("data/day10.txt");
    lines.into_iter().map(|line| Machine::from(line)).collect()
}

pub fn part1(machines: Machines) -> usize {
    machines.into_iter().map(|m| m.solve_1()).sum()
}

pub fn part2(machines: Machines) -> usize {
    machines.into_iter().map(|m| m.solve_2()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 491);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 20617);
    }
}
