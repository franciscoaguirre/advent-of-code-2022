use std::collections::HashMap;

use super::rucksack::{Item, Rucksack};

pub struct Group(Vec<Rucksack>);

impl Group {
    pub fn new(rucksacks: &[Rucksack]) -> Self {
        Group(rucksacks.to_vec())
    }

    pub fn find_shared_type_in_members(&self, num_members: u32) -> Option<Item> {
        let mut all_types_of_items = HashMap::new();

        for rucksack in &self.0 {
            let mut types_of_items = HashMap::new();

            let full_content = rucksack.get_full_content();

            for char in full_content.chars() {
                *types_of_items.entry(char).or_insert(0) += 1;

                if let Some(1) = types_of_items.get(&char) {
                    *all_types_of_items.entry(char).or_insert(0) += 1;
                }
            }
        }

        for (&k, &v) in &all_types_of_items {
            if v == num_members {
                return Some(Item(k));
            }
        }

        None
    }
}
