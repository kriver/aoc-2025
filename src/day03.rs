use crate::util::load;

type Bank = Vec<i8>;
type Input = Vec<Bank>;

pub fn input() -> Input {
    let values: Vec<String> = load("data/day03.txt");
    values
        .into_iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
        .collect()
}

fn to_joltage(bank: Bank, n: usize) -> u64 {
    let mut digits = bank[bank.len() - n..].to_vec();
    let sz = bank.len() - n;
    for mut digit in bank.into_iter().take(sz).rev() {
        for i in 0..digits.len() {
            if digit >= digits[i] {
                let prev = digits[i];
                digits[i] = digit;
                digit = prev;
            } else {
                break;
            }
        }
    }
    (digits
        .into_iter()
        .map(|d| d as u64)
        .reduce(|a, b| a * 10 + b))
    .unwrap()
}

pub fn part1(banks: Input) -> u64 {
    banks.into_iter().map(|bank| to_joltage(bank, 2)).sum()
}

pub fn part2(banks: Input) -> u64 {
    banks.into_iter().map(|bank| to_joltage(bank, 12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 17144);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 170371185255900);
    }
}
