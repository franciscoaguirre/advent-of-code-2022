use super::round::Round;

pub fn solve(input: &Vec<Round>) -> u32 {
    input.iter().map(|round| round.calculate_score()).sum()
}
