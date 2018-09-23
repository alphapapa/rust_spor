extern crate ndarray;
extern crate spor;

use spor::*;

fn score_func(a: char, b: char) -> f32 {
    if a == b {
        3.0
    } else {
        -3.0
    }
}

fn gap_penalty(gap: u32) -> f32 {
    if gap == 1 {
        2.0
    } else {
        (gap as f32) * gap_penalty(1)
    }
}

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
    
    let (score_matrix, _) = build_score_matrix(
        input1, input2, &score_func, &gap_penalty);

    assert_eq!(expected, score_matrix);
}
