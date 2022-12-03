use super::rucksack::Rucksack;

pub fn solve(input: &Vec<Rucksack>) -> u32 {
    input
        .iter()
        .map(|rucksack| {
            rucksack
                .find_shared_type_in_compartments()
                .unwrap()
                .get_priority()
        })
        .sum()
}
