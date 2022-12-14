use super::Packet;

pub fn solve(pairs: &[(Packet, Packet)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(index, _)| index + 1)
        .sum()
}
