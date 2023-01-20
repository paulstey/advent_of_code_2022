// src/main.rs

use anyhow::Result;
use cathode_ray_tube::{self, device::Device, parser};
use time::Instant;

fn main() -> Result<()> {
    let t1 = Instant::now();
    let lines_string_vec = cathode_ray_tube::read_lines_from_file("data/data.txt");

    let instructions = parser::get_instructions(&lines_string_vec);
    let mut device = Device::new();
    let solution = device.execute_instructions(&instructions);

    println!("{:?}", Instant::now() - t1);
    println!("{}", solution);

    Ok(())
}
