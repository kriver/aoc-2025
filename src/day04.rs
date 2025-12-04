use std::collections::HashMap;

use crate::util::{Coord2D, Grid};

type NeighbourCount = usize;
type Input = Grid<i32, NeighbourCount>;

impl Input {
    const NEIGHBOURS: [Coord2D<i32>; 8] = [
        Coord2D::new(-1, -1),
        Coord2D::new(0, -1),
        Coord2D::new(1, -1),
        Coord2D::new(-1, 0),
        Coord2D::new(1, 0),
        Coord2D::new(-1, 1),
        Coord2D::new(0, 1),
        Coord2D::new(1, 1),
    ];

    fn count_neighbours(&self, coord: &Coord2D<i32>) -> usize {
        Self::NEIGHBOURS
            .iter()
            .filter_map(|delta| {
                let neighbour_coord = *coord + *delta;
                self.squares.get(&neighbour_coord)
            })
            .count()
    }

    pub fn init_neighbour_count(&mut self) {
        let m = self
            .squares
            .iter()
            .map(|(coord, _count)| (*coord, self.count_neighbours(coord)))
            .collect();
        self.squares = m;
    }

    pub fn remove_paper(&mut self) {
        loop {
            let removed: HashMap<Coord2D<i32>, NeighbourCount> = self
                .squares
                .extract_if(|_coord, count| *count < 4)
                .collect();
            if removed.is_empty() {
                break;
            }
            for coord in removed.keys() {
                for delta in Self::NEIGHBOURS.iter() {
                    let neighbour_coord = *coord + *delta;
                    if let Some(neighbour_count) = self.squares.get_mut(&neighbour_coord) {
                        *neighbour_count -= 1;
                    }
                }
            }
        }
    }
}

pub fn input() -> Input {
    Grid::from_file("data/day04.txt", |c, _coord| match c {
        '@' => Some(0),
        _ => None,
    })
}

pub fn part1(mut grid: Input) -> usize {
    grid.init_neighbour_count();
    grid.squares.values().filter(|&count| *count < 4).count()
}

pub fn part2(mut grid: Input) -> usize {
    grid.init_neighbour_count();
    let before = grid.squares.len();
    grid.remove_paper();
    let after = grid.squares.len();
    before - after
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1551);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 9784);
    }
}
