use crate::util::load;

#[derive(Debug)]
pub enum Op {
    Add,
    Mul,
}

pub struct Part1 {
    numbers: Vec<Vec<u64>>,
    ops: Vec<Op>,
}

pub fn input1() -> Part1 {
    let lines: Vec<String> = load("data/day06.txt");
    let numbers = lines
        .iter()
        .take(4)
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let ops = lines
        .iter()
        .skip(4)
        .map(|line| {
            line.split_whitespace()
                .map(|s| match s {
                    "+" => Op::Add,
                    "*" => Op::Mul,
                    _ => panic!("Unknown operation"),
                })
                .collect::<Vec<Op>>()
        })
        .next()
        .unwrap();
    Part1 { numbers, ops }
}

pub fn part1(problems: Part1) -> u64 {
    let mut total = 0;
    let sz = problems.numbers[0].len();
    for i in 0..sz {
        let op = &problems.ops[i];
        let mut res = if let Op::Add = op { 0 } else { 1 };
        for j in 0..problems.numbers.len() {
            match op {
                Op::Add => res += problems.numbers[j][i],
                Op::Mul => res *= problems.numbers[j][i],
            }
        }
        total += res;
    }
    total
}

type Part2 = Vec<Vec<char>>;

pub fn input2() -> Part2 {
    let lines: Vec<String> = load("data/day06.txt");
    lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part2(lines: Part2) -> u64 {
    let mut total = 0;
    let mut numbers = vec![];
    for i in (0..lines[0].len()).rev() {
        let chars = format!(
            "{}{}{}{}",
            lines[0][i], lines[1][i], lines[2][i], lines[3][i]
        );
        let s = chars.trim();
        if !s.is_empty() {
            let n = s.parse::<u64>().unwrap();
            numbers.push(n);
        }
        match lines[4][i] {
            ' ' => (),
            '+' => {
                total += numbers.iter().sum::<u64>();
                numbers.clear();
            }
            '*' => {
                let prod = numbers.iter().product::<u64>();
                total += prod;
                numbers.clear();
            }
            _ => panic!("Unknown operation"),
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input1()), 8108520669952);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input2()), 11708563470209);
    }
}
