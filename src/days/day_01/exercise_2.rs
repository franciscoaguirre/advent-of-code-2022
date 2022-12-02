use super::elf::{Calories, Elf};

pub fn solve(input: &mut Vec<Elf>) -> Calories {
    input.sort_by(|a, b| {
        b.get_total_calories()
            .partial_cmp(&a.get_total_calories())
            .unwrap()
    });
    input
        .iter()
        .take(3)
        .map(|elf| elf.get_total_calories())
        .sum()
}
