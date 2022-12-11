pub struct Monkey {
    items: Vec<Item>,
    operation: Box<dyn FnMut(&mut Item)>,
    test: Box<dyn Fn(&Item) -> usize>,
    item_inspection_count: u32,
    pub divisible_by: u64,
    common_divisor: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    worry_level: u64,
}

impl Item {
    pub fn new(worry_level: u64) -> Self {
        Self { worry_level }
    }
}

impl Monkey {
    pub fn from(line: &[String]) -> Self {
        let mut lines = line.iter();

        lines.next().unwrap(); // Skip monkey index

        let starting_items_raw = lines.next().unwrap();
        let starting_items = Self::parse_starting_items(starting_items_raw);

        let operation_raw = lines.next().unwrap();
        let operation = Self::parse_operation(operation_raw);

        let test_raw: Vec<String> = lines.map(|line| line.clone()).collect();
        let (divisible_by, test) = Self::parse_test(test_raw);

        Self {
            items: starting_items,
            operation,
            test,
            item_inspection_count: 0,
            divisible_by,
            common_divisor: 0,
        }
    }

    pub fn inspect_items(&mut self, too_worried: bool) -> Vec<(Item, usize)> {
        let mut new_items = Vec::new();
        let num_items = self.items.len();
        for _ in 0..num_items {
            let item = self.items.remove(0);
            new_items.push(self.inspect_item(item, too_worried));
            self.item_inspection_count += 1;
        }
        new_items
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn get_inspection_count(&self) -> u32 {
        self.item_inspection_count
    }

    pub fn set_common_divisor(&mut self, common_divisor: u64) {
        self.common_divisor = common_divisor;
    }

    fn inspect_item(&mut self, mut item: Item, too_worried: bool) -> (Item, usize) {
        let operation = &mut self.operation;
        operation(&mut item);

        if !too_worried {
            item.worry_level /= 3;
        }

        let test = &self.test;
        let monkey_index_to_throw = test(&item);
        item.worry_level %= self.common_divisor;
        (item, monkey_index_to_throw)
    }

    fn parse_starting_items(starting_items_raw: &str) -> Vec<Item> {
        let colon_index = starting_items_raw.find(": ").unwrap();
        let items_raw = &starting_items_raw[colon_index + 2..];
        items_raw
            .split(", ")
            .map(|item_raw| item_raw.parse().unwrap())
            .map(|item| Item::new(item))
            .collect()
    }

    fn parse_operation(operation_raw: &str) -> Box<dyn FnMut(&mut Item)> {
        let old = "old ";
        let olds_index = operation_raw.find(old).unwrap();
        let expression_raw: Vec<&str> = operation_raw[olds_index + old.len()..]
            .split_whitespace()
            .collect();

        match expression_raw.as_slice() {
            ["+", operand] => {
                let operand = operand.parse::<u64>();
                if let Ok(operand) = operand {
                    Box::new(move |old| old.worry_level += operand)
                } else {
                    Box::new(move |old| old.worry_level += old.worry_level)
                }
            }
            ["*", operand] => {
                // TODO: This is overfitting to the input too hard
                let operand = operand.parse::<u64>();
                if let Ok(operand) = operand {
                    Box::new(move |old| old.worry_level *= operand)
                } else {
                    Box::new(move |old| old.worry_level *= old.worry_level)
                }
            }
            _ => panic!("Wrong number of arguments!"),
        }
    }

    fn parse_test(test_raw: Vec<String>) -> (u64, Box<dyn Fn(&Item) -> usize>) {
        match test_raw.as_slice() {
            [divisible_line, true_line, false_line]
            | [divisible_line, true_line, false_line, _] => {
                let divisible_by: u64 = divisible_line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();

                let monkey_to_throw_if_true: usize = true_line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();
                let monkey_to_throw_if_false: usize = false_line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();

                (
                    divisible_by,
                    Box::new(move |num| {
                        if &num.worry_level % divisible_by == 0 {
                            monkey_to_throw_if_true
                        } else {
                            monkey_to_throw_if_false
                        }
                    }),
                )
            }
            _ => panic!("Wrong number of lines left to parse!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() {
        let monkey_raw = [
            "Monkey 0:".to_string(),
            "  Starting items: 79, 98".to_string(),
            "  Operation: new = old * 19".to_string(),
            "  Test: divisible by 23".to_string(),
            "    If true: throw to monkey 2".to_string(),
            "    If false: throw to monkey 3".to_string(),
            "".to_string(),
        ];
        let monkey = Monkey::from(&monkey_raw);
        let mut monkey_operation = monkey.operation;
        let monkey_test = monkey.test;

        assert_eq!(monkey.items, vec![Item::new(79), Item::new(98)]);

        let mut item = Item::new(79);
        monkey_operation(&mut item);
        assert_eq!(item, Item::new(1501));

        let other_item = Item::new(500);
        assert_eq!(monkey_test(&other_item), 3);
    }
}
