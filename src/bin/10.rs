use std::collections::VecDeque;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let solution = input
        .lines()
        .map(|line| Machine::from(line))
        .map(|machine| get_no_of_presses_for_lights(&machine))
        .sum();

    Some(solution)
}

pub fn part_two(input: &str) -> Option<u64> {
    let solution = input
        .lines()
        .map(|line| Machine::from(line))
        .map(|machine| get_no_of_presses_for_joltage(&machine))
        .sum();

    Some(solution)
}

fn get_no_of_presses_for_lights(machine: &Machine) -> u64 {
    let initial_lights = vec![false; machine.target_lights.len()];
    let mut known_states = vec![initial_lights.clone()];
    let mut states_to_check = VecDeque::from([LightState {
        no_of_presses: 0,
        lights: initial_lights,
    }]);

    while !states_to_check.is_empty() {
        let current_state = states_to_check.pop_front().unwrap();

        for button in &machine.buttons {
            let new_state = button.press_for_lights(&current_state);

            if known_states.contains(&&new_state.lights) {
                continue;
            }

            if new_state.lights == machine.target_lights {
                return new_state.no_of_presses;
            }

            known_states.push(new_state.lights.clone());
            states_to_check.push_back(new_state);
        }
    }

    0
}

fn get_no_of_presses_for_joltage(machine: &Machine) -> u64 {
    let mut lgs = vec![vec![0u64; machine.buttons.len() + 1]; machine.target_joltage.len()];

    // add buttons to matrix
    for button_index in 0..machine.buttons.len() {
        machine.buttons[button_index]
            .indicies_to_handle
            .iter()
            .for_each(|handled_index| lgs[*handled_index][button_index] = 1);
    }

    // add result to matrix
    let last_column = machine.buttons.len();
    machine.target_joltage.iter().enumerate().for_each(|(i, joltage)| lgs[i][last_column] = *joltage);

    // debug
    for row_index in 0..lgs.len() {
        for cell in lgs[row_index].iter() {
            print!("{} ", cell);
        }
        println!();
    }
    println!();

    // solve
    let result = solve_bareiss(lgs);

    0
}

#[derive(Debug)]
pub enum SolveStatus {
    Unique(Vec<i128>),
    Infinite,
    NoSolution,
}

pub fn solve_bareiss(mut m: Vec<Vec<u64>>) -> SolveStatus {
    let rows = m.len();
    let cols = m[0].len();
    let n = cols - 1; // variables

    // Convert to i128 to avoid overflow
    let mut a: Vec<Vec<i128>> = m
        .into_iter()
        .map(|r| r.into_iter().map(|x| x as i128).collect())
        .collect();

    let mut prev_pivot = 1i128;
    let mut rank = 0;

    for k in 0..rows.min(n) {
        // Find pivot
        let mut pivot_row = None;
        for i in k..rows {
            if a[i][k] != 0 {
                pivot_row = Some(i);
                break;
            }
        }

        let i = match pivot_row {
            Some(i) => i,
            None => continue,
        };

        a.swap(k, i);
        let pivot = a[k][k];

        for i in k + 1..rows {
            for j in k + 1..cols {
                a[i][j] =
                    (pivot * a[i][j] - a[i][k] * a[k][j]) / prev_pivot;
            }
            a[i][k] = 0;
        }

        prev_pivot = pivot;
        rank += 1;
    }

    // Check for inconsistency
    for i in rank..rows {
        let all_zero = (0..n).all(|j| a[i][j] == 0);
        if all_zero && a[i][n] != 0 {
            return SolveStatus::NoSolution;
        }
    }

    if rank < n {
        return SolveStatus::Infinite;
    }

    // Back substitution (now upper triangular)
    let mut x = vec![0i128; n];
    for i in (0..n).rev() {
        let mut sum = a[i][n];
        for j in i + 1..n {
            sum -= a[i][j] * x[j];
        }
        x[i] = sum / a[i][i];
    }

    SolveStatus::Unique(x)
}

#[derive(Debug)]
pub enum SolveError {
    SingularMatrix,
    DimensionMismatch,
}

pub fn gaussian_elimination(
    mut a: Vec<Vec<f64>>,
    mut b: Vec<f64>,
) -> Result<Vec<f64>, SolveError> {
    let n = a.len();

    if b.len() != n || a.iter().any(|row| row.len() != n) {
        return Err(SolveError::DimensionMismatch);
    }

    // Forward elimination
    for i in 0..n {
        // Partial pivoting: find max element in column i
        let mut max_row = i;
        for k in (i + 1)..n {
            if a[k][i].abs() > a[max_row][i].abs() {
                max_row = k;
            }
        }

        if a[max_row][i].abs() < 1e-12 {
            return Err(SolveError::SingularMatrix);
        }

        // Swap rows in A
        a.swap(i, max_row);
        // Swap corresponding values in b
        b.swap(i, max_row);

        // Eliminate entries below pivot
        for k in (i + 1)..n {
            let factor = a[k][i] / a[i][i];
            for j in i..n {
                a[k][j] -= factor * a[i][j];
            }
            b[k] -= factor * b[i];
        }
    }

    // Back substitution
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = b[i];
        for j in (i + 1)..n {
            sum -= a[i][j] * x[j];
        }
        x[i] = sum / a[i][i];
    }

    Ok(x)
}


struct LightState {
    no_of_presses: u64,
    lights: Vec<bool>,
}

#[derive(Debug)]
struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<MachineButton>,
    target_joltage: Vec<u64>,
}

impl Machine {
    fn from(line: &str) -> Machine {
        let mut parts = line.split_whitespace().collect::<VecDeque<&str>>();

        let lights_str = parts.pop_front().unwrap();
        let lights_str = &lights_str[1..lights_str.len() - 1];
        let lights: Vec<bool> = lights_str.chars().map(|c| c == '#').collect();

        let joltage_requirements_str = parts.pop_back().unwrap();
        let joltage_requirements_str =
            &joltage_requirements_str[1..joltage_requirements_str.len() - 1];
        let joltage_requirements = joltage_requirements_str
            .split(",")
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let buttons: Vec<MachineButton> = parts
            .iter()
            .map(|s| &s[1..s.len() - 1])
            .map(|s| MachineButton::from(s))
            .collect();

        Self {
            target_lights: lights,
            buttons,
            target_joltage: joltage_requirements,
        }
    }
}

#[derive(Debug)]
struct MachineButton {
    indicies_to_handle: Vec<usize>,
}

impl MachineButton {
    fn from(s: &str) -> MachineButton {
        let indicies_to_toggle = s
            .split(",")
            .map(|index| index.parse::<usize>().unwrap())
            .collect();
        Self {
            indicies_to_handle: indicies_to_toggle,
        }
    }

    fn press_for_lights(&self, state: &LightState) -> LightState {
        let mut new_state = state.lights.clone();
        self.indicies_to_handle.iter().for_each(|index| {
            new_state[*index] = !new_state[*index];
        });

        LightState {
            no_of_presses: state.no_of_presses + 1,
            lights: new_state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
