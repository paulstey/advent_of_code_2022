// src/main.rs

use anyhow::Result;
use rope_bridge::{self, parser, rope::Rope, motion::Motion};
use time::Instant;

fn main() -> Result<()> {
    let t1 = Instant::now();
    let lines_string_vec = rope_bridge::read_lines_from_file("data/data.txt");

    let motions = parser::get_motions_vector(&lines_string_vec);
    let solution = compute_rope_path(&motions);

    println!("{:?}", Instant::now() - t1);
    println!("{:?}", solution);

    Ok(())
}



fn compute_rope_path(motions: &Vec<Motion>) -> u32 {
    let mut rope: Rope<10> = Rope::new(); 

    motions.iter().for_each(|motion| rope.move_head_knot(motion));

    let n_unique_tail_positions = rope.path.len() as u32;
    
    n_unique_tail_positions
}
