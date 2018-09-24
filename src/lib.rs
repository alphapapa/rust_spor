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
type Index = (ndarray::Ix, ndarray::Ix);
type Traceback = Vec<((ndarray::Ix, ndarray::Ix), Direction)>;

// Calculate the tracebacks for `traceback_matrix` starting at index `idx`.
//
// Returns: An iterable of tracebacks where each traceback is sequence of
//   (index, direction) tuples. Each `index` is an index into
//   `traceback_matrix`. `direction` indicates the direction into which the
//   traceback "entered" the index.
pub fn tracebacks(score_matrix: &ScoreMatrix,
                 traceback_matrix: &TracebackMatrix,
                 idx: Index) -> Vec<Traceback>
{
    let mut tbs: Vec<Traceback> = Vec::new();


    if let Directions::Some(ref dirs) = traceback_matrix.get(idx).expect("index is invalid") {
        let (row, col) = idx;

        for dir in dirs {
            let tail_idx = match dir {
                Direction::Up => (row - 1, col),
                Direction::Diag => (row - 1, col - 1),
                Direction::Left => (row, col - 1),
            };

            let tails = tracebacks(score_matrix, traceback_matrix, tail_idx);

            for tail in tails {
                let mut tb = vec![(idx, *dir)];
                tb.extend(tail);
                tbs.push(tb);
            }
        }
    }

    tbs
}

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
