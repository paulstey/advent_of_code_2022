// src/main.rs

use anyhow::Result;
use time::Instant;
use treetop_tree_house::{self, parser, visible};

fn main() -> Result<()> {
    let t1 = Instant::now();
    let lines_string_vec = treetop_tree_house::read_lines_from_file("data/data.txt");

    let mat = parser::lines_vec_to_array2(&lines_string_vec); 

    let solution = visible::flag_visible_trees(&mat).sum();

    println!("{:?}", Instant::now() - t1);
    println!("{:?}", solution);

    Ok(())
}
