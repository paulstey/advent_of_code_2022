use rucksack_reorganization::group::Group;
use time::Instant;

fn main() {
    let t1 = Instant::now();
    let lines_string_vec = rucksack_reorganization::read_lines_from_file("data/data.txt");

    let solution = lines_string_vec.chunks(3).fold(0, |acc, lines| {
        acc + Group::new(&lines[0], &lines[1], &lines[2]).shared_item_priority
    });

    println!("{:?}", Instant::now() - t1);
    println!("{:?}", solution);
}
