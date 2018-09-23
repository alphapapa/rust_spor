extern crate ndarray;
extern crate ordered_float;

pub mod scoring;

// TODO: Consider using the seal library for smith-waterman. Once we learn how to do it ourselves...

type ScoringFunction = Fn(char, char) -> f32;
type GapPenaltyFunction = Fn(u32) -> f32;

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

pub fn build_score_matrix(
    a: &str,
    b: &str,
    score_func: &ScoringFunction,
    gap_penalty: &GapPenaltyFunction) -> (ndarray::Array2<f32>,
                                          ndarray::Array2<u8>) {
    let mut score_matrix = ndarray::Array2::<f32>::zeros(
        (a.len() + 1, b.len() + 1));
    let mut traceback_matrix = ndarray::Array2::<u8>::zeros(
        (a.len() + 1, b.len() + 1));

    for (row, a_char) in a.chars().enumerate() {
        for (col, b_char) in b.chars().enumerate() {
            let row = row + 1;
            let col = col + 1;
            let match_score = score_func(a_char, b_char);

            let mut scores = [
                (score_matrix[(row - 1, col - 1)] + match_score,
                 Direction::DIAG),
                (score_matrix[(row - 1, col)] - gap_penalty(1),
                 Direction::UP),
                (score_matrix[(row, col - 1)] - gap_penalty(1),
                 Direction::LEFT),
                (0.0, Direction::NONE)
            ];
            scores.sort_by_key(|k| ordered_float::OrderedFloat(k.0));
            scores.reverse();

            let max_score = scores[0].0;

            let scores = scores.iter().take_while(|n| n.0 == max_score);

            score_matrix[(row, col)] = max_score;

            for (_, direction) in scores {
                traceback_matrix[(row, col)] = traceback_matrix[(row, col)] | direction_value(direction);
            }
        }
    }

    (score_matrix, traceback_matrix)
}

