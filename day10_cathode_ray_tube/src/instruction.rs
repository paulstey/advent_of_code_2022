// src/instruction.rs

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    pub fn new(line: &str) -> Self {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        let instruction = line_split[0];

        let x_value = if line_split.len() == 2 {
            line_split[1]
                .parse::<i32>()
                .expect("Failed to parse `x_value` from `line_split[1]`...")
        } else {
            0
        };

        match instruction {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(x_value),
            _ => panic!("Found `instruction` not in (noop, addx) set..."),
        }
    }
}
