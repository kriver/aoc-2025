use crate::util::{Coord2D, load};

type Coord = Coord2D<u32>;
type Corners = Vec<Coord>;

pub fn input() -> Corners {
    let lines: Vec<String> = load("data/day09.txt");
    lines
        .into_iter()
        .map(|line| {
            let mut parts = line.split(',');
            let x: u32 = parts.next().unwrap().parse().unwrap();
            let y: u32 = parts.next().unwrap().parse().unwrap();
            Coord { x, y }
        })
        .collect()
}

pub fn part1(corners: Corners) -> u64 {
    let mut largest: u64 = 0;
    for i in 0..corners.len() {
        for j in (i + 1)..corners.len() {
            let a = &corners[i];
            let b = &corners[j];
            let area = (a.x.abs_diff(b.x) + 1) as u64 * (a.y.abs_diff(b.y) + 1) as u64;
            if area > largest {
                largest = area;
            }
        }
    }
    largest
}

pub fn part2(corners: Corners) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 4715966250);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}
