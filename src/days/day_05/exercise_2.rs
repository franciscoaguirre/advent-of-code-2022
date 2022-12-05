use super::{Crates, Instruction, STACKS};

pub fn solve(crates: &mut Crates<STACKS>, instructions: &[Instruction]) -> String {
    instructions
        .iter()
        .for_each(|instruction| crates.move_crates(instruction));

    crates.return_top_crates()
}
