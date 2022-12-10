use std::fs;
use time::Instant;
use tuning_trouble::{self, marker};

fn main() {
    let t1 = Instant::now();
    let line_str = fs::read_to_string("data/data.txt").expect("Failed to read line");

    let solution = marker::find_first_marker(&line_str);

    println!("{:?}", Instant::now() - t1);
    println!("{:?}", solution);
}
