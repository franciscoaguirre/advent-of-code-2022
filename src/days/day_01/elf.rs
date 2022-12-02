pub type Calories = u32;

#[derive(Debug, Default)]
pub struct Elf {
    food_items: Vec<Calories>,
    total_calories: Calories,
}

impl Elf {
    pub fn add_food_item(&mut self, calories: Calories) {
        self.food_items.push(calories);
        self.total_calories += calories;
    }

    pub fn get_total_calories(&self) -> Calories {
        self.total_calories
    }
}
