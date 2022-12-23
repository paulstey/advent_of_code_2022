// src/parser.rs

use crate::motion::Motion;

pub fn get_motions_vector(motions_vec: &[String]) -> Vec<Motion> {
    motions_vec.iter().map(|line| Motion::new(line)).collect()
}
