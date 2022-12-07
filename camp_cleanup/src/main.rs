use camp_cleanup;
use camp_cleanup::pair::{self, Pair};
use time::Instant;

fn main() {
    let t1 = Instant::now();
    let lines_string_vec = camp_cleanup::read_lines_from_file("data/data.txt");

    let solution = lines_string_vec
        .iter()
        .map(|line| {
            let pair_tuple = pair::parse_pair_numbers(&line);
            Pair::new(pair_tuple.0, pair_tuple.1)
        })
        .filter(|pair| pair.overlaps)
        .collect::<Vec<Pair>>()
        .len();

    println!("{:?}", Instant::now() - t1);
    println!("{:?}", solution);
}
