use rucksack_reorganization::group::Group;
use time::Instant;

fn main() {
    let t1 = Instant::now();
    let _lines_string_vec = rucksack_reorganization::read_lines_from_file("data/data.txt");

    //    let solution = lines_string_vec
    //        .iter()
    //        .fold(0, |acc, line| acc + Rucksack::new(line).shared_item_priority);

    let solution = Group::new(
        "DsPhSBQQQhqmBDhPDsFwjwsLjlRjlttvjvvtRb",
        "rNJMNNbrHrtjHLHjvwtg",
        "fNbNzZdrZnMnMPnQShFPDmnqFm",
    );

    println!("{:?}", Instant::now() - t1);
    println!("{}", solution);
}
