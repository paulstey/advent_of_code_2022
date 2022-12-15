use treetop_tree_house::{self, parser};
use time::Instant;

fn main() {
    let t1 = Instant::now();
    let lines_string_vec = treetop_tree_house::read_lines_from_file("data/test_data.txt");

    let num_vec_vec = lines_string_vec
        .iter()
        .map(|line| parser::parse(&line))
        .collect::<Vec<Vec<i32>>>();
 
    let arr = Array2::from_shape_vec((nrows, ncols), data)?; 

    println!("{:?}", Instant::now() - t1);
    println!("{:?}", solution); 
}
