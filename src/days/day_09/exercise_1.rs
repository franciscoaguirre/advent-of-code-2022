use super::Rope;

pub fn solve(input: &Rope) -> usize {
    input.get_tail_visited_positions_count()
}
