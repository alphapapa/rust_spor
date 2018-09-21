extern crate ordered_float;
extern crate matrix;

use matrix::prelude::*;
use std::fmt;

// TODO: Consider writing our own matrix for the learning experience.
// TODO: Consider using the seal library for smith-waterman. Once we learn how to do it ourselves...

#[derive(Debug)]
enum Direction {
    DIAG,
    UP,
    LEFT,
    NONE,
}

fn score_func(a: char, b: char) -> i32 {
    if a == b {
        3
    } else {
        -3
    }
}

fn gap_penalty(gap: u32) -> u32 {
    if gap == 1 {
        2
    } else {
        gap * gap_penalty(1)
    }
}

pub fn build_score_matrix(a: &str, b: &str) -> usize {
    let mut score_matrix: Conventional<f32> = Conventional::zero(
        (a.len() + 1, b.len() + 1));
    let mut traceback_matrix: Conventional<u8> = Conventional::zero(
        (a.len() + 1, b.len() + 1));

    for (row, row_char) in a.chars().enumerate().take(a.len() - 1) {
        for (col, col_char) in b.chars().enumerate().take(b.len() - 1) {
            let row = row + 1;
            let col = col + 1;
            let match_score = score_func(row_char, col_char) as f32;

            let mut scores = [
                (score_matrix[(row - 1, col - 1)] + match_score,
                 Direction::DIAG),
                (score_matrix[(row - 1, col)] - (gap_penalty(1) as f32),
                 Direction::UP),
                (score_matrix[(row, col - 1)] - (gap_penalty(1) as f32),
                 Direction::LEFT),
                (0.0, Direction::NONE)
            ];
            scores.sort_by_key(|k| ordered_float::OrderedFloat(k.0));
            scores.reverse();
            let scores = scores;
            println!("{:?}", scores);

            // max_score = scores[0][0]
            // scores = itertools.takewhile(
            //     lambda x: x[0] == max_score,
            //     scores)

            // score_matrix[row, col] = max_score
            // for _, direction in scores:
            //     traceback_matrix[row, col] = traceback_matrix[row, col] | direction.value


            // println!("{} {} {} {}", row, row_char, col, col_char);
        }
    }

    // return score_matrix, traceback_matrix
    score_matrix.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_also_works() {
        let l = build_score_matrix("asdf", "zxcv");
        assert_eq!(l, 25);
    }
}
