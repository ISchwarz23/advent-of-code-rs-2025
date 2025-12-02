use std::ops::RangeInclusive;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .split(',')
        .map(|range_str| to_range(range_str))
        .flat_map(|range| range.into_iter().filter(|i| is_invalid_id_part1(i)))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .split(',')
        .map(|range_str| to_range(range_str))
        .flat_map(|range| range.into_iter().filter(|i| is_invalid_id_part2(i)))
        .sum();
    Some(result)
}

fn is_invalid_id_part1(id: &u64) -> bool {
    split_into_n_equal_parts_and_compare(&id.to_string(), 2)
}

fn is_invalid_id_part2(id: &u64) -> bool {
    for len in 2..=id.to_string().len() {
        if split_into_n_equal_parts_and_compare(&id.to_string(), len) {
            return true;
        }
    }
    false
}

fn to_range(range_str: &str) -> RangeInclusive<u64> {
    let parts: Vec<&str> = range_str.split('-').collect();
    let first: u64 = parts[0].parse().unwrap();
    let last: u64 = parts[1].parse().unwrap();
    first..=last
}

fn split_into_n_equal_parts_and_compare(s: &String, n: usize) -> bool {
    if n == 0 {
        return false;
    }

    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    // length must be divisible by n
    if len % n != 0 {
        return false;
    }

    let part_len = len / n;

    // take the first part as reference
    let reference = &chars[0..part_len];

    // check all parts
    for i in 1..n {
        let start = i * part_len;
        let end = start + part_len;
        if &chars[start..end] != reference {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
