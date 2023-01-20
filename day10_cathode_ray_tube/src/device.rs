// src/device.rs

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::instruction::Instruction;

#[derive(Debug)]
pub struct Device {
    register: i32,
    cycle: usize,
    pixel_array: [bool; 240],
}

impl Device {
    pub fn new() -> Self {
        Self {
            register: 1,
            cycle: 0,
            pixel_array: [false; 240],
        }
    }

    fn noop(&mut self) {
       let sprite_range = (self.register - 1)..=(self.register + 1);

        // The current line position is the cycle wrapped to 40-width lines. So, on
        // cycle 40, we've wrapped around to the first pixel of the second line.
        let line_pos = (self.cycle % 40) as i32;

        // If the current line position is within the sprite, set the corresponding
        // pixel in our array of pixels to `true`.
        if sprite_range.contains(&line_pos) {
            self.pixel_array[self.cycle] = true;
        }

        self.cycle += 1; 

    }

    fn addx(&mut self, x: i32) {
        self.noop();
        self.noop();
        self.register += x;
    }

    fn execute(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Noop => self.noop(),
            Instruction::Addx(x) => self.addx(*x),
        }
    }

    pub fn execute_instructions(&mut self, instruction_vec: &[Instruction]) -> String {
        instruction_vec.iter().for_each(|instr| self.execute(instr));

    
        PrettyPixels(self.pixel_array).to_string()
    }
}


/// This is how we worry about displaying pixels. Using the newtype syntax, we can
/// implement a standard library trait on a (wrapped) standard library data structure.
/// In this case, we want to implement `Display` for our array of pixels so we can
/// convert it to a string that can show us the answer to part two.
pub struct PrettyPixels([bool; 240]);

#[rustfmt::skip]
impl Display for PrettyPixels {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for (idx, pixel) in self.0.iter().enumerate() {
            // Wrap the pixel lines to a width of 40 characters
            if (idx % 40 == 0) && idx > 0 { writeln!(f)?; }

            // If the pixel is lit, print a '#', other wise print a space
            let glyph = if *pixel { "#" } else { " " };
            write!(f, "{glyph}")?;
        }

        write!(f, "") // Finish the print results
    }
}
