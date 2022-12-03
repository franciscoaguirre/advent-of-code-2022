use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Rucksack {
    compartments: Vec<Compartment>,
}

type Compartment = String;

pub struct Item(pub char);

impl Item {
    pub fn get_priority(&self) -> u32 {
        match self.0 {
            'a'..='z' => (self.0 as u32 - 'a' as u32) + 1,
            'A'..='Z' => (self.0 as u32 - 'A' as u32) + 26 + 1,
            _ => panic!("Unsupported char!"),
        }
    }
}

impl Rucksack {
    pub fn from(line: &str, num_compartments: u32) -> Self {
        let num_items = line.len();
        let compartments = line
            .chars()
            .collect::<Vec<char>>()
            .chunks(num_items / num_compartments as usize)
            .map(|chunk| chunk.iter().collect::<Compartment>())
            .collect::<Vec<Compartment>>();

        Self { compartments }
    }

    pub fn find_shared_type_in_compartments(&self) -> Option<Item> {
        let mut types_of_items = HashMap::new();

        let first_compartment = self.compartments.first().expect("No compartment found!");

        for char in first_compartment.chars() {
            *types_of_items.entry(char).or_insert(0) += 1;
        }

        for compartment in &self.compartments[1..] {
            for char in compartment.chars() {
                if types_of_items.contains_key(&char) {
                    return Some(Item(char));
                }
            }
        }

        None
    }

    pub fn get_full_content(&self) -> String {
        let mut result = String::new();

        for compartment in &self.compartments {
            result.push_str(compartment);
        }

        result
    }
}
