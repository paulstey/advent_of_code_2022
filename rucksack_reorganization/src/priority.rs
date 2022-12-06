// src/priority.rs 


pub fn char_to_priority(item: char) -> i32 {
    match item.is_ascii_uppercase() {
        true => (item as i32) - 38,
        false => (item as i32) - 96,
    }
    
}
