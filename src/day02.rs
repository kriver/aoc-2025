use crate::util::load;

type Range = (String, String);
type Input = Vec<Range>;

pub fn input() -> Input {
    let data: Vec<String> = load("data/day02.txt");
    let line = &data[0];
    line.split(',')
        .map(|r| {
            let bounds: Vec<&str> = r.split('-').collect();
            (bounds[0].to_string(), bounds[1].to_string())
        })
        .collect()
}

type IsInvalid = fn(&str) -> bool;

fn is_invalid(n: &str, only_twice: bool) -> bool {
    for i in 2..=n.len() {
        if only_twice && i > 2 {
            break;
        }
        if n.len() % i != 0 {
            continue;
        }
        let sz = n.len() / i;
        let parts: Vec<&str> = (0..i).map(|j| &n[j * sz..(j + 1) * sz]).collect();
        if parts.windows(2).all(|w| w[0] == w[1]) {
            return true;
        }
    }
    false
}
fn sum_invalids(range: Range, is_invalid: IsInvalid) -> u64 {
    let start: u64 = range.0.parse().unwrap();
    let end: u64 = range.1.parse().unwrap();
    let mut sum: u64 = 0;
    for n in start..=end {
        let s = n.to_string();
        if is_invalid(&s) {
            sum += n as u64;
        }
    }
    sum
}

pub fn part1(ranges: Input) -> u64 {
    ranges
        .into_iter()
        .map(|r| sum_invalids(r, |s| is_invalid(s, true)))
        .sum::<u64>()
}

pub fn part2(ranges: Input) -> u64 {
    ranges
        .into_iter()
        .map(|r| sum_invalids(r, |s| is_invalid(s, false)))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 28846518423);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 31578210022);
    }
}
