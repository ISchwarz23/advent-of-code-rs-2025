use std::cmp::{max, min};
use advent_of_code::vector::Vector2d;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let map = Map::new(&input);

    let rolls_to_be_removed = get_coords_of_rolls_to_be_removed(&map);

    Some(rolls_to_be_removed.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = Map::new(&input);

    let mut no_of_rolls_to_be_removed: u64 = 0;

    loop {
        let rolls_to_be_removed = get_coords_of_rolls_to_be_removed(&map);
        no_of_rolls_to_be_removed += rolls_to_be_removed.len() as u64;
        rolls_to_be_removed.iter().for_each(|coords| map.remove_roll(coords));

        if rolls_to_be_removed.is_empty() {
            break;
        }
    }

    Some(no_of_rolls_to_be_removed)
}

fn get_coords_of_rolls_to_be_removed(map: &Map) -> Vec<Vector2d> {
    let mut no_to_be_removed: Vec<Vector2d> = vec![];

    for y in 0..map.height {
        for x in 0..map.width {
            if !map.contains_roll(x, y) {
                continue;
            }

            if map.get_number_of_neighboring_rolls(x, y) < 4 {
                no_to_be_removed.push(Vector2d {
                    x: x as i64,
                    y: y as i64
                });
            }
        }
    }
    no_to_be_removed
}

struct Map {
    data: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Map {
        let data: Vec<Vec<bool>> = input
            .lines()
            .map(|l| l.chars().map(|c| c == '@').collect())
            .collect();

        let width = data[0].len();
        let height = data.len();

        Self {
            data,
            width,
            height,
        }
    }

    fn get_number_of_neighboring_rolls(&self, x: usize, y: usize) -> usize {
        let mut neighbors = 0usize;
        for iy in max(0isize, y as isize - 1)..min(self.height as isize, y as isize + 2) {
            for ix in max(0isize, x as isize - 1)..min(self.width as isize, x as isize + 2) {
                if iy == y as isize && ix == x as isize {
                    continue;
                };

                if self.contains_roll(ix as usize, iy as usize) {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn contains_roll(&self, x: usize, y: usize) -> bool {
        self.data[y][x]
    }

    fn remove_roll(&mut self, coors: &Vector2d) {
        self.data[coors.y as usize][coors.x as usize] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
