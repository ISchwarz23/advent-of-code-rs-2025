use std::ops::RangeInclusive;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let fresh_id_ranges = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| to_range(line))
        .collect::<Vec<_>>();

    let no_of_fresh_ids = input
        .lines()
        .skip(fresh_id_ranges.len() + 1)
        .map(|line| u64::from_str_radix(line, 10).unwrap())
        .filter(|id| fresh_id_ranges.iter().any(|range| range.contains(id)))
        .count() as u64;

    Some(no_of_fresh_ids)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sorted_fresh_id_ranges: Vec<RangeInclusive<u64>> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| to_range(line))
        .collect::<Vec<_>>();

    sorted_fresh_id_ranges.sort_by(|first, second| first.start().cmp(second.start()));

    let mut unique_ranges: Vec<RangeInclusive<u64>> = vec![];
    unique_ranges.push(sorted_fresh_id_ranges.first().unwrap().clone());
    for next_range in sorted_fresh_id_ranges {
        let previous_range = unique_ranges.last().unwrap();

        if next_range.end() <= previous_range.end() {
            // range already included
            continue;
        }

        if next_range.start() <= previous_range.end() {
            // start is already included, need to adapt range
            unique_ranges.push(RangeInclusive::new(
                previous_range.end() + 1,
                next_range.end().clone(),
            ));
        } else {
            // unique range
            unique_ranges.push(next_range);
        }
    }

    let no_of_fresh_ids = unique_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<u64>();

    Some(no_of_fresh_ids)
}

fn to_range(range_str: &str) -> RangeInclusive<u64> {
    let parts: Vec<&str> = range_str.split('-').collect();
    let first: u64 = parts[0].parse().unwrap();
    let last: u64 = parts[1].parse().unwrap();
    first..=last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
