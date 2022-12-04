use super::pair::Pair;

pub fn solve(input: &Vec<Pair>) -> u32 {
    input
        .iter()
        .filter(|pair| pair.one_contains_the_other())
        .count() as u32
}
