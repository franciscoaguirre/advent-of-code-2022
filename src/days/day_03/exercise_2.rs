use super::group::Group;

pub fn solve(input: &Vec<Group>) -> u32 {
    input
        .iter()
        .map(|group| group.find_shared_type_in_members(3).unwrap().get_priority())
        .sum()
}
