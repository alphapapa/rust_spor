extern crate ndarray;
extern crate spor;
extern crate ordered_float;

use ordered_float::*;
use spor::*;
use spor::scoring::*;

const INPUT1: &str = "GGTTGACTA";
const INPUT2: &str = "TGTTACGG";

#[test]
fn canned_tracebacks() {
    let (score_matrix, traceback_matrix) = build_score_matrix(
        INPUT1, INPUT2, &score_func, &gap_penalty);

    let max_idx = score_matrix.indexed_iter()
        .max_by_key(|n| OrderedFloat(*n.1))
        .unwrap().0;

    let tbs = tracebacks(&traceback_matrix, max_idx);
    assert_eq!(tbs.len(), 1);

    let expected = [
        (7, 6),
        (6, 5),
        (5, 4),
        (4, 4),
        (3, 3),
        (2, 2),
    ];

    assert_eq!(tbs[0], expected);
}


#[test]
fn canned_score_matrix() {
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
        INPUT1, INPUT2, &score_func, &gap_penalty);

    assert_eq!(expected, score_matrix);
}

#[test]
fn canned_alignment() {
    let (max_score, alignments) = align(INPUT1, INPUT2, &score_func, &gap_penalty);
    assert_eq!(max_score, 13.0);
    assert_eq!(alignments.len(), 1);

    let expected = vec![
        AlignmentCell::Both {left: 1, right: 1},
        AlignmentCell::Both {left: 2, right: 2},
        AlignmentCell::Both {left: 3, right: 3},
        AlignmentCell::RightGap {left: 4},
        AlignmentCell::Both {left: 5, right: 4},
        AlignmentCell::Both {left: 6, right: 5}
    ];

    assert_eq!(alignments[0], expected);
}
