// src/pair.rs

use std::ops::RangeInclusive;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Pair {
    assignment1: RangeInclusive<i32>,
    assignment2: RangeInclusive<i32>,
    pub is_subset: bool,
    pub overlaps: bool,
}

impl Pair {
    pub fn new(range1: (i32, i32), range2: (i32, i32)) -> Self {
        let (start1, end1) = range1;
        let (start2, end2) = range2;

        let is_subset = if start1 <= start2 && end1 >= end2 {
            true
        } else if start2 <= start1 && end2 >= end1 {
            true
        } else {
            false
        };

        let assignment1 = start1..=end1;
        let assignment2 = start2..=end2;

        let overlaps = if is_subset {
            true
        } else if assignment1.contains(&start2) || assignment1.contains(&end2) {
            true
        } else {
            false
        };

        Self {
            assignment1,
            assignment2,
            is_subset,
            overlaps,
        }
    }
}

pub fn parse_pair_numbers(line: &String) -> ((i32, i32), (i32, i32)) {
    let line_split: Vec<&str> = line.split(",").collect();
    let assignment1_vec: Vec<i32> = line_split[0]
        .split("-")
        .map(|num| num.parse::<i32>().expect("Failed to parse `num`"))
        .collect::<Vec<i32>>();

    let assignment2_vec: Vec<i32> = line_split[1]
        .split("-")
        .map(|num| num.parse::<i32>().expect("Failed to parse `num`"))
        .collect();

    (
        (assignment1_vec[0], assignment1_vec[1]),
        (assignment2_vec[0], assignment2_vec[1]),
    )
}
