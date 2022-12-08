use super::{GeneralTree, MAX_DIRECTORY_SIZE};

pub fn solve(input: &GeneralTree) -> u32 {
    let all_directory_sizes = input.get_all_directory_sizes();

    all_directory_sizes
        .iter()
        .filter(|&size| *size <= MAX_DIRECTORY_SIZE)
        .sum()
}
