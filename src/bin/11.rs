use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let result = Reactor::new(parse_input(input)).get_number_of_paths(
        "you",
        "out",
        vec![]
    );
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = Reactor::new(parse_input(input)).get_number_of_paths(
        "svr",
        "out",
        vec!["fft".to_string(), "dac".to_string()],
    );
    Some(result)
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|device_str| parse_device(device_str))
        .collect()
}

fn parse_device(input: &str) -> (String, Vec<String>) {
    let (input, outputs) = input.split_once(": ").unwrap();
    let outputs = outputs
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    (input.to_string(), outputs)
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct ReactorQuery {
    device_from: String,
    device_to: String,
    devices_need_to_visit: Vec<String>,
}

struct Reactor {
    device_outputs: HashMap<String, Vec<String>>,
    cache: HashMap<ReactorQuery, u64>,
}

impl Reactor {
    fn new(devices: HashMap<String, Vec<String>>) -> Self {
        Self {
            device_outputs: devices,
            cache: HashMap::new(),
        }
    }

    fn get_number_of_paths(&mut self, from: &str, to: &str, must_visit: Vec<String>) -> u64 {
        let input = ReactorQuery {
            device_from: from.to_string(),
            device_to: to.to_string(),
            devices_need_to_visit: must_visit.clone(),
        };
        if let Some(cached_result) = self.cache.get(&input) {
            return *cached_result;
        }

        let device_outputs = &self.device_outputs.get(from).unwrap().clone();

        let mut paths = 0;
        for device_output in device_outputs {
            if device_output == to {
                if must_visit.is_empty() {
                    paths += 1;
                }
            } else {
                let must_visit = must_visit
                    .iter()
                    .cloned()
                    .filter(|it| it != device_output)
                    .collect::<Vec<_>>();
                paths += self.get_number_of_paths(&device_output, to, must_visit);
            }
        }

        self.cache.insert(input.clone(), paths);
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
