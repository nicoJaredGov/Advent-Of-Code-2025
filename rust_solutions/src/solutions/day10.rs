use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::utils::symmetric_difference_all;

pub fn sol(input: &str) -> usize {
    input.lines().fold(0, |count, machine| {
        let machine: Vec<_> = machine.split_whitespace().collect();

        let lights = machine.first().unwrap();
        let light_set: HashSet<usize> = lights[1..lights.len() - 1]
            .char_indices()
            .filter_map(|(i, c)| match c {
                '#' => Some(i),
                _ => None,
            })
            .collect();

        let buttons: HashMap<usize, HashSet<usize>> = machine[1..machine.len() - 1]
            .iter()
            .enumerate()
            .map(|(k, button)| {
                let button_set = to_button_set(button);
                (k, button_set)
            })
            .collect();

        for combo in buttons.keys().copied().powerset() {
            let button_combos = combo.iter().map(|k| buttons.get(k).unwrap()).collect_vec();
            let result = symmetric_difference_all(&button_combos);
            if result == light_set {
                return count + combo.len();
            }
        }

        count
    })
}

/// Expected input: Comma-separated list of numbers inbetween circle parantheses
///
/// example: (1,2,3,4)
fn to_button_set(button: &str) -> HashSet<usize> {
    return button[1..button.len() - 1]
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
}
