use std::collections::VecDeque;
use z3::ast::{Bool, Int};
use z3::{Optimize, SatResult};

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

    panic!("No solution found");
}

fn get_no_of_presses_for_joltage(machine: &Machine) -> u64 {
    let mut button_consts: Vec<Int> = vec![];
    for i in 0..machine.buttons.len() {
        button_consts.push(Int::fresh_const(format!("Button{}", i).as_str()));
    }

    let mut joltage_sums: Vec<Bool> = vec![];
    for joltage_index in 0..machine.target_joltage.len() {
        let mut contributing_buttons: Vec<&Int> = vec![];
        for btn_index in 0..machine.buttons.len() {
            if machine.buttons[btn_index]
                .indicies_to_handle
                .contains(&joltage_index)
            {
                contributing_buttons.push(&button_consts[btn_index])
            }
        }

        let joltage_sum = Int::add(&contributing_buttons).eq(machine.target_joltage[joltage_index]);
        joltage_sums.push(joltage_sum);
    }

    let optimizer = Optimize::new();
    button_consts
        .iter()
        .for_each(|btn_const| optimizer.assert(&btn_const.ge(0)));
    joltage_sums
        .iter()
        .for_each(|joltage_sum| optimizer.assert(joltage_sum));
    optimizer.minimize(&Int::add(&button_consts));

    let sat_result = optimizer.check(&[]);
    if sat_result != SatResult::Sat {
        panic!("No solution found");
    }

    let model = optimizer.get_model().unwrap();
    button_consts
        .iter()
        .map(|btn_cont| model.eval(btn_cont, true).unwrap().as_u64().unwrap())
        .sum()
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
