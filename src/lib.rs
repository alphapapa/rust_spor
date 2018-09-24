extern crate ndarray;
extern crate ordered_float;

pub mod scoring;

// TODO: Consider using the seal library for smith-waterman. Once we learn how to do it ourselves...

type ScoringFunction = Fn(char, char) -> f32;
type GapPenaltyFunction = Fn(u32) -> f32;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Diag,
    Up,
    Left
}

#[derive(Debug)]
pub enum Directions {
    None,
    Some(Vec<Direction>)
}

type ScoreMatrix = ndarray::Array2<f32>;
type TracebackMatrix = ndarray::Array2<Directions>;

fn _tracebacks<T>(score_matrix: &ScoreMatrix,
                  _traceback_matrix: &TracebackMatrix,
                  idx: T) -> [u32;0]
    where T: ndarray::NdIndex<ndarray::Ix2>
{
    let score = score_matrix[idx];
    if score == 0.0 {
        return []
    }

    // let directions = traceback_matrix[idx];
    // assert!(direction_value(&directions) != direction_value(Direction::NONE),
    //        "Tracebacks with direction NONE should have value 0!");
    []

//     score = score_matrix[idx]
//     if score == 0:
//         yield ()
//         return

//     directions = traceback_matrix[idx]

//     assert directions != Direction.NONE, 'Tracebacks with direction NONE should have value 0!'

//     row, col = idx

//     if directions & Direction.UP.value:
//         for tb in _tracebacks(score_matrix, traceback_matrix, (row - 1, col)):
//             yield itertools.chain(tb, ((idx, Direction.UP),))

//     if directions & Direction.LEFT.value:
//         for tb in _tracebacks(score_matrix, traceback_matrix, (row, col - 1)):
//             yield itertools.chain(tb, ((idx, Direction.LEFT),))

//     if directions & Direction.DIAG.value:
//         for tb in _tracebacks(score_matrix, traceback_matrix, (row - 1, col - 1)):
//             yield itertools.chain(tb, ((idx, Direction.DIAG),))
}

// def tracebacks(score_matrix, traceback_matrix, idx):
// """Calculate the tracebacks for `traceback_matrix` starting at index `idx`.

//     Returns: An iterable of tracebacks where each traceback is sequence of
//       (index, direction) tuples. Each `index` is an index into
//       `traceback_matrix`. `direction` indicates the direction into which the
//       traceback "entered" the index.
//     """
//     return filter(lambda tb: tb != (),
//                   _tracebacks(score_matrix,
//                               traceback_matrix,
//                               idx))

pub fn build_score_matrix(
    a: &str,
    b: &str,
    score_func: &ScoringFunction,
    gap_penalty: &GapPenaltyFunction)
    -> (ScoreMatrix,
        TracebackMatrix) {
        let mut score_matrix = ScoreMatrix::zeros(
            (a.len() + 1, b.len() + 1));

        let mut traceback_matrix = TracebackMatrix::from_shape_fn(
            (a.len() + 1, b.len() + 1),
            |_| Directions::None);

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

                if !directions.is_empty() {
                    traceback_matrix[(row, col)] = Directions::Some(directions);
                }
            }
        }
    }

    (score_matrix, traceback_matrix)
}
