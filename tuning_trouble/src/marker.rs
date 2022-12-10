// src/marker.rs

use itertools::Itertools;

pub fn find_first_marker(buffer: &String) -> i32 {
    let mut solution = 0;
    let buffer: Vec<char> = buffer.chars().collect();

    for i in 13..buffer.len() {
        let window = vec![
            buffer[i - 13],
            buffer[i - 12],
            buffer[i - 11],
            buffer[i - 10],
            buffer[i - 9],
            buffer[i - 8],
            buffer[i - 7],
            buffer[i - 6],
            buffer[i - 5],
            buffer[i - 4],
            buffer[i - 3],
            buffer[i - 2],
            buffer[i - 1],
            buffer[i],
        ];

        if window.into_iter().unique().collect::<Vec<char>>().len() == 14 {
            solution = i + 1;
            break;
        }
    }
    solution.try_into().unwrap()
}
