use std::collections::{BTreeMap, HashSet};

use crate::util::{Coord3D, load};

type Coord = Coord3D<u64>;
type Boxes = Vec<Coord>;
type Distances = BTreeMap<u64, (Coord, Coord)>;
type Circuits = Vec<HashSet<Coord>>;

pub fn input() -> Boxes {
    let lines: Vec<String> = load("data/day08.txt");
    lines
        .into_iter()
        .map(|line| {
            let coords: Vec<u64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Coord::new(coords[0], coords[1], coords[2])
        })
        .collect()
}

fn distance_sq(a: &Coord, b: &Coord) -> u64 {
    let dx = a.x.abs_diff(b.x);
    let dy = a.y.abs_diff(b.y);
    let dz = a.z.abs_diff(b.z);
    dx * dx + dy * dy + dz * dz
}

fn calc_distances(boxes: &Boxes) -> Distances {
    let mut distances: Distances = BTreeMap::new();
    let sz = boxes.len();
    for i in 0..sz {
        for j in (i + 1)..sz {
            let a = &boxes[i];
            let b = &boxes[j];
            let dist = distance_sq(a, b);
            distances.insert(dist, (*a, *b));
        }
    }
    distances
}

fn connect(circuits: &mut Circuits, c1: Coord, c2: Coord) {
    let matches: Vec<_> = circuits
        .extract_if(.., |c| c.contains(&c1) || c.contains(&c2))
        .collect();
    if matches.is_empty() {
        // none of the boxes found
        circuits.push(vec![c1, c2].into_iter().collect());
    } else if matches.len() == 2 {
        // both boxes found in different circuits, merge them
        circuits.push(matches[0].union(&matches[1]).cloned().collect());
    } else {
        // one circuit found
        let mut c = matches.into_iter().next().unwrap();
        if c.contains(&c1) && c.contains(&c2) {
            // both boxes already in same circuit, do nothing
            circuits.push(c);
        } else {
            // one box found
            c.insert(c1);
            c.insert(c2);
            circuits.push(c);
        }
    }
}

pub fn part1(boxes: Boxes) -> usize {
    let mut distances: Distances = calc_distances(&boxes);
    let mut circuits: Circuits = Vec::new();
    for _ in 0..1000 {
        if distances.is_empty() {
            break;
        }
        let (_dist, (c1, c2)) = distances.pop_first().unwrap();
        connect(&mut circuits, c1, c2);
    }
    let mut sizes: Vec<usize> = circuits.into_iter().map(|c| c.len()).collect();
    sizes.sort();
    sizes.into_iter().rev().take(3).product()
}

pub fn part2(boxes: Boxes) -> u64 {
    let mut distances: Distances = calc_distances(&boxes);
    let mut circuits: Circuits = Vec::new();
    let (c1, c2) = loop {
        if distances.is_empty() {
            panic!("ran out of distances before all boxes connected");
        }
        let (_dist, (c1, c2)) = distances.pop_first().unwrap();
        connect(&mut circuits, c1, c2);
        if circuits.len() == 1 && circuits[0].len() == boxes.len() {
            break (c1, c2);
        }
    };
    c1.x * c2.x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 46398);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 8141888143);
    }
}
