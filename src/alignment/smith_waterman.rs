extern crate ordered_float;

// TODO: Consider using the seal library for smith-waterman. Once we learn how to do it ourselves...

type ScoringFunction = Fn(char, char) -> f32;
type GapPenaltyFunction = Fn(u32) -> f32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Diag,
    Up,
    Left,
}

type Directions = Vec<Direction>;

type ScoreMatrix = ndarray::Array2<f32>;
type TracebackMatrix = ndarray::Array2<Directions>;
type Index = (ndarray::Ix, ndarray::Ix);
type Traceback = Vec<Index>;

#[derive(Debug, PartialEq)]
pub enum AlignmentCell {
    Both { left: usize, right: usize }, // no gap; indices for both strings
    RightGap { left: usize },           // gap on right; index is for left string
    LeftGap { right: usize },           // gap on left; index is for right string
}
type Alignment = Vec<AlignmentCell>;

// Calculate the tracebacks for `traceback_matrix` starting at index `idx`.
//
// Note that tracebacks are in reverse. The first element in the traceback is
// the "biggest" index in the traceback, and they work their way backward
// through the strings being aligned.
pub fn tracebacks(traceback_matrix: &TracebackMatrix, idx: Index) -> Vec<Traceback> {
    let directions = traceback_matrix.get(idx).expect("index is invalid");
    if directions.is_empty() {
        vec![vec![]]
    } else {
        let mut tbs: Vec<Traceback> = Vec::new();

        let (row, col) = idx;

        for dir in directions {
            let tail_idx = match dir {
                Direction::Up => (row - 1, col),
                Direction::Diag => (row - 1, col - 1),
                Direction::Left => (row, col - 1),
            };

            let mut tails = tracebacks(traceback_matrix, tail_idx);

            for mut tail in tails {
                let mut tb = vec![idx];
                tb.extend(tail);
                tbs.push(tb);
            }
        }

        tbs
    }
}

pub fn build_score_matrix(
    a: &str,
    b: &str,
    score_func: &ScoringFunction,
    gap_penalty: &GapPenaltyFunction,
) -> (ScoreMatrix, TracebackMatrix) {
    let mut score_matrix = ScoreMatrix::zeros((a.len() + 1, b.len() + 1));

    let mut traceback_matrix =
        TracebackMatrix::from_elem((a.len() + 1, b.len() + 1), Directions::new());

    for (row, a_char) in a.chars().enumerate() {
        for (col, b_char) in b.chars().enumerate() {
            let row = row + 1;
            let col = col + 1;
            let match_score = score_func(a_char, b_char);

            let scores = [
                (
                    Direction::Diag,
                    score_matrix[(row - 1, col - 1)] + match_score,
                ),
                (Direction::Up, score_matrix[(row - 1, col)] - gap_penalty(1)),
                (
                    Direction::Left,
                    score_matrix[(row, col - 1)] - gap_penalty(1),
                ),
            ];

            let max_score = scores
                .iter()
                .max_by_key(|n| ordered_float::OrderedFloat(n.1))
                .unwrap()
                .1;

            let directions: Vec<Direction> = scores
                .iter()
                .filter(|n| n.1 == max_score)
                .map(|n| n.0)
                .collect();

            if max_score > 0.0 {
                score_matrix[(row, col)] = max_score;
                traceback_matrix[(row, col)].extend(directions);
            }
        }
    }

    (score_matrix, traceback_matrix)
}

// Convert a traceback (i.e. as returned by `tracebacks()`) into an alignment
// (i.e. as returned by `align`).
//
// Arguments:
//   tb: A traceback.
//   a: the sequence defining the rows in the traceback matrix.
//   b: the sequence defining the columns in the traceback matrix.
//
// Returns: An iterable of (index, index) tupless where ether (but not both)
//   tuples can be `None`.
fn traceback_to_alignment(traceback: &Traceback) -> Result<Alignment, String> {
    if traceback.is_empty() {
        return Result::Ok(Alignment::new());
    }

    // We subtract 1 from the indices here because we're translating from the
    // alignment matrix space (which has one extra row and column) to the space
    // of the input sequences.
    let mut traceback: Traceback = traceback.iter().map(|(i1, i2)| (i1 - 1, i2 - 1)).collect();
    traceback.reverse();

    let mut alignment = Alignment::new();

    // The first element in the traceback is always included.
    alignment.push(AlignmentCell::Both {
        left: traceback[0].0,
        right: traceback[0].1,
    });

    // Now compare adjacent traceback entries to see how they changed.
    for ((curr_a, curr_b), (next_a, next_b)) in traceback.iter().zip(traceback.iter().skip(1)) {
        if *next_a == curr_a + 1 {
            if *next_b == curr_b + 1 {
                alignment.push(AlignmentCell::Both {
                    left: *next_a,
                    right: *next_b,
                });
            } else {
                if next_b != curr_b {
                    return Result::Err(format!("Invalid traceback: {:?}", traceback));
                }

                alignment.push(AlignmentCell::RightGap { left: *next_a });
            }
        } else {
            if next_a != curr_a {
                return Result::Err(format!("Invalid traceback: {:?}", traceback));
            }

            alignment.push(AlignmentCell::LeftGap { right: *next_b });
        }
    }

    Result::Ok(alignment)
}

// Calculate the best alignments of sequences `a` and `b`.
//
// Arguments:
//     a: The first of two sequences to align
//     b: The second of two sequences to align
//     score_func: A 2-ary callable which calculates the "match" score between
//     two elements in the sequences.
//     gap_penalty: A 1-ary callable which calculates the gap penalty for a gap
//     of a given size.
//
// Returns: A (score, alignments) tuple. `score` is the score that all of the
//     `alignments` received. `alignments` is an iterable of `((index, index), .
//     . .)` tuples describing the best (i.e. maximal and equally good)
//     alignments. The first index in each pair is an index into `a` and the
//     second is into `b`. Either (but not both) indices in a pair may be `None`
//     indicating a gap in the corresponding sequence.
pub fn align(
    a: &str,
    b: &str,
    score_func: &ScoringFunction,
    gap_penalty: &GapPenaltyFunction,
) -> (f32, Vec<Alignment>) {
    let (score_matrix, tb_matrix) = build_score_matrix(a, b, score_func, gap_penalty);
    let max_score = score_matrix
        .iter()
        .max_by_key(|&n| ordered_float::OrderedFloat(*n))
        .expect("alignment is not possible with empty strings.");

    let max_indices: Vec<Index> = score_matrix
        .indexed_iter()
        .filter(|(_, score)| *score == max_score)
        .map(|(index, _)| index)
        .collect();

    let mut alignments = vec![];
    for index in max_indices {
        for traceback in tracebacks(&tb_matrix, index) {
            match traceback_to_alignment(&traceback) {
                Ok(alignment) => alignments.push(alignment),
                Err(msg) => panic!(msg),
            }
        }
    }
    (*max_score, alignments)
}

#[cfg(test)]
mod tests {
    extern crate ndarray;

    use super::*;
    use scoring::*;

    const INPUT1: &str = "GGTTGACTA";
    const INPUT2: &str = "TGTTACGG";

    #[test]
    fn canned_tracebacks() {
        let (score_matrix, traceback_matrix) =
            build_score_matrix(INPUT1, INPUT2, &score_func, &gap_penalty);

        let max_idx = score_matrix
            .indexed_iter()
            .max_by_key(|n| ordered_float::OrderedFloat(*n.1))
            .unwrap()
            .0;

        let tbs = tracebacks(&traceback_matrix, max_idx);
        assert_eq!(tbs.len(), 1);

        let expected = [(7, 6), (6, 5), (5, 4), (4, 4), (3, 3), (2, 2)];

        assert_eq!(tbs[0], expected);
    }

    #[test]
    fn canned_score_matrix() {
        let expected = ndarray::Array::from_shape_vec(
            (10, 9),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 1, 0, 0, 0, 3, 3, 0, 0, 3, 1, 0, 0, 0, 3, 6, 0,
                3, 1, 6, 4, 2, 0, 1, 4, 0, 3, 1, 4, 9, 7, 5, 3, 2, 0, 1, 6, 4, 7, 6, 4, 8, 6, 0, 0,
                4, 3, 5, 10, 8, 6, 5, 0, 0, 2, 1, 3, 8, 13, 11, 9, 0, 3, 1, 5, 4, 6, 11, 10, 8, 0,
                1, 0, 3, 2, 7, 9, 8, 7,
            ].iter()
            .map(|n| *n as f32)
            .collect(),
        ).unwrap();

        let (score_matrix, _) = build_score_matrix(INPUT1, INPUT2, &score_func, &gap_penalty);

        assert_eq!(expected, score_matrix);
    }

    #[test]
    fn canned_alignment() {
        let (max_score, alignments) = align(INPUT1, INPUT2, &score_func, &gap_penalty);
        assert_eq!(max_score, 13.0);
        assert_eq!(alignments.len(), 1);

        let expected = vec![
            AlignmentCell::Both { left: 1, right: 1 },
            AlignmentCell::Both { left: 2, right: 2 },
            AlignmentCell::Both { left: 3, right: 3 },
            AlignmentCell::RightGap { left: 4 },
            AlignmentCell::Both { left: 5, right: 4 },
            AlignmentCell::Both { left: 6, right: 5 },
        ];

        assert_eq!(alignments[0], expected);
    }

}
