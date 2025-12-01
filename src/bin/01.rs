advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut value = 50;
    let mut password = 0;

    for clicks in input.lines().map(|l| to_instruction(l)).map(|i| i % 100) {
        value += clicks;

        if value < 0 {
            value += 100;
        } else if value > 99 {
            value -= 100;
        }

        if value == 0 {
            password += 1;
        }
    }

    Some(password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut value = 50;
    let mut password: u64 = 0;

    for clicks in input.lines().map(|l| to_instruction(l)) {
        password += (clicks.abs() / 100) as u64;
        let remaining_clicks = clicks % 100;

        let old_value = value;
        value += remaining_clicks;

        if value < 0 {
            value += 100;
        } else if value > 99 {
            value -= 100;
        }

        if clicks > 0 {
            // turned right
            if old_value > value {
                password += 1;
            }
        } else if clicks < 0 {
            // turned left
            if value == 0 || (old_value < value && old_value != 0) {
                password += 1;
            }
        }
    }

    Some(password)
}

fn to_instruction(instruction_str: &str) -> i32 {
    if let Some((first, rest)) = instruction_str
        .chars()
        .next()
        .map(|c| (c, &instruction_str[c.len_utf8()..]))
    {
        let mut clicks = rest.parse::<i32>().unwrap_or(0);
        if first == 'L' {
            clicks *= -1;
        }

        clicks
    } else {
        0
    }
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
        assert_eq!(result, Some(6));
    }
}
