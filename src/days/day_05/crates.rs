use super::Instruction;

#[derive(Debug, Clone)]
pub struct Crates<const N: usize> {
    stacks: [Vec<char>; N],
}

impl<const N: usize> Crates<N> {
    pub fn new() -> Self {
        Self {
            stacks: [(); N].map(|_| Vec::new()),
        }
    }

    #[cfg(test)]
    pub fn stacks(&self) -> &[Vec<char>; N] {
        &self.stacks
    }

    pub fn add_crate(&mut self, stack_index: usize, _crate: char) {
        self.stacks[stack_index].push(_crate);
    }

    pub fn invert_order(&mut self) {
        for stack in &mut self.stacks {
            stack.reverse();
        }
    }

    pub fn move_crate(&mut self, instruction: &Instruction) {
        let (how_many, from, to) = instruction.get_fields();

        for _ in 0..how_many {
            let _crate = self.stacks[from - 1] // -1 because it's 1-indexed
                .pop()
                .expect("Crate to move was not there!");
            self.stacks[to - 1].push(_crate);
        }
    }

    pub fn move_crates(&mut self, instruction: &Instruction) {
        let (how_many, from, to) = instruction.get_fields();

        let crates_in_stack = self.stacks[from - 1].len();
        let mut crates_to_move: Vec<char> = self.stacks[from - 1]
            .drain(crates_in_stack - how_many..)
            .collect();
        self.stacks[to - 1].append(&mut crates_to_move);
    }

    pub fn return_top_crates(&self) -> String {
        let mut result = String::new();

        for stack in &self.stacks {
            let top_crate = stack.last().unwrap_or(&'\0');
            result.push_str(&top_crate.to_string());
        }

        result
    }
}
