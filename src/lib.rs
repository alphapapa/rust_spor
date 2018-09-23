extern crate ordered_float;
extern crate matrix;

use matrix::prelude::*;

// TODO: Consider writing our own matrix for the learning experience.
// TODO: Consider using the seal library for smith-waterman. Once we learn how to do it ourselves...

#[derive(Debug)]
enum Direction {
    NONE,
    DIAG,
    UP,
    LEFT,
}

fn direction_value(direction: &Direction) -> u8 {
    match direction {
        Direction::NONE => 0x00,
        Direction::DIAG => 0x01,
        Direction::UP => 0x02,
        Direction::LEFT => 0x04,
    }
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

// TODO: score and penalty functions should be arguments.
pub fn build_score_matrix(a: &str, b: &str) -> (matrix::format::Conventional<f32>,
                                                matrix::format::Conventional<u8>) {
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
            println!("{:?}", scores);

            let max_score = scores[0].0;
            println!("max_score: {:?}", max_score);

            let scores = scores.iter().take_while(|n| n.0 == max_score);
            // scores = itertools.takewhile(
            //     lambda x: x[0] == max_score,
            //     scores)

            score_matrix[(row, col)] = max_score;
            // score_matrix[row, col] = max_score

            for (_, direction) in scores {
                traceback_matrix[(row, col)] = traceback_matrix[(row, col)] | direction_value(direction);
            }
        }
    }

    (score_matrix, traceback_matrix)
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
