use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub mod device;
pub mod instruction;
pub mod parser;

pub fn read_lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("No such file...");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line..."))
        .collect()
}
