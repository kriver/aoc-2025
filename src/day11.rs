use std::collections::{HashMap, HashSet};

use crate::util::load;

type Devices = HashMap<String, Vec<String>>;
type Steps = HashMap<String, (usize, HashSet<String>)>;

pub fn input() -> Devices {
    let lines: Vec<String> = load("data/day11.txt");
    lines
        .into_iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let name = parts.next().unwrap()[0..3].to_string();
            let connections = parts.map(|p| p.to_string()).collect::<Vec<String>>();
            (name, connections)
        })
        .collect()
}

pub fn part1(devices: Devices) -> usize {
    let start = "you".to_owned();
    let mut visited: Steps = HashMap::from([(start.clone(), (1, HashSet::new()))]);
    let mut todo: Vec<_> = vec![start];
    loop {
        if todo.is_empty() {
            break;
        }
        let current = todo.pop().unwrap();
        let nexts = devices.get(&current);
        for next in nexts.unwrap_or(&vec![]) {
            let entry = visited.entry(next.clone()).or_insert((0, HashSet::new()));
            entry.0 += 1;
            entry.1.insert(current.clone());
            todo.push(next.clone());
        }
    }
    visited["out"].0
}

pub fn part2(_devices: Devices) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 652);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}
