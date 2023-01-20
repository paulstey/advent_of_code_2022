// src/parser.rs

use crate::instruction::Instruction;

pub fn get_instructions(instructions_vec: &[String]) -> Vec<Instruction> {
    instructions_vec
        .iter()
        .map(|line| Instruction::new(line))
        .collect()
}
