// src/motion.rs

#[derive(Debug)]
pub enum Motion {
    Up(i32),
    Down(i32),
    Right(i32),
    Left(i32),
}

impl Motion {
    pub fn new(line: &str) -> Self {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        let direction = line_split[0];
        let n_steps = line_split[1]
            .parse::<i32>()
            .expect("Failed to parse `n_steps` from `line_split[1]`...");

        match direction {
            "U" => Motion::Up(n_steps),
            "D" => Motion::Down(n_steps),
            "L" => Motion::Left(n_steps),
            "R" => Motion::Right(n_steps),
            _ => panic!("Found `direction` not in (U, D, L, R) set..."),
        }
    }
}
