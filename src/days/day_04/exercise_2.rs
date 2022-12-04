use super::pair::Pair;

pub fn solve(input: &Vec<Pair>) -> u32 {
    input.iter().filter(|pair| pair.ranges_overlap()).count() as u32
}
