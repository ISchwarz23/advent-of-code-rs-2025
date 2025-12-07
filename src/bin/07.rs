use advent_of_code::vector::Vector2d;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let splitters = extract_splitters(input);
    let start_beam_index = extract_beam_start(input);

    let cm = ClassicalManifold::new();
    Some(cm.get_number_of_splits(splitters, start_beam_index))
}

pub fn part_two(input: &str) -> Option<u64> {
    let splitters = extract_splitters(input);
    let beam_index = extract_beam_start(input);

    let mut qsm = QuantumTachyonManifold::new();
    Some(qsm.get_number_of_timelines(&splitters, beam_index, 1))
}

struct ClassicalManifold {}

impl ClassicalManifold {
    fn new() -> ClassicalManifold {
        Self {}
    }

    fn get_number_of_splits(&self, splitters: Vec<Vec<usize>>, start_beam_index: usize) -> u64 {
        let mut beams: HashSet<usize> = HashSet::new();
        beams.insert(start_beam_index);

        let mut row_index = 1usize;
        let mut number_of_splits = 0u64;
        while row_index < splitters.len() {
            let mut new_beams: HashSet<usize> = HashSet::new();

            for beam_index in beams.iter() {
                if splitters[row_index].contains(beam_index) {
                    number_of_splits += 1;
                    new_beams.insert(beam_index - 1);
                    new_beams.insert(beam_index + 1);
                } else {
                    new_beams.insert(*beam_index);
                }
            }

            beams = new_beams;
            row_index += 1;
        }
        number_of_splits
    }
}

struct QuantumTachyonManifold {
    timeline_cache: HashMap<Vector2d, u64>,
}

impl QuantumTachyonManifold {
    fn new() -> QuantumTachyonManifold {
        Self {
            timeline_cache: HashMap::new(),
        }
    }

    fn get_number_of_timelines(
        &mut self,
        splitters: &Vec<Vec<usize>>,
        beam_index: usize,
        row_index: usize,
    ) -> u64 {
        let beam_location = Vector2d {
            x: beam_index as i64,
            y: row_index as i64,
        };
        if let Some(timelines) = self.timeline_cache.get(&beam_location) {
            return *timelines;
        }

        let mut row_index = row_index;
        while row_index < splitters.len() {
            if splitters[row_index].contains(&beam_index) {
                let left_timelines =
                    self.get_number_of_timelines(splitters, beam_index - 1, row_index + 1);
                let right_timelines =
                    self.get_number_of_timelines(splitters, beam_index + 1, row_index + 1);
                let timelines = left_timelines + right_timelines;
                self.timeline_cache.insert(beam_location, timelines);
                return timelines;
            } else {
                row_index += 1;
            }
        }
        1
    }
}

fn extract_beam_start(input: &str) -> usize {
    input.lines().nth(0).unwrap().find('S').unwrap()
}

fn extract_splitters(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| if c == '^' { Some(x) } else { None })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
