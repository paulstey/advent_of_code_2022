use rucksack_reorganization::rucksack::Rucksack;
use time::Instant;

fn main() {
    let t1 = Instant::now();
    let lines_string_vec = rucksack_reorganization::read_lines_from_file("data/data.txt");

    let solution = lines_string_vec
        .iter()
        .fold(0, |acc, line| acc + Rucksack::new(line).shared_item_priority);

    println!("{:?}", Instant::now() - t1);
    println!("{}", solution);
}
