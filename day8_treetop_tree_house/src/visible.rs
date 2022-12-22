// src/visible.r
//
// NOTE: This is used for Part I of Problem 8

use ndarray::Array2;

pub fn flag_visible_trees(tree_mat: &Array2<u32>) -> Array2<u32> {
    let shape = tree_mat.dim();
    let mut visible_mat = Array2::zeros(shape);

    let (nrows, ncols) = shape;

    for j in 0..ncols {
        visible_mat[[0, j]] = 1;

        let mut current_max = tree_mat[[0, j]];

        // Looking from north in this loop
        for i in 1..nrows {
            if tree_mat[[i, j]] > current_max {
                visible_mat[[i, j]] = 1;
                current_max = tree_mat[[i, j]];
            }
        }

        visible_mat[[nrows - 1, j]] = 1;

        let mut current_max = tree_mat[[nrows - 1, j]];

        // Looking from south in this loop
        for i in (1..nrows).rev() {
            if tree_mat[[i, j]] > current_max {
                visible_mat[[i, j]] = 1;
                current_max = tree_mat[[i, j]];
            }
        }
    }

    for i in 0..nrows {
        visible_mat[[i, 0]] = 1;

        let mut current_max = tree_mat[[i, 0]];

        // Looking from west in this loop
        for j in 1..ncols {
            if tree_mat[[i, j]] > current_max {
                visible_mat[[i, j]] = 1;
                current_max = tree_mat[[i, j]];
            }
        }

        visible_mat[[i, ncols - 1]] = 1;

        let mut current_max = tree_mat[[i, ncols - 1]];

        // Looking from east in this loop
        for j in (1..ncols).rev() {
            if tree_mat[[i, j]] > current_max {
                visible_mat[[i, j]] = 1;
                current_max = tree_mat[[i, j]];
            }
        }
    }

    visible_mat
}
