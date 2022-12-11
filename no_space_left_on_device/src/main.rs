use no_space_left_on_device;
use no_space_left_on_device::directory::{self, Directory};
use time::Instant;

fn main() {
    let t1 = Instant::now();
    let lines_string_vec = no_space_left_on_device::read_lines_from_file("data/test_data.txt");

    let mut directory_tree = Directory::new("/".to_string(), None);
    let mut current_parent = directory_tree.clone();

    let i = 1;
    while i < lines_string_vec.len() {
        if lines_string_vec[i].contains("$ ls".to_string()) {
            let num_items_in_dir = directory::count_items_in_dir(i, &lines_string_vec);

            for j in 1..=num_items_in_dir {
                if lines_string_vec[i + j].contains("dir ".to_string()) {
                    let name = lines_string_vec[i + j].remove("dir ").trim();
                    let dir = Directory::new(name, current_parent); 
                    
                }
            }
        }
    }
    println!("{:?}", Instant::now() - t1);
    println!("{:?}", directory);
}
