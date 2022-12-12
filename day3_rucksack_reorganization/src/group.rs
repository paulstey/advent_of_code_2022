// src/group.rs

use crate::priority;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Group {
    rucksack1: String,
    rucksack2: String,
    rucksack3: String,
    shared_item: char,
    pub shared_item_priority: i32,
}

impl Group {
    pub fn new(rucksack1: &String, rucksack2: &String, rucksack3: &String) -> Self {
        let rucksack_1and2_shared: Vec<char> = rucksack1
            .chars()
            .filter(|&item| rucksack2.contains(item))
            .collect();

        let shared_item = rucksack3
            .chars()
            .find(|&item| rucksack_1and2_shared.contains(&item))
            .unwrap_or('?');

        Self {
            rucksack1: rucksack1.to_string(),
            rucksack2: rucksack2.to_string(),
            rucksack3: rucksack3.to_string(),
            shared_item,
            shared_item_priority: priority::char_to_priority(shared_item),
        }
    }
}
