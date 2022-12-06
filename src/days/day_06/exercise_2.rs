use std::collections::HashSet;

use super::WINDOW_SIZE_FOR_MESSAGE;

pub fn solve(input: &[char]) -> usize {
    let mut done = false;
    let mut result = 0;

    input
        .windows(WINDOW_SIZE_FOR_MESSAGE)
        .enumerate()
        .for_each(|(index, window)| {
            let mut set = HashSet::new();

            window.iter().for_each(|char| {
                set.insert(char);
            });

            // No duplicates
            if set.len() == WINDOW_SIZE_FOR_MESSAGE && !done {
                result = index + WINDOW_SIZE_FOR_MESSAGE;
                done = true;
            }
        });

    result
}
