// src/step.rs

#[derive(Debug)]
pub struct Step {
    pub n_moves: i32,
    pub src: i32,
    pub dest: i32,
}

impl Step {
    pub fn new(n_moves: i32, src: i32, dest: i32) -> Self {
        Self { n_moves, src, dest }
    }
}

pub fn parse_step(step_str: &String) -> Step {
    let split_vec: Vec<&str> = step_str.split("from").collect();
    let n_moves = split_vec[0]
        .replace("move", "")
        .trim()
        .parse()
        .expect("Failed to parse `n_moves`");

    let src_dest_vec: Vec<&str> = split_vec[1].split("to").collect();
    let src = src_dest_vec[0]
        .trim()
        .parse()
        .expect("Failed to parse `src`");

    let dest = src_dest_vec[1]
        .trim()
        .parse()
        .expect("Failed to parse `dest`");

    Step::new(n_moves, src, dest)
}
