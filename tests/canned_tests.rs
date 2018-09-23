extern crate ndarray;
extern crate spor;

use spor::*;
use spor::scoring::*;

const INPUT1: &str = "GGTTGACTA";
const INPUT2: &str = "TGTTACGG";

#[test]
fn canned_tracebacks() {
    // let (score_matrix, _) = build_score_matrix(
    //     INPUT1, INPUT2, &score_func, &gap_penalty);

    // let mut max_idx = (0, 0);
    // let mut max_score = std::f32::MIN;

    // // TODO: Is there a way to lazily enumerate the array? We rolls our own for now...
    // for row in 0..score_matrix.dim().0 {
    //     for col in 0..score_matrix.dim().1 {
    //         let score = score_matrix[(row, col)];
    //         if score > max_score {
    //             max_score = score;
    //             max_idx = (row, col);
    //         }
    //     }
    // }

//     tbs = list(tracebacks(score_matrix, traceback_matrix, max_idx))
//     assert len(tbs) == 1

//     expected = (
//         ((2, 2), Direction.DIAG),
//         ((3, 3), Direction.DIAG),
//         ((4, 4), Direction.DIAG),
//         ((5, 4), Direction.UP),
//         ((6, 5), Direction.DIAG),
//         ((7, 6), Direction.DIAG))

//     assert tuple(tbs[0]) == expected
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

// def test_canned_alignment():
// _, alignments = align(ROWS, COLS, score, gap_penalty)
//     alignments = list(alignments)
//     assert len(alignments) == 1
//     actual = tuple(alignments[0])
//     expected = ((1, 1), (2, 2), (3, 3), (4, None), (5, 4), (6, 5))
//     assert actual == expected
