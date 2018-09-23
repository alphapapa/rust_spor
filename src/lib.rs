extern crate ndarray;
extern crate ordered_float;

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

// TODO: These should just return an f32
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
pub fn build_score_matrix(a: &str, b: &str) -> (ndarray::Array2<f32>,
                                                ndarray::Array2<u8>) {
    let mut score_matrix = ndarray::Array2::<f32>::zeros(
        (a.len() + 1, b.len() + 1));
    let mut traceback_matrix = ndarray::Array2::<u8>::zeros(
        (a.len() + 1, b.len() + 1));

    for (row, a_char) in a.chars().enumerate() {
        for (col, b_char) in b.chars().enumerate() {
            let row = row + 1;
            let col = col + 1;
            let match_score = score_func(a_char, b_char) as f32;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]  
    fn canned_score_matrix() {
        let input1 = "GGTTGACTA";
        let input2 = "TGTTACGG";
        let expected = ndarray::Array::from_shape_vec(
            (10, 9),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0,
                 0, 0, 3, 1, 0, 0, 0, 3, 3,
                 0, 0, 3, 1, 0, 0, 0, 3, 6,
                 0, 3, 1, 6, 4, 2, 0, 1, 4,
                 0, 3, 1, 4, 9, 7, 5, 3, 2,
                 0, 1, 6, 4, 7, 6, 4, 8, 6,
                 0, 0, 4, 3, 5, 10, 8, 6, 5,
                 0, 0, 2, 1, 3, 8, 13, 11, 9,
                 0, 3, 1, 5, 4, 6, 11, 10, 8,
                 0, 1, 0, 3, 2, 7, 9, 8, 7].iter().map(|n| {*n as f32}).collect()).unwrap();
        
        let (score_matrix, _) = build_score_matrix(input1, input2);
        assert_eq!(expected, score_matrix);
    }

}
