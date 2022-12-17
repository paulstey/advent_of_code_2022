// src/parser.rs

use ndarray::Array2;

fn parse(line: &String) -> Vec<u32> {
    line.chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|c| c.to_digit(10).expect("Failed to parse `c` to digit..."))
        .collect()
}

pub fn lines_vec_to_array2(lines_vec: &Vec<String>) -> Array2<u32> {
    let num_vec_vec = lines_vec
        .iter()
        .map(|line| parse(&line))
        .collect::<Vec<Vec<u32>>>();

    let nrows = num_vec_vec.len();
    let ncols = num_vec_vec
        .get(0)
        .expect("There was not a zero-th element in `num_vec_vec`")
        .len();

    let flat_vec = num_vec_vec.iter().flatten().cloned().collect();

    let arr = Array2::from_shape_vec((nrows, ncols), flat_vec)
        .expect("Failed to convert `flat_vec` to Array2");

    arr
}
