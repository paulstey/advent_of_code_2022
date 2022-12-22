// src/scenic.rs

use ndarray::Array2;

fn update_score(score: i32, trees_in_view: i32) -> i32 {
    let new_score = if score == 0 {
        trees_in_view
    } else if trees_in_view == 0 {
        score
    } else {
        score * trees_in_view
    };

    new_score
}

fn scenic_score(tree_mat: &Array2<u32>, row_idx: i32, col_idx: i32) -> i32 {
    let current_tree_height = tree_mat[[row_idx as usize, col_idx as usize]];
    let (nrow, ncol) = tree_mat.dim();

    let mut i = row_idx;
    let mut j = col_idx;

    let mut score = 0;
    let mut trees_in_view = 0;

    // looking upward in this loop:
    i -= 1;
    while i >= 0 {
        if current_tree_height > tree_mat[[i as usize, j as usize]] {
            trees_in_view += 1;
            i -= 1;
        } else {
            trees_in_view += 1;
            break;
        }
    }
    score = update_score(score, trees_in_view);
    trees_in_view = 0;

    // looking downward in this loop:
    j = col_idx;
    i = row_idx + 1;
    while i < (nrow as i32) {
        if current_tree_height > tree_mat[[i as usize, j as usize]] {
            trees_in_view += 1;
            i += 1;
        } else {
            trees_in_view += 1;
            break;
        }
    }
    score = update_score(score, trees_in_view);
    trees_in_view = 0;

    // looking leftward in this loop:
    i = row_idx;
    j = col_idx - 1;
    while j >= 0 {
        if current_tree_height > tree_mat[[i as usize, j as usize]] {
            trees_in_view += 1;
            j -= 1; 
        } else {
            trees_in_view += 1;
            break;
        }
    }

    score = update_score(score, trees_in_view);
    trees_in_view = 0;

    // looking rightward in this loop:
    i = row_idx;
    j = col_idx + 1;
    while j < (ncol as i32) {
        if current_tree_height > tree_mat[[i as usize, j as usize]] {
            trees_in_view += 1;
            j += 1;
        } else {
            trees_in_view += 1;
            break;
        }
    }

    score = update_score(score, trees_in_view);

    score
}

pub fn compute_max_scenic_scores(tree_mat: &Array2<u32>) -> i32 {
    let (nrow, ncol) = tree_mat.dim();

    let mut scenic_score_vec = Vec::new();

    for j in 1..(ncol - 1) {
        for i in 1..(nrow - 1) {
            scenic_score_vec.push(scenic_score(
                tree_mat,
                i.try_into().unwrap(),
                j.try_into().unwrap(),
            ));
        }
    }
    *scenic_score_vec
        .iter()
        .max()
        .expect("Iterator is empty; no max value...")
}
