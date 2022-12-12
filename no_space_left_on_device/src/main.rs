/// NOTE: This solution is largely a copy of ericwburden's solution, which can be
/// found here: `https://github.com/ericwburden/advent_of_code_2022`

use no_space_left_on_device;
use no_space_left_on_device::filesystem::FileSystem;
use no_space_left_on_device::parser;
use std::fs;
use time::Instant;

fn main() {
    let t1 = Instant::now();
    let input = fs::read_to_string("data/data.txt").expect("Could not read from file");

    let commands = parser::commands(&input).expect("Could not parse input!");

    let fs = FileSystem::try_from(commands).expect("Could not build filesystem!");
    fs.calculate_directory_sizes();

    // Calculate the space we need to free up as described by the puzzle
    let space_remaining = 70_000_000 - fs.total_size();
    let space_needed = 30_000_000 - space_remaining;

    let solution: u32 = fs
        .get_directory_sizes()
        .into_iter()
        .filter(|x| *x >= space_needed)
        .min()
        .unwrap()
        .into();

    println!("{:?}", Instant::now() - t1);
    println!("{:?}", solution);
}
