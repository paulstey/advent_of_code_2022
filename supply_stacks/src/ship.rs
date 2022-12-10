// src/ship.rs

use crate::step::Step;

#[derive(Debug)]
pub struct Ship {
    stacks_vec: Vec<Stack>,
}

impl Ship {
    pub fn new(stacks_vec: Vec<Stack>) -> Self {
        Self {
            stacks_vec: stacks_vec.to_vec(),
        }
    }

    pub fn execute_moves(&mut self, step: &Step) {
        let mut crate_char_vec =  Vec::new(); 

        for _i in 0..step.n_moves {
            let crate_char = self.stacks_vec[(step.src - 1) as usize]
                .crates_vec
                .pop()
                .expect("Failed to pop `crate_char` from `crates_vec`");

            crate_char_vec.push(crate_char);
        }  

        crate_char_vec.reverse();

        self.stacks_vec[(step.dest - 1) as usize]
            .crates_vec
            .append(&mut crate_char_vec);
    }

    pub fn collect_top_crates(&mut self) -> String {
        let top_crates: Vec<String> = self
            .stacks_vec
            .iter_mut()
            .map(|stack| {
                stack
                    .crates_vec
                    .pop()
                    .expect("Failed to pop crate off the top of stack")
            })
            .map(|crate_char| crate_char.to_string())
            .collect();

        top_crates.join("")
    }
}

#[derive(Debug, Clone)]
pub struct Stack {
    crates_vec: Vec<char>,
}

impl Stack {
    pub fn new(crates_vec: &[char]) -> Self {
        Self {
            crates_vec: crates_vec.to_vec(),
        }
    }
}
