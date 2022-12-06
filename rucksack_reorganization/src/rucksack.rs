// src/rucksack.rs

use crate::priority;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Rucksack {
    compartment1: String,
    compartment2: String,
    shared_item: char,
    pub shared_item_priority: i32,
}

impl Rucksack {
    pub fn new(contents: &String) -> Self {
        let mut compartment1 = contents.clone();
        let num_items = compartment1.len() / 2; // number of items per compartment

        let compartment2 = compartment1.split_off(num_items);

        let shared_item = compartment1
            .chars()
            .find(|&item| compartment2.contains(item))
            .unwrap_or('?');

        Self {
            compartment1,
            compartment2,
            shared_item,
            shared_item_priority: priority::char_to_priority(shared_item),
        }
    }
}
