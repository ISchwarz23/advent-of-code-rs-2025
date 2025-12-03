advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let result = input.lines().map(|line| get_highest_joltage(line, 2)).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input.lines().map(|line| get_highest_joltage(line, 12)).sum();
    Some(result)
}

fn get_highest_joltage(bank_str: &str, joltage_length: usize) -> u64 {
    let mut start_index = 0;
    let mut joltate_str = "".to_string();
    for i in 0..joltage_length {
        let (highest_char, index) = get_highest_digit(bank_str, start_index, joltage_length - i);
        start_index = index + 1;
        joltate_str += &highest_char.to_string();
    }
    joltate_str.parse().unwrap()
}

fn get_highest_digit(bank_str: &str, start_index: usize, until: usize) -> (char, usize) {
    let bank_chars: Vec<char> = bank_str
        .chars()
        .skip(start_index)
        .take(bank_str.len() - start_index - until + 1)
        .collect();

    let mut highest_digit_index: usize = 0;
    for index in 0..bank_chars.len() {
        if bank_chars[highest_digit_index] < bank_chars[index] {
            highest_digit_index = index;
        }
    }

    (bank_chars[highest_digit_index], start_index + highest_digit_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
