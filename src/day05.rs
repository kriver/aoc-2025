use std::collections::BTreeSet;

use crate::util::load;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Range(pub u64, pub u64);

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.cmp(&other.0) {
            std::cmp::Ordering::Equal => self.1.cmp(&other.1),
            other => other,
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Data {
    fresh: BTreeSet<Range>,
    ingredients: BTreeSet<u64>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            fresh: BTreeSet::new(),
            ingredients: BTreeSet::new(),
        }
    }

    pub fn get_fresh(&self) -> Vec<u64> {
        let mut result = Vec::new();
        let mut it_range = self.fresh.iter();
        let mut current_range = it_range.next().unwrap();
        let mut it_ing = self.ingredients.iter();
        let mut current_ingredient = it_ing.next().unwrap();
        loop {
            if *current_ingredient <= current_range.1 {
                if *current_ingredient >= current_range.0 {
                    // it's fresh
                    result.push(*current_ingredient);
                }
                // next ingredient
                match it_ing.next() {
                    Some(ing) => {
                        current_ingredient = ing;
                        continue;
                    }
                    None => break,
                }
            } else {
                // next range
                match it_range.next() {
                    Some(range) => {
                        current_range = range;
                        continue;
                    }
                    None => break,
                }
            }
        }
        result
    }

    pub fn get_fresh_range_count(&self) -> u64 {
        let mut count = 0;
        let mut current_range = *self.fresh.first().unwrap();
        for range in self.fresh.iter().skip(1) {
            if range.0 > current_range.1 {
                // no overlap, add current
                count += current_range.1 - current_range.0 + 1;
                current_range = *range;
            } else if range.1 <= current_range.1 {
                // fully contained, skip
                continue;
            } else {
                // partial overlap, extend current
                current_range.1 = range.1;
            }
        }
        // add last range
        count += current_range.1 - current_range.0 + 1;
        count
    }
}

type Input = Data;

pub fn input() -> Input {
    let lines: Vec<String> = load("data/day05.txt");
    let mut reading_ranges = true;
    let mut ingredients = Data::new();
    for line in lines {
        if line.is_empty() {
            reading_ranges = false;
            continue;
        }
        if reading_ranges {
            let parts: Vec<&str> = line.split('-').collect();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            ingredients.fresh.insert(Range(start, end));
        } else {
            let ingredient: u64 = line.parse().unwrap();
            ingredients.ingredients.insert(ingredient);
        }
    }
    ingredients
}

pub fn part1(data: Input) -> usize {
    data.get_fresh().len()
}

pub fn part2(data: Input) -> u64 {
    data.get_fresh_range_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 511);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 350939902751909);
    }
}
