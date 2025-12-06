use regex::Regex;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let operators: Vec<char> = extract_operator_chars(input);
    let numbers: Vec<Vec<u64>> = extract_numbers_part_1(input);
    let result: u64 = calculate_solution(&operators, &numbers);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let operators: Vec<char> = extract_operator_chars(input);
    let numbers = extract_numbers_part_2(input);
    let result: u64 = calculate_solution(&operators, &numbers);
    Some(result)
}

fn calculate_solution(operators: &Vec<char>, numbers: &Vec<Vec<u64>>) -> u64 {
    numbers.iter().enumerate().map(|(i, numbers_in_problem)| {
        let (init, operation) = to_fold_input(operators[i]);
        numbers_in_problem.iter().fold(init, operation)
    }).sum()
}

fn extract_operator_chars(input: &str) -> Vec<char> {
    Regex::new(r"\s+")
        .unwrap()
        .split(input.lines().last().unwrap())
        .filter(|part| !part.is_empty())
        .map(|part| part.chars().nth(0).unwrap())
        .collect::<Vec<char>>()
}

fn extract_numbers_part_1(input: &str) -> Vec<Vec<u64>> {
    let spaces_regex = Regex::new(r"\s+").unwrap();

    let cells: Vec<Vec<&str>> = input
        .lines()
        .map(|line| {
            spaces_regex
                .split(line)
                .filter(|cell| !cell.is_empty())
                .collect()
        })
        .collect();

    let number_cells: Vec<Vec<u64>> = cells
        .iter()
        .take(cells.len() - 1)
        .map(|row| {
            row.iter()
                .map(|cell| cell.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let mut problems: Vec<Vec<u64>> = Vec::new();
    for x in 0..number_cells.first().unwrap().len() {
        let mut numbers_in_problem: Vec<u64> = Vec::new();
        for y in 0..number_cells.len() {
            numbers_in_problem.push(number_cells[y][x]);
        }
        problems.push(numbers_in_problem);
    }
    problems
}

fn extract_numbers_part_2(input: &str) -> Vec<Vec<u64>> {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut problems: Vec<Vec<u64>> = Vec::new();
    let mut numbers_in_problem: Vec<u64> = Vec::new();
    for x in (0..chars.first().unwrap().len()).rev() {
        let mut number_chars: String = String::new();
        for y in 0..(chars.len() - 1) {
            let c: char = chars[y][x];
            if c != ' ' {
                number_chars.push(c);
            }
        }
        if !number_chars.is_empty() {
            numbers_in_problem.push(number_chars.parse::<u64>().unwrap());
        }

        if chars.last().unwrap()[x] != ' ' {
            problems.push(numbers_in_problem);
            numbers_in_problem = Vec::new();
        }
    }

    problems.into_iter().rev().collect()
}

fn to_fold_input(operator_char: char) -> (u64, Box<dyn FnMut(u64, &u64) -> u64>) {
    if operator_char == '+'
    {
        (0, Box::new(|first, second| first + second))
    } else {
        (1, Box::new(|first, second| first * second))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
