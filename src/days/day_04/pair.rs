pub struct Pair {
    range_1: Range,
    range_2: Range,
}

struct Range {
    min: u32,
    max: u32,
}

impl Range {
    pub fn min(&self) -> u32 {
        self.min
    }

    pub fn max(&self) -> u32 {
        self.max
    }
}

impl Pair {
    pub fn from(input: &str) -> Self {
        let ranges = input.split(',').collect::<Vec<_>>();

        let first_range_str = ranges[0].split('-').collect::<Vec<_>>();
        let first_range = Range {
            min: first_range_str[0].parse().unwrap(),
            max: first_range_str[1].parse().unwrap(),
        };

        let second_range_str = ranges[1].split('-').collect::<Vec<_>>();
        let second_range = Range {
            min: second_range_str[0].parse().unwrap(),
            max: second_range_str[1].parse().unwrap(),
        };

        Self {
            range_1: first_range,
            range_2: second_range,
        }
    }

    pub fn one_contains_the_other(&self) -> bool {
        if (self.range_1.min() <= self.range_2.min() && self.range_1.max() >= self.range_2.max())
            || (self.range_2.min() <= self.range_1.min()
                && self.range_2.max() >= self.range_1.max())
        {
            return true;
        }

        false
    }

    pub fn ranges_overlap(&self) -> bool {
        if (self.range_1.min() <= self.range_2.max() && self.range_1.max() >= self.range_2.min())
            || (self.range_2.min() <= self.range_1.max()
                && self.range_2.max() >= self.range_1.min())
        {
            return true;
        }

        false
    }
}
