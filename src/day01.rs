use crate::util::load;

pub fn input() -> Vec<String> {
    let values: Vec<String> = load("data/day01.txt");
    values
}

pub fn part1(values: Vec<String>) -> usize {
    let mut cnt = 0;
    let mut pos = 50;
    values.iter().for_each(|line| {
        let num = line[1..].parse::<i32>().unwrap();
        match line.chars().next().unwrap() {
            'L' => pos = (pos - num) % 100,
            'R' => pos = (pos + num) % 100,
            _ => (),
        };
        if pos == 0 {
            cnt += 1;
        }
    });
    cnt
}

pub fn part2(values: Vec<String>) -> u32 {
    let mut cnt = 0;
    let mut pos = 50;
    values.iter().for_each(|line| {
        let mut num = line[1..].parse::<i32>().unwrap();
        cnt += (num / 100) as u32;
        num %= 100;
        match line.chars().next().unwrap() {
            'L' => {
                pos -= num;
                if pos == -num || pos == 0 {
                    // started or ended at zero
                    pos += 100;
                } else if pos < 0 {
                    // crossed zero
                    pos += 200;
                }
            }
            'R' => pos += num,
            _ => (),
        };
        cnt += (pos / 100) as u32;
        pos %= 100;
    });
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1182);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 6907);
    }
}
