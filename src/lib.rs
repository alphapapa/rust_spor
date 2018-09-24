extern crate ndarray;
extern crate ordered_float;

pub mod scoring;

// TODO: Consider using the seal library for smith-waterman. Once we learn how to do it ourselves...

type ScoringFunction = Fn(char, char) -> f32;
type GapPenaltyFunction = Fn(u32) -> f32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Diag,
    Up,
    Left
}

type Directions = Vec<Direction>;

type ScoreMatrix = ndarray::Array2<f32>;
type TracebackMatrix = ndarray::Array2<Directions>;
type Index = (ndarray::Ix, ndarray::Ix);
type Traceback = Vec<Index>;

#[derive(Debug, PartialEq)]
pub enum AlignmentCell {
    Both {left: usize, right: usize}, // no gap; indices for both strings
    RightGap {left: usize},           // gap on right; index is for left string
    LeftGap {right: usize},       // gap on left; index is for right string
}
type Alignment = Vec<AlignmentCell>;

// Calculate the tracebacks for `traceback_matrix` starting at index `idx`.
//
// Note that tracebacks are in reverse. The first element in the traceback is
// the "biggest" index in the traceback, and they work their way backward
// through the strings being aligned.
pub fn tracebacks(traceback_matrix: &TracebackMatrix,
                  idx: Index) -> Vec<Traceback>
{
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
    gap_penalty: &GapPenaltyFunction)
    -> (ScoreMatrix,
        TracebackMatrix)
{
    let mut score_matrix = ScoreMatrix::zeros(
        (a.len() + 1, b.len() + 1));

    let mut traceback_matrix = TracebackMatrix::from_elem(
        (a.len() + 1, b.len() + 1),
        Directions::new());

    for (row, a_char) in a.chars().enumerate() {
        for (col, b_char) in b.chars().enumerate() {
            let row = row + 1;
            let col = col + 1;
            let match_score = score_func(a_char, b_char);

            let scores = [
                (Direction::Diag,
                 score_matrix[(row - 1, col - 1)] + match_score),
                (Direction::Up,
                 score_matrix[(row - 1, col)] - gap_penalty(1)),
                (Direction::Left,
                 score_matrix[(row, col - 1)] - gap_penalty(1))
            ];

            let max_score = scores.iter()
                .max_by_key(|n| ordered_float::OrderedFloat(n.1))
                .unwrap().1;

            let directions: Vec<Direction> = scores.iter()
                .filter(|n| n.1 == max_score)
                .map(|n| n.0)
                .collect();

            if max_score > 0.0 {
                score_matrix[(row, col)] = max_score;
                traceback_matrix[(row, col)].extend(directions);
            }
        }
    }

    println!("{:?}", traceback_matrix);
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
fn traceback_to_alignment(
    traceback: &Traceback
) -> Result<Alignment, String>
{
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
    alignment.push(AlignmentCell::Both {left: traceback[0].0, right: traceback[0].1});

    println!("traceback: {:?}", traceback);
    // Now compare adjacent traceback entries to see how they changed.
    for ((curr_a, curr_b), (next_a, next_b)) in traceback.iter().zip(traceback.iter().skip(1)) {
        if *next_a == curr_a + 1 {
            if *next_b == curr_b + 1 {
                alignment.push(AlignmentCell::Both {left: *next_a, right: *next_b});
            }
            else {
                if next_b != curr_b {
                    return Result::Err(format!("Invalid traceback: {:?}", traceback));
                }

                alignment.push(AlignmentCell::RightGap {left: *next_a});
            }
        }
        else {
            if next_a != curr_a {
                return Result::Err(format!("Invalid traceback: {:?}", traceback));
            }

            alignment.push(AlignmentCell::LeftGap {right: *next_b});
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
    gap_penalty: &GapPenaltyFunction
) -> (f32, Vec<Alignment>)
{
    let (score_matrix, tb_matrix) = build_score_matrix(a, b, score_func, gap_penalty);
    let max_score = score_matrix.iter()
        .max_by_key(|&n| ordered_float::OrderedFloat(*n))
        .expect("alignment is not possible with empty strings.");

    let max_indices: Vec<Index> = score_matrix.indexed_iter()
        .filter(|(_, score)| *score == max_score)
        .map(|(index, _)| index)
        .collect();

    let mut alignments = vec![];
    for index in max_indices {
        for traceback in tracebacks(&tb_matrix, index) {
            match traceback_to_alignment(&traceback) {
                Ok(alignment) => alignments.push(alignment),
                Err(msg) => panic!(msg)
            }
        }
    }
    (*max_score, alignments)
}
