use super::{GeneralTree, TARGET_FREE_SPACE, TOTAL_DISK_SPACE};

pub fn solve(input: &GeneralTree) -> u32 {
    let total_used_space = input.get_total_used_space();
    let free_space = TOTAL_DISK_SPACE - total_used_space;
    let space_needed = TARGET_FREE_SPACE - free_space;

    let all_directory_sizes = input.get_all_directory_sizes();

    all_directory_sizes
        .iter()
        .filter(|&size| *size >= space_needed)
        .min()
        .unwrap()
        .to_owned()
}
