use super::elf::{Calories, Elf};

pub fn solve(input: &Vec<Elf>) -> Calories {
    input
        .iter()
        .map(|elf| elf.get_total_calories())
        .max()
        .unwrap()
}
