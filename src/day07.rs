use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use crate::util::{Coord2D, Direction, load};

type Splitters = HashSet<Coord2D<i32>>;
type Timelines = HashMap<Coord2D<i32>, usize>;

pub struct Teleporter {
    size: Coord2D<i32>,
    start: Coord2D<i32>,
    splitters: Splitters,
}

pub fn input() -> Teleporter {
    let lines: Vec<String> = load("data/day07.txt");
    let size = Coord2D::new(lines[0].len() as i32, lines.len() as i32);
    let mut start = Coord2D::new(0, 0);
    let mut splitters = HashSet::new();
    lines.into_iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            'S' => {
                start = Coord2D::new(x as i32, y as i32);
            }
            '^' => {
                splitters.insert(Coord2D::new(x as i32, y as i32));
            }
            _ => {}
        })
    });
    Teleporter {
        size,
        start,
        splitters,
    }
}

pub fn part1(teleporter: Teleporter) -> usize {
    fn beam(t: &Teleporter, pos: Coord2D<i32>, hit: &mut Splitters) {
        let new_pos = pos.add(Direction::Down.into());
        if new_pos.y < t.size.y && !hit.contains(&new_pos) {
            if t.splitters.contains(&new_pos) {
                hit.insert(new_pos);
                beam(t, new_pos.add(Direction::Left.into()), hit);
                beam(t, new_pos.add(Direction::Right.into()), hit);
            } else {
                beam(t, new_pos, hit);
            }
        }
    }
    let mut hit = HashSet::new();
    beam(&teleporter, teleporter.start, &mut hit);
    hit.len()
}

pub fn part2(teleporter: Teleporter) -> usize {
    fn insert(tls: &mut Timelines, pos: Coord2D<i32>, count: usize) {
        tls.entry(pos).and_modify(|c| *c += count).or_insert(count);
    }
    fn beam(t: &Teleporter, mut timelines: Timelines) -> Timelines {
        loop {
            let mut new_tls: Timelines = HashMap::new();
            timelines.iter().for_each(|(pos, count)| {
                let new_pos = pos.add(Direction::Down.into());
                if new_pos.y < t.size.y {
                    if t.splitters.contains(&new_pos) {
                        insert(&mut new_tls, new_pos.add(Direction::Left.into()), *count);
                        insert(&mut new_tls, new_pos.add(Direction::Right.into()), *count);
                    } else {
                        insert(&mut new_tls, new_pos, * count);
                    }
                }
            });
            if new_tls.is_empty() {
                break timelines;
            }
            timelines = new_tls;
        }
    }
    let mut timelines: Timelines = HashMap::new();
    timelines.insert(teleporter.start, 1);
    let timelines = beam(&teleporter, timelines);
    timelines.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1570);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 15118009521693);
    }
}
