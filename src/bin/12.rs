advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let parts: Vec<Vec<&str>> = input
        .lines()
        .collect::<Vec<&str>>()
        .split(|l| l.is_empty())
        .map(|group| group.to_vec())
        .collect();

    let presents: Vec<Present> = parts
        .iter()
        .take(parts.len() - 1)
        .map(|group| parse_present(group))
        .collect();

    let regions: Vec<Region> = parts
        .last()
        .unwrap()
        .iter()
        .map(|l| parse_region(l))
        .collect();

    let result = regions
        .iter()
        .filter(|region| can_fit_presents(region, &presents))
        .count();

    Some(result as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn can_fit_presents(region: &&Region, presents: &Vec<Present>) -> bool {
    let max_required_space: u64 = region
        .required_presents
        .iter()
        .enumerate()
        .map(|(present_index, present_count)| {
            (presents[present_index].max_space_required * present_count) as u64
        })
        .sum();
    let min_required_space: u64 = region
        .required_presents
        .iter()
        .enumerate()
        .map(|(present_index, present_count)| {
            (presents[present_index].min_space_required * present_count) as u64
        })
        .sum();
    let available_space = (region.width * region.height) as u64;

    if available_space < min_required_space {
        false
    } else if available_space >= max_required_space {
        true
    } else {
        // TODO: try to fit presents. for now assume it does not fits
        false
    }
}

fn parse_present(lines: &Vec<&str>) -> Present {
    let shape: Vec<Vec<bool>> = lines
        .iter()
        .skip(1)
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();

    let height = shape.len();
    let width = shape[0].len();
    let min_space_required = shape.iter().flatten().filter(|&&v| v).count();
    let max_space_required = width * height;

    Present {
        shape,
        height,
        width,
        min_space_required,
        max_space_required,
    }
}

fn parse_region(l: &str) -> Region {
    let (dimensions_str, required_presents_str) = l.split_once(": ").unwrap();

    let (width_str, height_str) = dimensions_str.split_once('x').unwrap();
    let required_presents = required_presents_str
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect::<Vec<usize>>();

    Region {
        width: width_str.parse().unwrap(),
        height: height_str.parse().unwrap(),
        required_presents,
    }
}

struct Present {
    shape: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    min_space_required: usize,
    max_space_required: usize,
}

struct Region {
    width: usize,
    height: usize,
    required_presents: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
