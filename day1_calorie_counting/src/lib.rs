use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn string_vec_to_i32_vec(lines_string_vec: &Vec<String>) -> Result<Vec<i32>> {
    let lines_i32_vec: Vec<i32> = lines_string_vec
        .iter()
        .map(|line| match line.parse::<i32>() {
            Ok(num) => num,
            Err(_e) => 0,
        })
        .collect();

    Ok(lines_i32_vec)
}

// ---
