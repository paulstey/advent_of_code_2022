use time::Instant;
use rock_paper_scissors::{self, round::Round};

fn main() {
    let t1 = Instant::now();

    let lines_string_vec = rock_paper_scissors::read_lines_from_file("data/data.txt");

    let round_vec = lines_string_vec
        .iter()
        .map(|line| Round::new(line))
        .collect::<Vec<Round>>();

    let solution = round_vec.iter().fold(0, |acc, x| acc + x.score);

    println!("{:?}", Instant::now() - t1);
    println!("{:?}", solution);
}
