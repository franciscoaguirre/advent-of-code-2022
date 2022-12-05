#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    how_many: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    #[cfg(test)]
    pub fn new(how_many: usize, from: usize, to: usize) -> Self {
        Self { how_many, from, to }
    }

    pub fn from(line: &str) -> Self {
        let numbers: Vec<usize> = line
            .split_whitespace()
            .filter_map(|word| word.parse::<usize>().ok())
            .collect();

        Self {
            how_many: numbers[0],
            from: numbers[1],
            to: numbers[2],
        }
    }

    pub fn get_fields(&self) -> (usize, usize, usize) {
        (self.how_many, self.from, self.to)
    }
}
