use anyhow::Result;
use rock_paper_scissors::{self, round::Round};

fn main() {
    let lines_string_vec = rock_paper_scissors::read_lines_from_file("data/data.txt");

    let round_vec = lines_string_vec
        .iter()
        .map(|line| Round::new(line))
        .collect::<Vec<Round>>();

    println!("{:?}", round_vec);

}
