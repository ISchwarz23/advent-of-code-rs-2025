use advent_of_code::range::range_inclusive_contains;
use advent_of_code::rect::Rectangle;
use advent_of_code::vector::Vector2d;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::RangeInclusive;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let corners: Vec<Vector2d> = input.lines().map(|line| Vector2d::from_str(line)).collect();

    let mut biggest_area = 0u64;
    for i in 0..corners.len() - 1 {
        for j in (i + 1)..corners.len() {
            let corner_i = &corners[i];
            let corner_j = &corners[j];

            let width = max(corner_i.x, corner_j.x) - min(corner_i.x, corner_j.x) + 1;
            let height = max(corner_i.y, corner_j.y) - min(corner_i.y, corner_j.y) + 1;
            let area = (width * height) as u64;
            if area > biggest_area {
                biggest_area = area;
            }
        }
    }

    Some(biggest_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = Map::new(input);

    /* for y in 0..9 {
        let green_tiles = map.get_green_tiles_in_line(&y);
        for x in 0..14 {
            if map.red_tiles.contains(&Vector2d { x, y }) {
                print!("#")
            } else if green_tiles.iter().any(|tile_range| tile_range.contains(&x)) {
                print!("X")
            } else {
                print!(".")
            }
        }
        println!();
    } */

    // find areas: big to small
    let mut areas: Vec<Rectangle> = Vec::new();
    for i in 0..map.red_tiles.len() - 1 {
        for j in (i + 1)..map.red_tiles.len() {
            let corner_i = &map.red_tiles[i];
            let corner_j = &map.red_tiles[j];

            areas.push(Rectangle {
                x_from: min(corner_i.x, corner_j.x),
                x_to: max(corner_i.x, corner_j.x),
                y_from: min(corner_i.y, corner_j.y),
                y_to: max(corner_i.y, corner_j.y),
            });
        }
    }
    areas.sort_by(|a, b| b.area().cmp(&a.area()));

    for area in areas {
        let is_inside_green_tiles = area.y_range().into_iter().all(|y| {
            map.get_green_tiles_in_line(&y)
                .iter()
                .any(|green_tile_range| range_inclusive_contains(green_tile_range, &area.x_range()))
        });

        if is_inside_green_tiles {
            return Some(area.area());
        }
    }

    None
}

struct Map {
    pub red_tiles: Vec<Vector2d>,
    horizontal_edges: Vec<(i64, RangeInclusive<i64>)>,
    vertical_edges: Vec<(i64, RangeInclusive<i64>)>,
    cache: HashMap<i64, Vec<RangeInclusive<i64>>>,
}

impl Map {
    fn new(input: &str) -> Map {
        let mut red_tiles: Vec<Vector2d> =
            input.lines().map(|line| Vector2d::from_str(line)).collect();
        red_tiles.push(red_tiles.first().unwrap().clone());

        let horizontal_edges: Vec<(i64, RangeInclusive<i64>)> = red_tiles
            .windows(2)
            .filter(|window| window[0].y == window[1].y)
            .map(|window| {
                (
                    window[0].y,
                    min(window[0].x, window[1].x)..=max(window[0].x, window[1].x),
                )
            })
            .collect::<Vec<_>>();

        let vertical_edges: Vec<(i64, RangeInclusive<i64>)> = red_tiles
            .windows(2)
            .filter(|window| window[0].x == window[1].x)
            .map(|window| {
                (
                    window[0].x,
                    min(window[0].y, window[1].y)..=max(window[0].y, window[1].y),
                )
            })
            .collect::<Vec<_>>();

        Self {
            red_tiles,
            horizontal_edges,
            vertical_edges,
            cache: HashMap::new(),
        }
    }

    fn get_green_tiles_in_line(&mut self, y: &i64) -> Vec<RangeInclusive<i64>> {
        if self.cache.contains_key(y) {
            return self.cache[y].clone();
        }

        let mut green_tiles_vertical_y: Vec<i64> = self
            .vertical_edges
            .iter()
            .filter(|(_, y_range)| y_range.contains(&y))
            .map(|(x, _)| *x)
            .collect();
        green_tiles_vertical_y.sort();

        let horizontal_edges_y: Vec<&RangeInclusive<i64>> = self
            .horizontal_edges
            .iter()
            .filter(|(yy, _)| yy == y)
            .map(|(_, range)| range)
            .collect();

        let mut green_tile_ranges_horizontal_y: Vec<RangeInclusive<i64>> = green_tiles_vertical_y
            .iter()
            .filter(|x| {
                horizontal_edges_y
                    .iter()
                    .all(|edge| &edge.start() != x && &edge.end() != x)
            })
            .map(|x| *x..=*x)
            .collect();

        horizontal_edges_y
            .into_iter()
            .for_each(|range| green_tile_ranges_horizontal_y.push(range.clone()));
        green_tile_ranges_horizontal_y
            .sort_by(|range1, range2| range1.start().cmp(&range2.start()));

        let green_tiles_horizontal_y: Vec<RangeInclusive<i64>> = green_tile_ranges_horizontal_y
            .chunks(2)
            .map(|chunk| {
                if chunk.len() == 2 {
                    *(chunk[0].start())..=*(chunk[1].end())
                } else {
                    chunk[0].clone()
                }
            })
            .fold(vec![], |mut acc, range| {
                if acc.is_empty() {
                    acc.push(range);
                } else if &(acc.last().unwrap().end() + 1) == range.start() {
                    // merge ranges if they are adjacent
                    let last_range = acc.pop().unwrap();
                    acc.push(*last_range.start()..=*range.end());
                } else {
                    acc.push(range);
                }
                acc
            });

        // println!("{:?}", green_tiles_horizontal_y);

        self.cache.insert(*y, green_tiles_horizontal_y.clone());

        green_tiles_horizontal_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
