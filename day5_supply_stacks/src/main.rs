use supply_stacks;
use supply_stacks::ship::{Ship, Stack};
use supply_stacks::step::{self, Step};
use time::Instant;

fn main() {
    let t1 = Instant::now();
    let lines_string_vec = supply_stacks::read_lines_from_file("data/data.txt");

    let steps_vec = lines_string_vec
        .iter()
        .map(|line| step::parse_step(&line))
        .collect::<Vec<Step>>();

    let mut ship = Ship::new(vec![
        Stack::new(&['D', 'H', 'N', 'Q', 'T', 'W', 'V', 'B']),
        Stack::new(&['D', 'W', 'B']),
        Stack::new(&['T', 'S', 'Q', 'W', 'J', 'C']),
        Stack::new(&['F', 'J', 'R', 'N', 'Z', 'T', 'P']),
        Stack::new(&['G', 'P', 'V', 'J', 'M', 'S', 'T']),
        Stack::new(&['B', 'W', 'F', 'T', 'N']),
        Stack::new(&['B', 'L', 'D', 'Q', 'F', 'H', 'V', 'N']),
        Stack::new(&['H', 'P', 'F', 'R']),
        Stack::new(&['Z', 'S', 'M', 'B', 'L', 'N', 'P', 'H']),
    ]);

    for step in steps_vec.iter() {
        ship.execute_moves(step);
    }

    println!("{:?}", Instant::now() - t1);
    println!("{:?}", ship.collect_top_crates());
}
